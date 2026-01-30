import type { ServerStatusEnum } from '@/domain/enum/ServerStatus.ts'

export type Server = {
  status: ServerStatusEnum
  message: string
}
