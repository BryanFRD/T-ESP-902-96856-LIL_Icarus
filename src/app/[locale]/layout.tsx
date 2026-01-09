import type { Metadata, Viewport } from 'next'

import '@/app/globals.css'
import { Footer } from '@/components/footer'
import { Header } from '@/components/header'
import { Providers } from '@/components/providers'
import { getStaticParams } from '@/i18n/server'
import { interFont } from '@/lib/fonts'
import { cn } from '@/lib/utils'

export const metadata: Metadata = {
  title: 'ICARUS - Plateforme Souveraine de Pentesting',
  description:
    "ICARUS est la plateforme collaborative souveraine pour des pentests professionnels. Intégration d'outils, rapports automatisés, transparence client."
}

export const viewport: Viewport = {
  width: 'device-width',
  initialScale: 1,
  userScalable: false,
  themeColor: 'var(--background)'
}

export function generateStaticParams() {
  return getStaticParams()
}

type RootLayoutProps = React.PropsWithChildren<{
  params: Promise<{ locale: string }>
}>

export default async function LocaleRootLayout({
  children,
  params
}: RootLayoutProps) {
  const { locale } = await params

  return (
    <html lang={locale} suppressHydrationWarning>
      <head />

      <body className={cn(interFont.className, 'antialiased')}>
        <Providers locale={locale}>
          <Header />
          <main>{children}</main>
          <Footer locale={locale} />
        </Providers>
      </body>
    </html>
  )
}
