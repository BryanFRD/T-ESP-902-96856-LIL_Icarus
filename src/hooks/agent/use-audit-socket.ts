import { useEffect, useRef } from 'react'
import type { Socket } from 'socket.io-client'

import { disconnectSocket, getSocket } from '@/lib/socket.ts'

export function useAuditSocket(auditId: string): Socket {
  const socketRef = useRef<Socket>(getSocket(auditId))

  useEffect(() => {
    socketRef.current = getSocket(auditId)

    return () => {
      disconnectSocket(auditId)
    }
  }, [auditId])

  return socketRef.current
}
