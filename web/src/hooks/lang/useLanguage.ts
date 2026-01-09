import { useTranslation } from 'react-i18next'

import type { Language } from '@/types/global.ts'

const useLanguage = () => {
  const { i18n } = useTranslation()

  const changeLanguage = (lang: Language) => {
    i18n.changeLanguage(lang)
  }

  return {
    currentLanguage: i18n.language,
    changeLanguage
  }
}

export default useLanguage
