import z from 'zod'

export const getCompanySchema = (t: (key: string) => string) => {
  return z.object({
    name: z
      .string()
      .min(1, t('team.validation.companyNameRequired'))
      .max(100, t('team.validation.companyNameMaxLength')),
    siret: z
      .string()
      .min(1, t('team.validation.siretRequired'))
      .regex(/^\d{14}$/, t('team.validation.siretInvalid')),
    description: z
      .string()
      .max(500, t('team.validation.descriptionMaxLength'))
      .optional()
  })
}

export type CompanySchemaType = z.infer<ReturnType<typeof getCompanySchema>>
