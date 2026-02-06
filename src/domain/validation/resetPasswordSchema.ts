import z from 'zod'

export const getPasswordSchema = (t: (key: string) => string) =>
  z.object({
    resetEmail: z
      .string()
      .email(t('auth:validation.emailInvalid'))
      .min(1, t('auth:validation.emailRequired'))
  })

export const getNewPasswordSchema = (t: (key: string) => string) => {
  return z.object({
    newPassword: z.string().min(8, t('auth:validation.passwordMin'))
  })
}
