import { ArrowLeft } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { useNavigate } from 'react-router-dom'
import { twMerge } from 'tailwind-merge'

interface BackButtonProps {
  label?: string
  to?: string
  className?: string
  onClick?: () => void
}

const BackButton = ({ label, to, className, onClick }: BackButtonProps) => {
  const navigate = useNavigate()
  const { t } = useTranslation()

  const handleNavigation = () => {
    if (onClick) {
      onClick()
      return
    }

    if (to) {
      navigate(to)
    } else {
      navigate(-1)
    }
  }

  return (
    <button
      onClick={handleNavigation}
      type='button'
      className={twMerge(
        'group flex cursor-pointer items-center gap-3 rounded-lg text-sm font-medium transition-colors outline-none focus-visible:ring-2 focus-visible:ring-orange-500',
        label
          ? 'text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-100'
          : '',
        className
      )}
      aria-label={label || t('common.back')}
    >
      <div
        className={twMerge(
          'flex h-8 w-8 items-center justify-center rounded-full border border-gray-300 transition-all duration-300',
          'dark:border-gray-700 dark:bg-gray-800 dark:text-gray-200',
          'group-hover:border-orange-200 group-hover:bg-orange-50 group-hover:text-orange-600',
          'dark:group-hover:border-orange-900/50 dark:group-hover:bg-orange-900/20 dark:group-hover:text-orange-400'
        )}
      >
        <ArrowLeft size={16} className='transition-transform duration-300' />
      </div>

      {label && <span>{label}</span>}
    </button>
  )
}

export default BackButton
