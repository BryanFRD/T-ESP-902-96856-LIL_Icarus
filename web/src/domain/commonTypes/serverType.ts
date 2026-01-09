import type { ServerStatusEnum } from '@/domain/enums/serverStatusEnum.ts'

export type ServerStatus = {
  status: ServerStatusEnum
  message: string
}
