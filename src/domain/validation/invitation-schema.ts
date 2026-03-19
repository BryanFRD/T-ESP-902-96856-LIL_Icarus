import z from 'zod'

export const getInvitationSchema = (t: (key: string) => string) => {
  return z.object({
    maxUses: z
      .number()
      .min(1, t('team:validation.maxUsesMin'))
      .max(100, t('team:validation.maxUsesMax')),
    expiresAt: z.string().optional()
  })
}

export type InvitationSchemaType = z.infer<
  ReturnType<typeof getInvitationSchema>
>
