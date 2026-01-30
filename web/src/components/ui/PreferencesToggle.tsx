import { AnimatePresence, motion } from 'framer-motion'
import { useState } from 'react'
import ReactCountryFlag from 'react-country-flag'

import { useTheme } from '@/context/ThemeContext.tsx'
import useLanguage from '@/hooks/lang/useLanguage.ts'

import DarkModeIcon from '../icon/DarkModeIcon.tsx'

const PreferencesToggle = () => {
  const { theme, toggleTheme } = useTheme()
  const { currentLanguage, changeLanguage } = useLanguage()
  const [internalLang, setInternalLang] = useState(currentLanguage)

  const handleToggleLanguage = () => {
    const nextLang = internalLang === 'en' ? 'fr' : 'en'
    setInternalLang(nextLang)
    changeLanguage(nextLang)
  }

  return (
    <>
      <button
        onClick={handleToggleLanguage}
        className='flex items-center justify-center rounded-md p-2 transition-colors duration-300 hover:cursor-pointer hover:bg-gray-200 dark:hover:bg-gray-800'
      >
        <AnimatePresence mode='wait'>
          <motion.span
            key={internalLang}
            initial={{ opacity: 1, rotateY: -90 }}
            animate={{ opacity: 1, rotateY: 0 }}
            exit={{ opacity: 1, rotateY: 90 }}
            transition={{ duration: 0.15 }}
          >
            <ReactCountryFlag
              countryCode={internalLang === 'en' ? 'US' : 'FR'}
              svg
              style={{ width: '1.5em', height: '1.5em' }}
            />
          </motion.span>
        </AnimatePresence>
      </button>
      <button
        onClick={toggleTheme}
        className='flex items-center justify-center rounded-md p-2 transition-colors duration-300 hover:cursor-pointer hover:bg-gray-200 dark:hover:bg-gray-800'
      >
        <DarkModeIcon className='h-6' isDark={theme === 'dark'} />
      </button>
    </>
  )
}

export default PreferencesToggle
