import { useCallback, useEffect, useRef } from 'react'
import type { Socket } from 'socket.io-client'

interface AgentOutputPayload {
  agent_id: string
  stream_id: number
  data: string
}

interface AgentStreamClosedPayload {
  agent_id: string
  stream_id: number
}

export function useAgentShell(
  socket: Socket,
  auditId: string,
  agentId: string,
  onOutput: (data: string) => void
) {
  const onOutputRef = useRef(onOutput)
  onOutputRef.current = onOutput

  useEffect(() => {
    const handleOutput = (payload: AgentOutputPayload) => {
      if (payload.agent_id !== agentId) return
      const decoded = atob(payload.data)
      onOutputRef.current(decoded)
    }

    const handleStreamClosed = (payload: AgentStreamClosedPayload) => {
      if (payload.agent_id !== agentId) return
      onOutputRef.current('\r\n')
    }

    socket.on('agent:output', handleOutput)
    socket.on('agent:stream_closed', handleStreamClosed)

    return () => {
      socket.off('agent:output', handleOutput)
      socket.off('agent:stream_closed', handleStreamClosed)
    }
  }, [socket, agentId])

  const sendCommand = useCallback(
    (command: string) => {
      socket.emit('agent:command', {
        audit_id: auditId,
        agent_id: agentId,
        command
      })
    },
    [socket, auditId, agentId]
  )

  return { sendCommand }
}
