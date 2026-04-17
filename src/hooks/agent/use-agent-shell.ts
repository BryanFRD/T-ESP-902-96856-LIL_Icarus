import { useCallback, useEffect, useRef, useState } from 'react'
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

interface AgentCwdPayload {
  agent_id: string
  path: string
}

export function useAgentShell(
  socket: Socket,
  auditId: string,
  agentId: string,
  onOutput: (data: string) => void
) {
  const onOutputRef = useRef(onOutput)
  onOutputRef.current = onOutput

  const [cwd, setCwd] = useState<string | null>(null)
  const replayDoneRef = useRef(false)
  const liveBufferRef = useRef<string[]>([])

  useEffect(() => {
    const handleOutput = (payload: AgentOutputPayload) => {
      if (payload.agent_id !== agentId) return
      const decoded = atob(payload.data)
      if (!replayDoneRef.current) {
        liveBufferRef.current.push(decoded)
        return
      }
      onOutputRef.current(decoded)
    }

    const handleStreamClosed = (payload: AgentStreamClosedPayload) => {
      if (payload.agent_id !== agentId) return
      onOutputRef.current('\r\n')
    }

    const handleCwd = (payload: AgentCwdPayload) => {
      if (payload.agent_id !== agentId) return
      setCwd(payload.path)
    }

    socket.on('agent:output', handleOutput)
    socket.on('agent:stream_closed', handleStreamClosed)
    socket.on('agent:cwd', handleCwd)

    return () => {
      socket.off('agent:output', handleOutput)
      socket.off('agent:stream_closed', handleStreamClosed)
      socket.off('agent:cwd', handleCwd)
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

  const markReplayComplete = useCallback(() => {
    if (replayDoneRef.current) return
    replayDoneRef.current = true
    for (const chunk of liveBufferRef.current) {
      onOutputRef.current(chunk)
    }
    liveBufferRef.current = []
  }, [])

  return { sendCommand, cwd, markReplayComplete }
}
