import { zodResolver } from '@hookform/resolvers/zod'
import { type Locale, format } from 'date-fns'
import { enUS, fr } from 'date-fns/locale'
import { Building2Icon, CalendarIcon, UsersIcon } from 'lucide-react'
import { Controller, useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { useNavigate } from 'react-router-dom'
import { toast } from 'sonner'
import { z } from 'zod'

import { Button } from '@/components/ui/button'
import { Calendar } from '@/components/ui/calendar'
import {
  Combobox,
  ComboboxContent,
  ComboboxEmpty,
  ComboboxInput,
  ComboboxItem,
  ComboboxList
} from '@/components/ui/combobox'
import {
  Field,
  FieldError,
  FieldGroup,
  FieldLabel
} from '@/components/ui/field'
import { Input } from '@/components/ui/input'
import { InputGroupAddon } from '@/components/ui/input-group'
import {
  Item,
  ItemContent,
  ItemDescription,
  ItemTitle
} from '@/components/ui/item'
import {
  Popover,
  PopoverContent,
  PopoverTrigger
} from '@/components/ui/popover'
import { Textarea } from '@/components/ui/textarea.tsx'
import type { Company } from '@/domain/types/company.ts'
import type { TeamWithDetails } from '@/domain/types/team.ts'
import { useCreateAudit } from '@/hooks/audit/use-create-audit.ts'
import { useMyTeam } from '@/hooks/team/use-my-team.ts'
import { cn } from '@/lib/utils'
import { ROUTES, getRoutePath } from '@/router/routes-name.ts'

const DATE_LOCALES: Record<string, Locale> = { fr, en: enUS }

const formSchema = z.object({
  name: z.string().min(1, 'Name is required'),
  description: z.string().min(1, 'Description is required'),
  endDate: z.string().refine(val => {
    const date = new Date(val)
    return !isNaN(date.getTime()) && date > new Date()
  }, 'End date must be a valid future date'),
  companyId: z.string().ulid(),
  teamId: z.string().ulid()
})

type FormSchema = z.infer<typeof formSchema>

export function AuditCreateForm() {
  const { t, i18n } = useTranslation('audit')

  const navigate = useNavigate()

  const { data: teams, isLoading: isTeamsLoading } = useMyTeam()
  const { mutateAsync: createAudit, isPending } = useCreateAudit()

  const locale = DATE_LOCALES[i18n.language] ?? enUS

  const form = useForm<FormSchema>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: '',
      description: '',
      endDate: '',
      companyId: '',
      teamId: ''
    }
  })

  const selectedTeamId = form.watch('teamId')

  const teamsOptions = teams || []

  const selectedTeam = teamsOptions.find(team => team.id === selectedTeamId)

  const companiesOptions = selectedTeam?.companies || []

  const onSubmit = (data: FormSchema) => {
    const handleCreate = async () => {
      await createAudit(data)
      navigate(getRoutePath(ROUTES.AUDITS))
    }

    toast.promise(handleCreate(), {
      loading: t('create.form.toast.creating'),
      success: t('create.form.toast.success'),
      error: t('create.form.toast.error')
    })
  }

  return (
    <form onSubmit={form.handleSubmit(onSubmit)} className='space-y-6'>
      <FieldGroup>
        <Controller
          control={form.control}
          name='name'
          render={({ field, fieldState }) => (
            <Field data-invalid={fieldState.invalid}>
              <FieldLabel htmlFor={field.name}>
                {t('create.form.fields.name.label')}
              </FieldLabel>
              <Input
                {...field}
                id={field.name}
                aria-invalid={fieldState.invalid}
                placeholder={t('create.form.fields.name.placeholder')}
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
                {t('create.form.fields.description.label')}
              </FieldLabel>
              <Textarea
                {...field}
                id={field.name}
                aria-invalid={fieldState.invalid}
                placeholder={t('create.form.fields.description.placeholder')}
                className='max-h-32 resize-y'
              />
              {fieldState.invalid && <FieldError errors={[fieldState.error]} />}
            </Field>
          )}
        />

        <div className='grid grid-cols-1 gap-6 md:grid-cols-2'>
          <Controller
            control={form.control}
            name='teamId'
            render={({ field, fieldState }) => (
              <Field data-invalid={fieldState.invalid}>
                <FieldLabel htmlFor={field.name}>
                  {t('create.form.fields.team.label')}
                </FieldLabel>

                <Combobox
                  items={teamsOptions}
                  name={field.name}
                  value={field.value}
                  onValueChange={field.onChange}
                >
                  <ComboboxInput
                    placeholder={t('create.form.fields.team.placeholder')}
                  >
                    <InputGroupAddon>
                      <UsersIcon className='size-4' />
                    </InputGroupAddon>
                  </ComboboxInput>

                  <ComboboxContent align='end'>
                    <ComboboxEmpty>Empty</ComboboxEmpty>

                    <ComboboxList>
                      {(item: TeamWithDetails) => (
                        <ComboboxItem key={item.id} value={item.id}>
                          <Item size='xs' className='p-0'>
                            <ItemContent>
                              <ItemTitle>{item.name}</ItemTitle>
                              <ItemDescription>
                                {item.companies?.length ?? 0} companies
                              </ItemDescription>
                            </ItemContent>
                          </Item>
                        </ComboboxItem>
                      )}
                    </ComboboxList>
                  </ComboboxContent>
                </Combobox>

                {fieldState.invalid && (
                  <FieldError errors={[fieldState.error]} />
                )}
              </Field>
            )}
          />

          <Controller
            control={form.control}
            name='companyId'
            render={({ field, fieldState }) => (
              <Field data-invalid={fieldState.invalid}>
                <FieldLabel htmlFor={field.name}>
                  {t('create.form.fields.company.label')}
                </FieldLabel>

                <Combobox
                  items={companiesOptions}
                  name={field.name}
                  value={field.value}
                  onValueChange={field.onChange}
                >
                  <ComboboxInput
                    placeholder={t('create.form.fields.company.placeholder')}
                    disabled={!selectedTeamId || isTeamsLoading}
                  >
                    <InputGroupAddon>
                      <Building2Icon className='size-4' />
                    </InputGroupAddon>
                  </ComboboxInput>

                  <ComboboxContent align='end'>
                    <ComboboxEmpty>Empty</ComboboxEmpty>

                    <ComboboxList>
                      {(item: Company) => (
                        <ComboboxItem key={item.id} value={item.id}>
                          <Item size='xs' className='p-0'>
                            <ItemContent>
                              <ItemTitle>{item.name}</ItemTitle>
                            </ItemContent>
                          </Item>
                        </ComboboxItem>
                      )}
                    </ComboboxList>
                  </ComboboxContent>
                </Combobox>

                {fieldState.invalid && (
                  <FieldError errors={[fieldState.error]} />
                )}
              </Field>
            )}
          />
        </div>

        <Controller
          control={form.control}
          name='endDate'
          render={({ field, fieldState }) => {
            const dateValue = field.value ? new Date(field.value) : undefined
            return (
              <Field data-invalid={fieldState.invalid}>
                <FieldLabel>{t('create.form.fields.endDate.label')}</FieldLabel>
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
                      {dateValue
                        ? format(dateValue, 'PPP', { locale })
                        : t('create.form.fields.endDate.placeholder')}
                    </Button>
                  </PopoverTrigger>
                  <PopoverContent
                    align='start'
                    side='top'
                    className='w-auto p-0'
                  >
                    <Calendar
                      mode='single'
                      selected={dateValue}
                      onSelect={date =>
                        field.onChange(date ? date.toISOString() : '')
                      }
                      disabled={date => date < new Date()}
                      locale={locale}
                    />
                  </PopoverContent>
                </Popover>
                {fieldState.invalid && (
                  <FieldError errors={[fieldState.error]} />
                )}
              </Field>
            )
          }}
        />
      </FieldGroup>

      <div className='flex justify-end'>
        <Button type='submit' disabled={isPending}>
          {t('create.form.actions.submit')}
        </Button>
      </div>
    </form>
  )
}
