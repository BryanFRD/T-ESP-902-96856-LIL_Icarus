import { useTranslation } from 'react-i18next'

import SidebarToggleIcon from '@/components/icon/SidebarToggleIcon'
import Button from '@/components/ui/Button'
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
      aria-label={isOpen ? t('sidebar.closeMenu') : t('sidebar.openMenu')}
      variant='ghost'
      size='lg'
      fullWidth
    >
      <SidebarToggleIcon isOpen={isOpen} size={20} />
      {isOpen && (
        <span className='text-sm whitespace-nowrap'>
          {t('sidebar.closeMenu')}
        </span>
      )}
    </Button>
  )
}

export default SidebarToggle
