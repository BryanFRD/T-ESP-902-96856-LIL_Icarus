'use client'

import { createI18nClient } from 'next-international/client'

import en from './locales/en'

export const {
  useI18n,
  useScopedI18n,
  I18nProviderClient,
  useCurrentLocale,
  useChangeLocale
} = createI18nClient(
  {
    en: () => import('./locales/en'),
    fr: () => import('./locales/fr')
  },
  { fallbackLocale: en }
)
