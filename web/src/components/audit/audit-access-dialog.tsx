import { zodResolver } from '@hookform/resolvers/zod'
import { LockIcon } from 'lucide-react'
import { Controller, useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { useNavigate } from 'react-router-dom'
import { toast } from 'sonner'
import { z } from 'zod'

import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle
} from '@/components/ui/dialog'
import {
  Field,
  FieldError,
  FieldGroup,
  FieldLabel
} from '@/components/ui/field'
import { useVerifyAuditAccess } from '@/hooks/audit/use-verify-audit-access.ts'
import ROUTES, { getRoutePath } from '@/router/routes-name.ts'

import { InputGroup, InputGroupAddon, InputGroupInput } from '../ui/input-group'

const formSchema = z.object({
  password: z.string().min(1, 'Password is required')
})

type FormSchema = z.infer<typeof formSchema>

interface Props {
  open: boolean
}

export function AuditAccessDialog({ open }: Props) {
  const { t } = useTranslation('audit')

  const navigate = useNavigate()
  const { mutateAsync: verifyAccess } = useVerifyAuditAccess()

  const form = useForm<FormSchema>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      password: ''
    }
  })

  const handleClose = () => {
    navigate(getRoutePath(ROUTES.DASHBOARD))
  }

  const onSubmit = (data: FormSchema) => {
    toast.promise(verifyAccess(data.password), {
      loading: t('access.form.toast.verifying'),
      success: t('access.form.toast.success'),
      error: t('access.form.toast.error')
    })
  }

  return (
    <Dialog open={open} onOpenChange={handleClose}>
      <DialogContent showCloseButton={false}>
        <DialogHeader>
          <DialogTitle>{t('access.title')}</DialogTitle>
          <DialogDescription>{t('access.subtitle')}</DialogDescription>
        </DialogHeader>

        <form onSubmit={form.handleSubmit(onSubmit)}>
          <FieldGroup>
            <Controller
              control={form.control}
              name='password'
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor={field.name}>
                    {t('access.form.fields.password.label')}
                  </FieldLabel>

                  <InputGroup>
                    <InputGroupInput
                      {...field}
                      id={field.name}
                      type='password'
                      aria-invalid={fieldState.invalid}
                      placeholder={t('access.form.fields.password.placeholder')}
                    />
                    <InputGroupAddon>
                      <LockIcon className='size-4' />
                    </InputGroupAddon>
                  </InputGroup>

                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />
          </FieldGroup>

          <DialogFooter className='mt-4'>
            <Button type='button' variant='outline' onClick={handleClose}>
              {t('access.form.actions.cancel')}
            </Button>
            <Button type='submit' disabled={form.formState.isSubmitting}>
              {t('access.form.actions.submit')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
