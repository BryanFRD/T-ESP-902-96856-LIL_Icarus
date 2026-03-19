import { zodResolver } from '@hookform/resolvers/zod'
import { type Locale, format } from 'date-fns'
import { enUS, fr } from 'date-fns/locale'
import { CalendarIcon } from 'lucide-react'
import { useEffect } from 'react'
import { Controller, useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { z } from 'zod'

import { Button } from '@/components/ui/button'
import { Calendar } from '@/components/ui/calendar'
import {
  Dialog,
  DialogContent,
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
import {
  Popover,
  PopoverContent,
  PopoverTrigger
} from '@/components/ui/popover'
import { useCreateInvitation } from '@/hooks/team/use-create-invitation.ts'
import { cn } from '@/lib/utils'

const invitationSchema = z.object({
  maxUses: z.coerce.number().int().min(1).max(1000),
  expiresAt: z.date().nullable()
})

type InvitationSchemaType = z.infer<typeof invitationSchema>

const LOCALES: Record<string, Locale> = { fr, en: enUS }

interface Props {
  open: boolean
  onOpenChange: (open: boolean) => void
  teamId: string
}

export function CreateInvitationDialog({ open, onOpenChange, teamId }: Props) {
  const { t, i18n } = useTranslation('team')
  const { mutateAsync: createInvitation, isPending } = useCreateInvitation()

  const locale = LOCALES[i18n.language] ?? enUS

  const form = useForm<InvitationSchemaType>({
    resolver: zodResolver(invitationSchema),
    defaultValues: { maxUses: 1, expiresAt: null }
  })

  useEffect(() => {
    if (open) form.reset({ maxUses: 1, expiresAt: null })
  }, [open, form])

  const onSubmit = async (data: InvitationSchemaType) => {
    const payload = {
      maxUses: data.maxUses,
      expiresAt: data.expiresAt ? data.expiresAt.toISOString() : null
    }

    try {
      await createInvitation({ teamId, payload })
      toast.success(t('show.invitations.createDialog.toast.success'))
      onOpenChange(false)
    } catch {
      toast.error(t('show.invitations.createDialog.toast.error'))
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t('show.invitations.createDialog.title')}</DialogTitle>
        </DialogHeader>

        <form onSubmit={form.handleSubmit(onSubmit)}>
          <FieldGroup>
            <Controller
              control={form.control}
              name='maxUses'
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor={field.name}>
                    {t('show.invitations.createDialog.fields.maxUses.label')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id={field.name}
                    type='number'
                    min={1}
                    aria-invalid={fieldState.invalid}
                    placeholder={t(
                      'show.invitations.createDialog.fields.maxUses.placeholder'
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
              name='expiresAt'
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel>
                    {t('show.invitations.createDialog.fields.expiresAt.label')}
                  </FieldLabel>
                  <Popover>
                    <PopoverTrigger asChild>
                      <Button
                        type='button'
                        variant='outline'
                        className={cn(
                          'w-full justify-start text-left font-normal',
                          !field.value && 'text-muted-foreground'
                        )}
                      >
                        <CalendarIcon className='size-4' />
                        {field.value
                          ? format(field.value, 'PPP', { locale })
                          : t(
                              'show.invitations.createDialog.fields.expiresAt.placeholder'
                            )}
                      </Button>
                    </PopoverTrigger>

                    <PopoverContent
                      align='start'
                      side='top'
                      className='w-auto p-0'
                    >
                      <Calendar
                        mode='single'
                        selected={field.value || undefined}
                        onSelect={field.onChange}
                        disabled={date => date < new Date()}
                        locale={locale}
                      />
                    </PopoverContent>
                  </Popover>
                  <FieldDescription>
                    {t('show.invitations.createDialog.fields.expiresAt.hint')}
                  </FieldDescription>
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
              {t('show.invitations.createDialog.actions.cancel')}
            </Button>
            <Button type='submit' disabled={isPending}>
              {t('show.invitations.createDialog.actions.create')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
