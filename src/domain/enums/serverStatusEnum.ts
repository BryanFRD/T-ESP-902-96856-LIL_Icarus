export const ServerStatusEnum = {
  Up: 'Up',
  Maintenance: 'Maintenance',
  DOWN: 'Down'
} as const

export type ServerStatusEnum =
  (typeof ServerStatusEnum)[keyof typeof ServerStatusEnum]
