import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect } from 'react'
import { Controller, useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
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
  FieldDescription,
  FieldError,
  FieldGroup,
  FieldLabel
} from '@/components/ui/field'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea.tsx'
import type { Company } from '@/domain/types/team.ts'
import { useCreateCompany } from '@/hooks/team/use-create-company.ts'
import { usePatchCompany } from '@/hooks/team/use-patch-company.ts'

const companySchema = z.object({
  name: z.string().min(1).max(100),
  siret: z
    .string()
    .min(1)
    .regex(/^\d{14}$/),
  description: z.string().max(500).optional()
})

type CompanySchemaType = z.infer<typeof companySchema>

interface Props {
  open: boolean
  onOpenChange: (open: boolean) => void
  teamId: string
  company?: Company
}

export function CompanyFormDialog({
  open,
  onOpenChange,
  teamId,
  company
}: Props) {
  const { t } = useTranslation('team')

  const isEditing = !!company

  const { mutateAsync: createCompany, isPending: isCreating } =
    useCreateCompany()
  const { mutateAsync: patchCompany, isPending: isPatching } = usePatchCompany()
  const isPending = isCreating || isPatching

  const form = useForm<CompanySchemaType>({
    resolver: zodResolver(companySchema),
    defaultValues: {
      name: company?.name ?? '',
      siret: company?.siret ?? '',
      description: company?.description ?? ''
    }
  })

  useEffect(() => {
    if (open) {
      form.reset({
        name: company?.name ?? '',
        siret: company?.siret ?? '',
        description: company?.description ?? ''
      })
    }
  }, [open, company, form])

  const onSubmit = async (data: CompanySchemaType) => {
    const payload = {
      name: data.name,
      siret: data.siret,
      description: data.description || undefined
    }

    try {
      if (isEditing) {
        await patchCompany({ teamId, companyId: company.id, payload })
        toast.success(t('show.companies.formDialog.toast.updateSuccess'))
      } else {
        await createCompany({ teamId, payload })
        toast.success(t('show.companies.formDialog.toast.createSuccess'))
      }
      onOpenChange(false)
    } catch {
      toast.error(t('show.companies.formDialog.toast.error'))
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>
            {isEditing
              ? t('show.companies.formDialog.editTitle')
              : t('show.companies.formDialog.createTitle')}
          </DialogTitle>

          {isEditing && (
            <DialogDescription>
              {t('show.companies.formDialog.editDescription', {
                name: company.name
              })}
            </DialogDescription>
          )}
        </DialogHeader>

        <form onSubmit={form.handleSubmit(onSubmit)}>
          <FieldGroup>
            <Controller
              control={form.control}
              name='name'
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor={field.name}>
                    {t('show.companies.formDialog.fields.name.label')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id={field.name}
                    aria-invalid={fieldState.invalid}
                    placeholder={t(
                      'show.companies.formDialog.fields.name.placeholder'
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
              name='siret'
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor={field.name}>
                    {t('show.companies.formDialog.fields.siret.label')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id={field.name}
                    aria-invalid={fieldState.invalid}
                    placeholder={t(
                      'show.companies.formDialog.fields.siret.placeholder'
                    )}
                  />
                  <FieldDescription>
                    {t('show.companies.formDialog.fields.siret.hint')}
                  </FieldDescription>
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />

            <Controller
              control={form.control}
              name='description'
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor={field.name}>
                    {t('show.companies.formDialog.fields.description.label')}
                  </FieldLabel>
                  <Textarea
                    {...field}
                    id={field.name}
                    aria-invalid={fieldState.invalid}
                    placeholder={t(
                      'show.companies.formDialog.fields.description.placeholder'
                    )}
                  />
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />
          </FieldGroup>

          <DialogFooter className='mt-4'>
            <Button
              type='button'
              variant='outline'
              onClick={() => onOpenChange(false)}
            >
              {t('show.companies.formDialog.actions.cancel')}
            </Button>
            <Button type='submit' disabled={isPending}>
              {isEditing
                ? t('show.companies.formDialog.actions.save')
                : t('show.companies.formDialog.actions.create')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
