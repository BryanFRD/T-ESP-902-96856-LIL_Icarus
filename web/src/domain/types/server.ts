import type { ServerStatusEnum } from '@/domain/enum/server-status.ts'

export type Server = {
  status: ServerStatusEnum
  message: string
}
