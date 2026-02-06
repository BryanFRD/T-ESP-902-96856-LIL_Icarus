import { X } from 'lucide-react'
import { useTranslation } from 'react-i18next'

import Logo from '@/components/icon/Logo.tsx'

const LogoSidebar = ({
  onClose,
  isDesktopCollapsed
}: {
  onClose: () => void
  isDesktopCollapsed: boolean
}) => {
  const { t } = useTranslation(['app', 'ui'])

  return (
    <div className='flex h-16 flex-shrink-0 items-center justify-start border-b border-gray-300/50 dark:border-gray-800/50'>
      <div
        className={`flex w-full items-center transition-all duration-300 ${
          isDesktopCollapsed ? 'ml-4.5 pr-0 pl-0' : 'ml-0 pr-4.5 pl-4.5'
        }`}
      >
        <div className='flex-shrink-0 p-0.5'>
          <Logo fill={'#ff6900'} className={'h-10 w-10'} />
        </div>
        <div
          className={`overflow-hidden transition-all duration-300 ${isDesktopCollapsed ? 'w-0 opacity-0' : 'ml-3 w-auto opacity-100'}`}
        >
          <h1 className='text-xl font-bold whitespace-nowrap text-gray-900 transition-colors duration-300 dark:text-white'>
            {t('app:name')}
          </h1>
          <p className='text-xs whitespace-nowrap text-gray-400'>
            {t('app:description')}
          </p>
        </div>
      </div>
      <button
        onClick={onClose}
        className='flex-shrink-0 rounded-lg p-2 text-gray-400 transition-colors hover:bg-gray-800/50 hover:text-white md:hidden'
        aria-label={t('ui:sidebar.closeSidebar')}
      >
        <X size={20} />
      </button>
    </div>
  )
}

export default LogoSidebar
