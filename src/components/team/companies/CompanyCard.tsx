import { motion } from 'framer-motion'
import { Building2, Hash, MoreVertical, Pencil, Trash2 } from 'lucide-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import Button from '@/components/ui/button/Button'
import type { Company } from '@/domain/types/Team.ts'
import useInteractionOutside from '@/hooks/common/useInteractionOutside.ts'

interface CompanyCardProps {
  company: Company
  index?: number
  canManage?: boolean
  onEdit?: (company: Company) => void
  onDelete?: (company: Company) => void
}

const CompanyCard = ({
  company,
  index = 0,
  canManage = false,
  onEdit,
  onDelete
}: CompanyCardProps) => {
  const { t } = useTranslation()
  const [showMenu, setShowMenu] = useState(false)

  const { ref } = useInteractionOutside<HTMLDivElement>(
    () => setShowMenu(false),
    { enabled: showMenu, eventType: 'click' }
  )

  return (
    <motion.div
      initial={{ opacity: 0, y: 10 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ delay: index * 0.05 }}
      className='rounded-lg border border-gray-200 bg-white p-4 dark:border-gray-700 dark:bg-gray-800'
    >
      <div className='flex items-start justify-between'>
        <div className='flex items-start gap-4'>
          <div className='flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-lg bg-blue-100 dark:bg-blue-900'>
            <Building2 className='h-5 w-5 text-blue-600 dark:text-blue-400' />
          </div>
          <div className='min-w-0 flex-1'>
            <h4 className='font-medium text-gray-900 dark:text-white'>
              {company.name}
            </h4>
            {company.description && (
              <p className='mt-1 text-sm text-gray-500 dark:text-gray-400'>
                {company.description}
              </p>
            )}
            {company.siret && (
              <p className='mt-2 inline-flex items-center gap-1 text-sm text-gray-500 dark:text-gray-400'>
                <Hash className='h-3 w-3' />
                {t('team:siret')}: {company.siret}
              </p>
            )}
          </div>
        </div>

        {canManage && (
          <div className='relative' ref={ref}>
            <Button
              variant='ghost'
              size='sm'
              onClick={() => setShowMenu(!showMenu)}
              aria-label={t('common:actions.more')}
            >
              <MoreVertical className='h-4 w-4' />
            </Button>

            {showMenu && (
              <div className='absolute right-0 z-10 mt-2 w-48 rounded-lg border border-gray-200 bg-white py-1 shadow-lg dark:border-gray-700 dark:bg-gray-800'>
                <button
                  type='button'
                  onClick={() => {
                    setShowMenu(false)
                    onEdit?.(company)
                  }}
                  className='flex w-full items-center gap-2 px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100 dark:text-gray-200 dark:hover:bg-gray-700'
                >
                  <Pencil className='h-4 w-4' />
                  {t('common:actions.edit')}
                </button>
                <button
                  type='button'
                  onClick={() => {
                    setShowMenu(false)
                    onDelete?.(company)
                  }}
                  className='flex w-full items-center gap-2 px-4 py-2 text-left text-sm text-red-600 hover:bg-gray-100 dark:text-red-400 dark:hover:bg-gray-700'
                >
                  <Trash2 className='h-4 w-4' />
                  {t('common:actions.delete')}
                </button>
              </div>
            )}
          </div>
        )}
      </div>
    </motion.div>
  )
}

export default CompanyCard
