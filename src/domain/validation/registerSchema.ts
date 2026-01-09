import z from 'zod'

export const getRegisterSchema = (t: (key: string) => string) => {
  const passwordSchema = z.string().min(8, t('password.min'))

  return z
    .object({
      username: z.string().min(1, t('inputField.usernameRequired')),
      email: z.string().email(t('email.invalid')),
      firstName: z.string().min(1, t('inputField.firstNameRequired')),
      lastName: z.string().min(1, t('inputField.lastNameRequired')),
      password: passwordSchema,
      confirmPassword: passwordSchema
    })
    .superRefine((data, ctx) => {
      if (data.confirmPassword !== data.password) {
        ctx.addIssue({
          code: z.ZodIssueCode.custom,
          path: ['confirmPassword'],
          message: t('password.mismatch')
        })
      }
    })
}

export type RegisterSchemaType = z.infer<ReturnType<typeof getRegisterSchema>>
