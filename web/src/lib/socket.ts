import { type Socket, io } from 'socket.io-client'

const sockets = new Map<string, Socket>()

export function getSocket(auditId: string): Socket {
  const existing = sockets.get(auditId)
  if (existing) return existing

  const socket = io(`${__CONFIG__.ICARUS_API_URL}/v1/ws`, {
    withCredentials: true,
    transports: ['websocket', 'polling']
  })

  const joinRoom = () => {
    socket.emit('room:join', { room: `audit:${auditId}` })
  }

  socket.on('connect', joinRoom)

  // If already connected (reconnection), join immediately
  if (socket.connected) {
    joinRoom()
  }

  sockets.set(auditId, socket)
  return socket
}

export function disconnectSocket(auditId: string): void {
  const socket = sockets.get(auditId)
  if (!socket) return

  socket.emit('room:leave', { room: `audit:${auditId}` })
  socket.disconnect()
  sockets.delete(auditId)
}
