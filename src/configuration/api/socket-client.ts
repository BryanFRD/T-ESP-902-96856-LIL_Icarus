import { type Socket, io } from 'socket.io-client'

let socket: Socket | null = null

export function getSocketClient(): Socket {
  if (!socket) {
    socket = io(__CONFIG__.ICARUS_API_URL, {
      path: '/v1/ws',
      withCredentials: true,
      autoConnect: true,
      reconnection: true,
      reconnectionAttempts: 5,
      reconnectionDelay: 1000
    })
  }
  return socket
}

export function disconnectSocket(): void {
  if (socket) {
    socket.disconnect()
    socket = null
  }
}
