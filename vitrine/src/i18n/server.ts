import { createI18nServer } from 'next-international/server'

export const { getStaticParams } = createI18nServer({
  en: () => import('./locales/en'),
  fr: () => import('./locales/fr')
})
