export const ServerStatus = {
  Up: 'Up',
  Maintenance: 'Maintenance',
  DOWN: 'Down'
} as const

export type ServerStatusEnum = (typeof ServerStatus)[keyof typeof ServerStatus]
