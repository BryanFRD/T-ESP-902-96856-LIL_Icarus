import { ThemeProvider } from 'next-themes'

import { I18nProviderClient } from '@/i18n/client'

type ProvidersProps = React.PropsWithChildren<{
  locale: string
}>

export function Providers({ children, locale }: ProvidersProps) {
  return (
    <I18nProviderClient locale={locale}>
      <ThemeProvider
        attribute='class'
        defaultTheme='system'
        enableSystem
        disableTransitionOnChange
      >
        {children}
      </ThemeProvider>
    </I18nProviderClient>
  )
}
