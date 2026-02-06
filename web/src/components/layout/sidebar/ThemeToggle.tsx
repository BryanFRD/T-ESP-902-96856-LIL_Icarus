import { useTranslation } from 'react-i18next'

import DarkModeIcon from '@/components/icon/DarkModeIcon'
import { useTheme } from '@/context/ThemeContext'

const ThemeToggle = () => {
  const { t } = useTranslation()
  const { theme, toggleTheme } = useTheme()

  return (
    <button
      onClick={toggleTheme}
      className='relative flex w-full items-center justify-between gap-2 px-4 py-3 transition-colors duration-300 hover:cursor-pointer hover:bg-gray-200/50 dark:hover:bg-gray-800/50'
    >
      <span>{t('common:theme')}</span>
      <DarkModeIcon className='h-6' isDark={theme === 'dark'} />
    </button>
  )
}

export default ThemeToggle
