import z from 'zod'

export const getLoginSchema = (t: (key: string) => string) => {
  const passwordSchema = z.string().min(8, t('auth:validation.passwordMin'))

  return z.object({
    email: z.string().email(t('auth:validation.emailInvalid')),
    password: passwordSchema
  })
}

export type LoginSchemaType = z.infer<ReturnType<typeof getLoginSchema>>
