import z from 'zod'

export const getTeamSchema = (t: (key: string) => string) => {
  return z.object({
    name: z
      .string()
      .min(1, t('team.validation.nameRequired'))
      .max(100, t('team.validation.nameMaxLength')),
    description: z
      .string()
      .max(500, t('team.validation.descriptionMaxLength'))
      .optional()
  })
}

export type TeamSchemaType = z.infer<ReturnType<typeof getTeamSchema>>
