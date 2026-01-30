import z from 'zod'

export const createAuditSchema = (t: (key: string) => string) => {
  return z.object({
    name: z.string().min(3, t('audit.name.min')).max(100, t('audit.name.max')),
    description: z.string().max(500, t('audit.description.max')),
    endDate: z.string().refine(date => !Number.isNaN(Date.parse(date)), {
      message: t('audit.date.invalid')
    }),
    companyId: z.string().ulid(t('audit.company.invalid')),
    teamId: z.string().ulid(t('audit.team.invalid'))
  })
}

export const verifyAuditAccessSchema = (t: (key: string) => string) => {
  return z.object({
    password: z.string().min(1, t('audit.password.required'))
  })
}

export type VerifyAuditAccessSchemaType = z.infer<
  ReturnType<typeof verifyAuditAccessSchema>
>
export type CreateAuditSchemaType = z.infer<
  ReturnType<typeof createAuditSchema>
>
