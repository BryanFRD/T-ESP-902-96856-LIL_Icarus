import { zodResolver } from '@hookform/resolvers/zod'
import { useState } from 'react'
import { Controller, useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import type { z } from 'zod'

import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger
} from '@/components/ui/dialog'
import { Field, FieldError, FieldLabel } from '@/components/ui/field'
import { Input } from '@/components/ui/input'
import { getPasswordSchema } from '@/domain/validation/reset-password-schema'
import { useSendPasswordResetRequest } from '@/hooks/auth/use-send-password-reset-request'

type ForgotPasswordFormValues = z.infer<ReturnType<typeof getPasswordSchema>>

export function ForgotPasswordDialog({
  children
}: Readonly<{ children: React.ReactNode }>) {
  const { t } = useTranslation('auth')
  const [open, setOpen] = useState(false)
  const { mutateAsync, isPending } = useSendPasswordResetRequest()

  const form = useForm<ForgotPasswordFormValues>({
    resolver: zodResolver(getPasswordSchema(t as (key: string) => string)),
    defaultValues: { resetEmail: '' }
  })

  const onSubmit = async (data: ForgotPasswordFormValues) => {
    try {
      await mutateAsync({ resetEmail: data.resetEmail })
      toast.success(t('forgotPassword.dialog.toast.success'))
      form.reset()
      setOpen(false)
    } catch {
      toast.error(t('forgotPassword.dialog.toast.error'))
    }
  }

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>{children}</DialogTrigger>

      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t('forgotPassword.dialog.title')}</DialogTitle>
          <DialogDescription>
            {t('forgotPassword.dialog.description')}
          </DialogDescription>
        </DialogHeader>

        <form
          id='form-forgot-password'
          onSubmit={form.handleSubmit(onSubmit)}
          className='flex flex-col gap-4'
        >
          <Controller
            control={form.control}
            name='resetEmail'
            render={({ field, fieldState }) => (
              <Field data-invalid={fieldState.invalid}>
                <FieldLabel htmlFor={field.name}>
                  {t('forgotPassword.dialog.fields.email.label')}
                </FieldLabel>
                <Input
                  {...field}
                  id={field.name}
                  type='email'
                  aria-invalid={fieldState.invalid}
                  placeholder={t(
                    'forgotPassword.dialog.fields.email.placeholder'
                  )}
                />
                {fieldState.invalid && (
                  <FieldError errors={[fieldState.error]} />
                )}
              </Field>
            )}
          />
        </form>

        <DialogFooter>
          <Button
            type='submit'
            form='form-forgot-password'
            disabled={isPending}
          >
            {t('forgotPassword.dialog.submit')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
