import { zodResolver } from '@hookform/resolvers/zod'
import { Building2, Users } from 'lucide-react'
import { useMemo } from 'react'
import { Controller, useForm } from 'react-hook-form'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'
import { useNavigate } from 'react-router-dom'

import DatePicker from '@/components/ui/DatePicker'
import InputField from '@/components/ui/InputField'
import Select, { type SelectOption } from '@/components/ui/Select'
import Button from '@/components/ui/button/Button'
import type { Team } from '@/domain/types/Team.ts'
import {
  type CreateAuditSchemaType,
  createAuditSchema
} from '@/domain/validation/auditSchema'
import { useCreateAudit } from '@/hooks/audit/useCreateAudit.ts'
import { useMyTeams } from '@/hooks/team/useMyTeams.ts'
import { ROUTES, getRoutePath } from '@/router/routesName.ts'

const CreateAuditForm = () => {
  const { data: teams, isLoading: isTeamLoading } = useMyTeams()
  const { mutateAsync: createAudit, isPending } = useCreateAudit()
  const { t } = useTranslation()
  const navigate = useNavigate()

  const {
    register,
    handleSubmit,
    control,
    watch,
    setValue,
    formState: { errors, isSubmitting }
  } = useForm<CreateAuditSchemaType>({
    resolver: zodResolver(createAuditSchema(t)),
    defaultValues: {
      name: '',
      description: '',
      endDate: '',
      companyId: '',
      teamId: ''
    }
  })

  const selectedTeamId = watch('teamId')

  const teamsOpt = useMemo(() => {
    if (!teams || !Array.isArray(teams)) return []
    return teams.map((team: Team) => ({
      label: team.name,
      value: team.id
    }))
  }, [teams])

  const companiesOpt = useMemo(() => {
    if (!teams || !Array.isArray(teams)) return []

    if (selectedTeamId) {
      const selectedTeam = teams.find(t => t.id === selectedTeamId)

      return (
        selectedTeam?.companies?.map(company => ({
          label: company.name,
          value: company.id
        })) ?? []
      )
    }

    const allCompanies = teams.flatMap(team => team.companies ?? [])

    const allOptions = allCompanies.map(company => ({
      label: company.name,
      value: company.id
    }))

    const uniqueOptionsMap = new Map(
      allOptions.map(option => [option.value, option])
    )

    return Array.from(uniqueOptionsMap.values())
  }, [teams, selectedTeamId])

  async function onSubmit(data: CreateAuditSchemaType) {
    toast
      .promise(createAudit(data), {
        loading: t('audit.creating'),
        success: t('audit.created'),
        error: t('error.auditCreationFailed')
      })
      .finally(() => navigate(getRoutePath(ROUTES.AUDITS)))
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} className='space-y-6'>
      <div className='grid grid-cols-1 gap-6 md:grid-cols-2'>
        <div className='md:col-span-2'>
          <InputField
            label={t('audit.create.auditLabel')}
            type='text'
            placeholder={t('audit.create.auditPlaceholder')}
            icon={Users}
            register={register}
            name='name'
            error={errors.name?.message}
          />
        </div>

        <div className='md:col-span-2'>
          <InputField
            label={t('audit.create.descriptionLabel')}
            type='text'
            icon={Users}
            placeholder={t('audit.create.descriptionPlaceholder')}
            register={register}
            name='description'
            error={errors.description?.message}
          />
        </div>

        <Controller
          name='teamId'
          control={control}
          render={({ field }) => (
            <Select
              label={t('audit.create.teamLabel')}
              options={teamsOpt}
              value={teamsOpt.find(opt => opt.value === field.value) ?? null}
              onChange={val => {
                const selectedOption = val as SelectOption | null
                field.onChange(selectedOption?.value ?? '')
                setValue('companyId', '')
              }}
              placeholder={t('audit.create.teamPlaceholder')}
              isLoading={isTeamLoading}
              isSearchable={true}
              icon={Users}
              error={errors.teamId?.message}
            />
          )}
        />

        <Controller
          name='companyId'
          control={control}
          render={({ field }) => (
            <Select
              label={t('audit.create.companyLabel')}
              options={companiesOpt}
              value={
                companiesOpt.find(opt => opt.value === field.value) ?? null
              }
              onChange={val => {
                const selectedOption = val as SelectOption | null
                field.onChange(selectedOption?.value ?? '')
              }}
              placeholder={t('audit.create.companyPlaceholder')}
              disabled={!selectedTeamId}
              isSearchable={true}
              icon={Building2}
              error={errors.companyId?.message}
            />
          )}
        />

        <div className='md:col-span-1'>
          <Controller
            name='endDate'
            control={control}
            render={({ field }) => (
              <DatePicker
                includeTime={true}
                label={t('audit.create.endDateLabel')}
                value={field.value ? new Date(field.value) : null}
                onChange={date => {
                  field.onChange(date ? date.toISOString() : '')
                }}
                placeholder={t('audit.create.endDatePlaceholder')}
                minDate={new Date()}
                error={errors.endDate?.message}
              />
            )}
          />
        </div>
      </div>

      <div className='flex justify-end pt-4'>
        <Button
          type='submit'
          className='w-full md:w-auto'
          disabled={isSubmitting || isPending}
        >
          {t('audit.create.submit')}
        </Button>
      </div>
    </form>
  )
}

export default CreateAuditForm
