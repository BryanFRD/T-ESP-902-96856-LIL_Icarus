import { useEffect, useRef, useState } from 'react'

import { getSocketClient } from '@/configuration/api/socket-client.ts'
import type { AuditMessage } from '@/domain/types/audit.ts'
import { getAuditMessages, sendAuditMessage } from '@/service/audit-service.ts'

export function useAuditMessages(auditId: string) {
  const [messages, setMessages] = useState<AuditMessage[]>([])
  const [isLoading, setIsLoading] = useState(true)
  const [isSending, setIsSending] = useState(false)
  const joinedRef = useRef(false)

  useEffect(() => {
    let cancelled = false

    setIsLoading(true)
    getAuditMessages(auditId)
      .then(res => {
        if (!cancelled) {
          setMessages(res.data)
        }
      })
      .finally(() => {
        if (!cancelled) setIsLoading(false)
      })

    const socket = getSocketClient()

    const joinRoom = () => {
      if (joinedRef.current) return
      socket.emit('room:join', { type: 'audit', id: auditId })
      joinedRef.current = true
    }

    if (socket.connected) {
      joinRoom()
    } else {
      socket.once('connect', joinRoom)
    }

    const handleMessage = (message: AuditMessage) => {
      if (cancelled) return
      setMessages(prev => {
        const exists = prev.some(m => m.id === message.id)
        if (exists) return prev
        return [...prev, message]
      })
    }

    socket.on('audit:message', handleMessage)

    return () => {
      cancelled = true
      socket.off('audit:message', handleMessage)
      socket.off('connect', joinRoom)
      if (joinedRef.current) {
        socket.emit('room:leave', { type: 'audit', id: auditId })
        joinedRef.current = false
      }
    }
  }, [auditId])

  async function send(content: string) {
    const trimmed = content.trim()
    if (!trimmed || isSending) return

    setIsSending(true)
    try {
      await sendAuditMessage(auditId, trimmed)
    } finally {
      setIsSending(false)
    }
  }

  return { messages, isLoading, isSending, send }
}
