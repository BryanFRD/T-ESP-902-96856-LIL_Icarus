import { useTranslation } from 'react-i18next'

import SidebarToggleIcon from '@/components/icon/SidebarToggleIcon'
import Button from '@/components/ui/button/Button.tsx'
import type { SidebarToggleProps } from '@/types/global.ts'

const SidebarToggle = ({
  isOpen,
  onToggle,
  className = ''
}: SidebarToggleProps) => {
  const { t } = useTranslation()

  return (
    <Button
      onClick={onToggle}
      className={`justify-start ${className}`}
      aria-label={isOpen ? t('ui:sidebar.closeMenu') : t('ui:sidebar.openMenu')}
      variant='ghost'
      size='lg'
      fullWidth
    >
      <SidebarToggleIcon isOpen={isOpen} size={20} />
      {isOpen && (
        <span className='text-sm whitespace-nowrap'>
          {t('ui:sidebar.closeMenu')}
        </span>
      )}
    </Button>
  )
}

export default SidebarToggle
