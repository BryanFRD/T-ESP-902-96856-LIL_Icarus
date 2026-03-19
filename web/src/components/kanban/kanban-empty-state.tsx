import { Clock, Columns3 } from 'lucide-react'
import { useTranslation } from 'react-i18next'

const KanbanEmptyState = () => {
  const { t } = useTranslation()

  return (
    <div className='flex h-full w-full items-center justify-center p-8'>
      <div className='max-w-md text-center'>
        <div className='mb-8 flex justify-center'>
          <div className='relative'>
            <div className='flex h-24 w-24 items-center justify-center rounded-2xl bg-gradient-to-br from-gray-100 to-gray-200 dark:from-gray-800 dark:to-gray-700'>
              <Columns3 className='h-12 w-12 text-gray-400 dark:text-gray-500' />
            </div>
            <div className='absolute -right-2 -bottom-2 flex h-10 w-10 items-center justify-center rounded-full border-4 border-white bg-orange-100 dark:border-gray-900 dark:bg-orange-900/30'>
              <Clock className='h-5 w-5 text-orange-600 dark:text-orange-400' />
            </div>
          </div>
        </div>

        <h2 className='mb-3 text-2xl font-bold text-gray-900 dark:text-gray-100'>
          {t('kanban:emptyState.title')}
        </h2>
        <p className='leading-relaxed text-gray-600 dark:text-gray-400'>
          {t('kanban:emptyState.description')}
        </p>

        <div className='mt-8 rounded-xl border-2 border-dashed border-gray-200 bg-gray-50 p-4 dark:border-gray-700 dark:bg-gray-800/50'>
          <p className='text-sm text-gray-500 dark:text-gray-400'>
            {t('kanban:emptyState.info')}
          </p>
        </div>
      </div>
    </div>
  )
}

export default KanbanEmptyState
