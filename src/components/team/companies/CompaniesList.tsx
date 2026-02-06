import { Building2 } from 'lucide-react'
import { useTranslation } from 'react-i18next'

import type { Company } from '@/domain/types/Team.ts'

import CompanyCard from './CompanyCard.tsx'

interface CompaniesListProps {
  companies: Company[]
  canManage?: boolean
  onEdit?: (company: Company) => void
  onDelete?: (company: Company) => void
}

const CompaniesList = ({
  companies,
  canManage = false,
  onEdit,
  onDelete
}: CompaniesListProps) => {
  const { t } = useTranslation()

  if (companies.length === 0) {
    return (
      <div className='flex flex-col items-center justify-center py-12 text-center'>
        <div className='mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-800'>
          <Building2 className='h-8 w-8 text-gray-400 dark:text-gray-500' />
        </div>
        <h3 className='mb-2 text-lg font-medium text-gray-900 dark:text-white'>
          {t('team:noCompanies')}
        </h3>
        <p className='text-sm text-gray-500 dark:text-gray-400'>
          {t('team:noCompaniesDescription')}
        </p>
      </div>
    )
  }

  return (
    <div className='space-y-3'>
      {companies.map((company, index) => (
        <CompanyCard
          key={company.id}
          company={company}
          index={index}
          canManage={canManage}
          onEdit={onEdit}
          onDelete={onDelete}
        />
      ))}
    </div>
  )
}

export default CompaniesList
