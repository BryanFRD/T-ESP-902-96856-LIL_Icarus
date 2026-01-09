import z from 'zod'

export const uploadAvatarFileSchema = z.object({
  avatar: z.instanceof(File)
})
