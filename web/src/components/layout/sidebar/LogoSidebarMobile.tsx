import { t } from 'i18next'
import { Menu, X } from 'lucide-react'

import Logo from '@/assets/img/icarus-white.png'

const LogoSidebarMobile = ({
  onToggle,
  isOpen
}: {
  onToggle: () => void
  isOpen: boolean
}) => {
  return (
    <div className='flex h-full items-center px-4'>
      <button
        onClick={onToggle}
        className='rounded-lg border bg-gray-800/50 p-2 shadow-lg backdrop-blur-xl transition-all duration-300 ease-in-out hover:shadow-xl active:scale-95 dark:border-gray-700/50'
        aria-label={isOpen ? 'Fermer le menu' : 'Ouvrir le menu'}
      >
        {isOpen ? (
          <X
            size={20}
            className='text-gray-300 transition-colors hover:text-white'
          />
        ) : (
          <Menu
            size={20}
            className='text-gray-300 transition-colors hover:text-white'
          />
        )}
      </button>
      <div className='ml-4 flex items-center'>
        <div className='flex-shrink-0 rounded-xl bg-gradient-to-r from-red-500 to-orange-500 p-0.5'>
          <img src={Logo} alt='Icarus Logo' className='h-8 w-8' />
        </div>
        <div className='ml-3'>
          <h1 className='text-lg font-bold text-gray-900 dark:text-white'>
            {t('app.name')}
          </h1>
        </div>
      </div>
    </div>
  )
}

export default LogoSidebarMobile
