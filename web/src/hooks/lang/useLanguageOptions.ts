import { useTranslation } from 'react-i18next'

import type { Language } from '@/types/global.ts'

export const useLanguageOptions = () => {
  const { t } = useTranslation()
  return [
    { code: 'en', label: t('common:languages.english') },
    { code: 'fr', label: t('common:languages.french') }
  ] as { code: Language; label: string }[]
}

export type LanguageOption = ReturnType<typeof useLanguageOptions>[number]
