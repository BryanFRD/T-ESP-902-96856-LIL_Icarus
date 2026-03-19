import { zodResolver } from '@hookform/resolvers/zod'
import { Controller, useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { z } from 'zod'

import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle
} from '@/components/ui/card'
import {
  Field,
  FieldError,
  FieldGroup,
  FieldLabel
} from '@/components/ui/field'
import { Input } from '@/components/ui/input'
import { useChangePassword } from '@/hooks/user/use-change-password.ts'

const formSchema = z
  .object({
    password: z.string().min(6),
    newPassword: z.string().min(6),
    confirmPassword: z.string().min(6)
  })
  .refine(data => data.newPassword === data.confirmPassword, {
    message: 'Passwords do not match',
    path: ['confirmPassword']
  })

type FormSchema = z.infer<typeof formSchema>

export function PasswordForm(props: React.ComponentProps<typeof Card>) {
  const { t } = useTranslation('auth')

  const { mutateAsync: changePassword, isPending } = useChangePassword()

  const form = useForm<FormSchema>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      password: '',
      newPassword: '',
      confirmPassword: ''
    }
  })

  const onSubmit = async (data: FormSchema) => {
    try {
      await changePassword(data)
      toast.success(t('profile.passwordForm.toast.success'))
      form.reset()
    } catch {
      toast.error(t('profile.passwordForm.toast.error'))
    }
  }

  return (
    <form id='form-change-password' onSubmit={form.handleSubmit(onSubmit)}>
      <Card {...props}>
        <CardHeader>
          <CardTitle>{t('profile.passwordForm.title')}</CardTitle>
          <CardDescription>
            {t('profile.passwordForm.description')}
          </CardDescription>
        </CardHeader>

        <CardContent>
          <FieldGroup className='gap-5'>
            <Controller
              control={form.control}
              name='password'
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor={field.name}>
                    {t('profile.passwordForm.fields.currentPassword.label')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id={field.name}
                    type='password'
                    aria-invalid={fieldState.invalid}
                    placeholder={t(
                      'profile.passwordForm.fields.currentPassword.placeholder'
                    )}
                  />
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />

            <Controller
              control={form.control}
              name='newPassword'
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor={field.name}>
                    {t('profile.passwordForm.fields.newPassword.label')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id={field.name}
                    type='password'
                    aria-invalid={fieldState.invalid}
                    placeholder={t(
                      'profile.passwordForm.fields.newPassword.placeholder'
                    )}
                  />
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />

            <Controller
              control={form.control}
              name='confirmPassword'
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor={field.name}>
                    {t('profile.passwordForm.fields.confirmPassword.label')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id={field.name}
                    type='password'
                    aria-invalid={fieldState.invalid}
                    placeholder={t(
                      'profile.passwordForm.fields.confirmPassword.placeholder'
                    )}
                  />
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />
          </FieldGroup>
        </CardContent>

        <CardFooter className='justify-end'>
          <Button type='submit' disabled={isPending}>
            {t('profile.passwordForm.submit')}
          </Button>
        </CardFooter>
      </Card>
    </form>
  )
}
