'use client'

import NextLink from 'next/link'

import { useCurrentLocale } from '@/i18n/client'

export function Link({
  href,
  ...props
}: React.ComponentProps<typeof NextLink>) {
  const locale = useCurrentLocale()

  return <NextLink href={`/${locale}${href}`} {...props} />
}
