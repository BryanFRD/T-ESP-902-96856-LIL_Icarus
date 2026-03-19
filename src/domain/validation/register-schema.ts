import z from 'zod'

export const getRegisterSchema = (t: (key: string) => string) => {
  const passwordSchema = z.string().min(8, t('auth:validation.passwordMin'))

  return z
    .object({
      username: z.string().min(1, t('auth:validation.usernameRequired')),
      email: z.string().email(t('auth:validation.emailInvalid')),
      firstName: z.string().min(1, t('auth:validation.firstNameRequired')),
      lastName: z.string().min(1, t('auth:validation.lastNameRequired')),
      password: passwordSchema,
      confirmPassword: passwordSchema
    })
    .superRefine((data, ctx) => {
      if (data.confirmPassword !== data.password) {
        ctx.addIssue({
          code: z.ZodIssueCode.custom,
          path: ['confirmPassword'],
          message: t('auth:validation.passwordMismatch')
        })
      }
    })
}

export type RegisterSchemaType = z.infer<ReturnType<typeof getRegisterSchema>>
