import { useEffect, useRef } from 'react'
import type { Socket } from 'socket.io-client'

import { getSocket } from '@/lib/socket.ts'

export function useAuditSocket(auditId: string): Socket {
  const socketRef = useRef<Socket | null>(null)

  if (!socketRef.current || !socketRef.current.connected) {
    socketRef.current = getSocket(auditId)
  }

  useEffect(() => {
    socketRef.current = getSocket(auditId)
  }, [auditId])

  return socketRef.current
}
