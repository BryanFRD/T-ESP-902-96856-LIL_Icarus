import z from 'zod'

export const getLoginSchema = (t: (key: string) => string) => {
  const passwordSchema = z.string().min(8, t('password.min'))

  return z.object({
    email: z.string().email(t('email.invalid')),
    password: passwordSchema
  })
}

export type LoginSchemaType = z.infer<ReturnType<typeof getLoginSchema>>
