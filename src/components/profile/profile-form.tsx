import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect } from 'react'
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
import { useUserContext } from '@/context/user-context.tsx'
import type { User } from '@/domain/types/user.ts'
import { usePatchMe } from '@/hooks/auth/use-patch-me.ts'
import { getModifiedFields } from '@/utils/modified-fields.ts'

const formSchema = z.object({
  firstName: z.string().min(1),
  lastName: z.string().min(1),
  username: z.string().min(3),
  email: z.string().email()
})

type FormSchema = z.infer<typeof formSchema>

export function ProfileForm(props: React.ComponentProps<typeof Card>) {
  const { t } = useTranslation('auth')

  const { user } = useUserContext()
  const { mutateAsync: patchMe, isPending } = usePatchMe()

  const form = useForm<FormSchema>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      firstName: user?.firstName ?? '',
      lastName: user?.lastName ?? '',
      username: user?.username ?? '',
      email: user?.email ?? ''
    }
  })

  useEffect(() => {
    if (user) {
      form.reset({
        firstName: user.firstName ?? '',
        lastName: user.lastName ?? '',
        username: user.username ?? '',
        email: user.email ?? ''
      })
    }
  }, [user, form])

  const onSubmit = async (data: FormSchema) => {
    const modifiedPayload = getModifiedFields<User>(
      data,
      form.formState.dirtyFields
    )

    if (Object.keys(modifiedPayload).length > 0) {
      try {
        await patchMe(modifiedPayload)
        toast.success(t('profile.accountForm.toast.success'))
      } catch {
        toast.error(t('profile.accountForm.toast.error'))
      }
    }
  }

  return (
    <form id='form-profile' onSubmit={form.handleSubmit(onSubmit)}>
      <Card {...props}>
        <CardHeader>
          <CardTitle>{t('profile.accountForm.title')}</CardTitle>
          <CardDescription>
            {t('profile.accountForm.description')}
          </CardDescription>
        </CardHeader>

        <CardContent>
          <FieldGroup className='gap-5'>
            <div className='grid grid-cols-1 gap-5 md:grid-cols-2'>
              <Controller
                control={form.control}
                name='firstName'
                render={({ field, fieldState }) => (
                  <Field data-invalid={fieldState.invalid}>
                    <FieldLabel htmlFor={field.name}>
                      {t('profile.accountForm.fields.firstName.label')}
                    </FieldLabel>
                    <Input
                      {...field}
                      id={field.name}
                      aria-invalid={fieldState.invalid}
                      placeholder={t(
                        'profile.accountForm.fields.firstName.placeholder'
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
                name='lastName'
                render={({ field, fieldState }) => (
                  <Field data-invalid={fieldState.invalid}>
                    <FieldLabel htmlFor={field.name}>
                      {t('profile.accountForm.fields.lastName.label')}
                    </FieldLabel>
                    <Input
                      {...field}
                      id={field.name}
                      aria-invalid={fieldState.invalid}
                      placeholder={t(
                        'profile.accountForm.fields.lastName.placeholder'
                      )}
                    />
                    {fieldState.invalid && (
                      <FieldError errors={[fieldState.error]} />
                    )}
                  </Field>
                )}
              />
            </div>

            <Controller
              control={form.control}
              name='username'
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor={field.name}>
                    {t('profile.accountForm.fields.username.label')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id={field.name}
                    aria-invalid={fieldState.invalid}
                    placeholder={t(
                      'profile.accountForm.fields.username.placeholder'
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
              name='email'
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor={field.name}>
                    {t('profile.accountForm.fields.email.label')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id={field.name}
                    type='email'
                    aria-invalid={fieldState.invalid}
                    placeholder={t(
                      'profile.accountForm.fields.email.placeholder'
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
            {t('profile.accountForm.submit')}
          </Button>
        </CardFooter>
      </Card>
    </form>
  )
}
