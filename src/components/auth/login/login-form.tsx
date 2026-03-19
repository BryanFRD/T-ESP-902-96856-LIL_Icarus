import { zodResolver } from '@hookform/resolvers/zod'
import { Controller, useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { z } from 'zod'

import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter } from '@/components/ui/card'
import {
  Field,
  FieldError,
  FieldGroup,
  FieldLabel
} from '@/components/ui/field'
import { Input } from '@/components/ui/input'
import { login } from '@/service/auth-service.ts'

const formSchema = z.object({
  email: z.string().email(),
  password: z.string().min(6)
})

type FormSchema = z.infer<typeof formSchema>

export function LoginForm(props: React.ComponentProps<typeof Card>) {
  const { t } = useTranslation('auth')

  const form = useForm<FormSchema>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      email: '',
      password: ''
    }
  })

  const onSubmit = async (data: FormSchema) => {
    try {
      await login(data)
    } catch {
      toast.error(t('login.form.toast.error'))
    } finally {
      toast.success(t('login.form.toast.success'))
    }
  }

  return (
    <form id='form-auth-login' onSubmit={form.handleSubmit(onSubmit)}>
      <Card {...props}>
        <CardContent>
          <FieldGroup className='gap-5'>
            <Controller
              control={form.control}
              name='email'
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor={field.name}>
                    {t('login.form.fields.email.label')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id={field.name}
                    aria-invalid={fieldState.invalid}
                    placeholder={t('login.form.fields.email.placeholder')}
                  />
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />

            <Controller
              control={form.control}
              name='password'
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor={field.name}>
                    {t('login.form.fields.password.label')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id={field.name}
                    type='password'
                    aria-invalid={fieldState.invalid}
                    placeholder={t('login.form.fields.password.placeholder')}
                  />
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />

            <Button size='sm' variant='link' className='w-min self-end'>
              Forgot password?
            </Button>
          </FieldGroup>
        </CardContent>

        <CardFooter>
          <Field>
            <Button type='submit' className='w-full'>
              {t('login.form.submit')}
            </Button>
          </Field>
        </CardFooter>
      </Card>
    </form>
  )
}
