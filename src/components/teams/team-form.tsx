import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect } from 'react'
import { Controller, useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { z } from 'zod'

import { Button } from '@/components/ui/button'
import {
  Field,
  FieldError,
  FieldGroup,
  FieldLabel
} from '@/components/ui/field'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea.tsx'
import type { Team } from '@/domain/types/team.ts'
import { useCreateTeam } from '@/hooks/team/use-create-team.ts'
import { usePatchTeam } from '@/hooks/team/use-patch-team.ts'

interface TeamFormProps {
  team?: Team
  onSuccess?: (team: Team) => void
}

const formSchema = z.object({
  name: z.string().min(1, 'Name is required'),
  description: z.string().optional()
})

type FormSchema = z.infer<typeof formSchema>

const TeamForm = ({ team, onSuccess }: TeamFormProps) => {
  const { t } = useTranslation('team')

  const { mutateAsync: createTeam, isPending: isCreating } = useCreateTeam()
  const { mutateAsync: patchTeam, isPending: isPatching } = usePatchTeam()

  const isEditing = !!team
  const isPending = isCreating || isPatching

  const form = useForm<FormSchema>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: '',
      description: ''
    }
  })

  useEffect(() => {
    if (team) {
      form.reset({ name: team.name, description: team.description || '' })
    }
  }, [team, form])

  const onSubmit = async (data: FormSchema) => {
    try {
      if (isEditing && team) {
        const updatedTeam = await patchTeam({ teamId: team.id, payload: data })
        toast.success(t('shared.teamForm.toast.updateSuccess'))
        onSuccess?.(updatedTeam)
      } else {
        const newTeam = await createTeam(data)
        toast.success(t('shared.teamForm.toast.createSuccess'))
        form.reset()
        onSuccess?.(newTeam)
      }
    } catch {
      toast.error(t('shared.teamForm.toast.error'))
    }
  }

  return (
    <form onSubmit={form.handleSubmit(onSubmit)} className='space-y-4'>
      <FieldGroup>
        <Controller
          control={form.control}
          name='name'
          render={({ field, fieldState }) => (
            <Field data-invalid={fieldState.invalid}>
              <FieldLabel htmlFor={field.name}>
                {t('shared.teamForm.form.fields.name.label')}
              </FieldLabel>
              <Input
                {...field}
                id={field.name}
                aria-invalid={fieldState.invalid}
                placeholder={t('shared.teamForm.form.fields.name.placeholder')}
              />
              {fieldState.invalid && <FieldError errors={[fieldState.error]} />}
            </Field>
          )}
        />

        <Controller
          control={form.control}
          name='description'
          render={({ field, fieldState }) => (
            <Field data-invalid={fieldState.invalid}>
              <FieldLabel htmlFor={field.name}>
                {t('shared.teamForm.form.fields.description.label')}
              </FieldLabel>
              <Textarea
                {...field}
                id={field.name}
                aria-invalid={fieldState.invalid}
                placeholder={t(
                  'shared.teamForm.form.fields.description.placeholder'
                )}
                className='max-h-32 resize-y'
              />
              {fieldState.invalid && <FieldError errors={[fieldState.error]} />}
            </Field>
          )}
        />
      </FieldGroup>

      <div className='flex justify-end gap-2'>
        <Button type='submit' disabled={isPending}>
          {isEditing
            ? t('shared.teamForm.actions.save')
            : t('shared.teamForm.actions.create')}
        </Button>
      </div>
    </form>
  )
}

export default TeamForm
