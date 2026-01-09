import z from 'zod'

export const getPasswordSchema = (t: (key: string) => string) =>
  z.object({
    resetEmail: z
      .string()
      .email(t('email.invalid'))
      .min(1, t('inputField.emailRequired'))
  })

export const getNewPasswordSchema = (t: (key: string) => string) => {
  return z.object({
    newPassword: z.string().min(8, t('password.min'))
  })
}
