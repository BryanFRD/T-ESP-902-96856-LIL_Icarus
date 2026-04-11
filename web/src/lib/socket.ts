import { type Socket, io } from 'socket.io-client'

const sockets = new Map<string, Socket>()

export function getSocket(auditId: string): Socket {
  const existing = sockets.get(auditId)
  if (existing) {
    if (!existing.connected) {
      existing.connect()
    }
    if (existing.connected) {
      existing.emit('room:join', { type: 'audit', id: auditId })
    }
    return existing
  }

  const socket = io(__CONFIG__.ICARUS_API_URL, {
    path: '/v1/ws',
    withCredentials: true,
    transports: ['websocket', 'polling'],
    reconnection: true,
    reconnectionAttempts: 10,
    reconnectionDelay: 1000
  })

  const joinRoom = () => {
    socket.emit('room:join', { type: 'audit', id: auditId })
  }

  // Join room on every connect/reconnect
  socket.on('connect', joinRoom)
  socket.io.on('reconnect', joinRoom)

  // If already connected, join immediately
  if (socket.connected) {
    joinRoom()
  }

  sockets.set(auditId, socket)
  return socket
}

export function disconnectSocket(auditId: string): void {
  const socket = sockets.get(auditId)
  if (!socket) return

  socket.emit('room:leave', { type: 'audit', id: auditId })
  socket.disconnect()
  sockets.delete(auditId)
}
