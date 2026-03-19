import type { LucideIcon } from 'lucide-react'
import type { ReactNode } from 'react'
import { twMerge } from 'tailwind-merge'

interface DetailCardProps<T extends string> {
  tabs: Array<{ id: T; label: string; icon: LucideIcon }>
  activeTab: T
  onTabChange: (tab: T) => void
  children: ReactNode
  actionButton?: ReactNode
}

function DetailCard<T extends string>({
  tabs,
  activeTab,
  onTabChange,
  children,
  actionButton
}: DetailCardProps<T>) {
  return (
    <div className='rounded-lg border border-gray-200 bg-white dark:border-gray-800 dark:bg-gray-900'>
      <div
        className={twMerge(
          'flex items-center px-6 pt-4',
          actionButton ? 'justify-between' : 'justify-start'
        )}
      >
        <div className='border-b border-gray-200 dark:border-gray-700'>
          <nav className='-mb-px flex space-x-8'>
            {tabs.map(tab => {
              const Icon = tab.icon
              const isActive = activeTab === tab.id

              return (
                <button
                  key={tab.id}
                  type='button'
                  onClick={() => onTabChange(tab.id)}
                  className={twMerge(
                    'flex items-center gap-2 border-b-2 px-1 py-4 text-sm font-medium transition-colors',
                    isActive
                      ? 'border-orange-600 text-orange-700 dark:border-orange-500 dark:text-orange-500'
                      : 'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 dark:text-gray-400 dark:hover:border-gray-600 dark:hover:text-gray-300'
                  )}
                >
                  <Icon className='size-4' />
                  {tab.label}
                </button>
              )
            })}
          </nav>
        </div>
        {actionButton && <div>{actionButton}</div>}
      </div>
      <div className='p-6'>{children}</div>
    </div>
  )
}

export default DetailCard
