import { Building2, Mail, Users } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { twMerge } from 'tailwind-merge'

export type TeamTab = 'members' | 'companies' | 'invitations'

interface TeamTabsProps {
  activeTab: TeamTab
  onTabChange: (tab: TeamTab) => void
}

interface Tab {
  id: TeamTab
  label: string
  icon: typeof Users
}

const TeamTabs = ({ activeTab, onTabChange }: TeamTabsProps) => {
  const { t } = useTranslation()

  const tabs: Tab[] = [
    { id: 'members', label: t('team:tabs.members'), icon: Users },
    { id: 'companies', label: t('team:tabs.companies'), icon: Building2 },
    { id: 'invitations', label: t('team:tabs.invitations'), icon: Mail }
  ]

  return (
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
  )
}

export default TeamTabs
