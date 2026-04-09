import { buildAvatarUrl } from '@/utils/string-utils'

export const useGetAvatar = (avatarPath: string | null | undefined) => {
  return { data: buildAvatarUrl(avatarPath) }
}
