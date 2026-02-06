import { ChevronRight } from 'lucide-react'
import { useState } from 'react'
import ReactCountryFlag from 'react-country-flag'
import { useTranslation } from 'react-i18next'

import useLanguage from '@/hooks/lang/useLanguage'
import { useLanguageOptions } from '@/hooks/lang/useLanguageOptions'

const LanguageSelector = () => {
  const { currentLanguage, changeLanguage } = useLanguage()
  const { t } = useTranslation()
  const options = useLanguageOptions()

  const [isOpen, setIsOpen] = useState(false)

  const toggleDropdown = () => setIsOpen(!isOpen)

  const handleSelect = (lang: string) => {
    changeLanguage(lang as 'en' | 'fr')
    setIsOpen(false)
  }

  return (
    <>
      <button
        onClick={toggleDropdown}
        className='flex w-full cursor-pointer items-center justify-between px-4 py-3 text-sm font-medium text-gray-300 transition-all duration-300 hover:bg-gray-800/50 hover:text-white'
      >
        <span>{t('common:language')}</span>
        <span className='ml-3 flex items-center gap-2'>
          <ReactCountryFlag
            countryCode={currentLanguage === 'en' ? 'US' : 'FR'}
            svg
            style={{ width: '1.2em', height: '1.2em' }}
          />
          <ChevronRight
            size={16}
            className={`transition-all ${isOpen ? 'rotate-90' : ''}`}
          />
        </span>
      </button>
      <div
        className={`overflow-hidden transition-all duration-200 ${isOpen ? 'h-18' : 'h-0'}`}
      >
        {options.map(opt => (
          <button
            key={opt.code}
            onClick={() => handleSelect(opt.code)}
            className={`flex w-full cursor-pointer items-center justify-between py-2 ps-8 pe-4 text-left text-sm font-medium transition-all duration-150 hover:bg-gray-800 ${
              opt.code === currentLanguage ? 'text-blue-400' : 'text-gray-300'
            }`}
          >
            <span>{opt.label}</span>
            <ReactCountryFlag
              countryCode={opt.code === 'en' ? 'US' : 'FR'}
              svg
              style={{ width: '1.2em', height: '1.2em' }}
            />
          </button>
        ))}
      </div>
    </>
  )
}

export default LanguageSelector
