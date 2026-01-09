'use client'

import { ChevronDown, Menu, Shield, X } from 'lucide-react'
import Link from 'next/link'
import { useParams } from 'next/navigation'
import { useState } from 'react'

import { useScopedI18n } from '@/i18n/client'

import { Button } from './ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger
} from './ui/dropdown-menu'

export function Header() {
  const t = useScopedI18n('components.appBar')
  const tLocale = useScopedI18n('components.localeDropdown')
  const params = useParams()
  const locale = params?.locale as string
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false)

  const navigation = [
    { name: t('links.home'), href: `/${locale}` },
    { name: t('links.product'), href: `/${locale}/product` },
    { name: t('links.about'), href: `/${locale}/about` }
  ]

  return (
    <header className='bg-background/95 supports-[backdrop-filter]:bg-background/60 sticky top-0 z-50 w-full border-b backdrop-blur'>
      <nav className='container mx-auto flex h-16 items-center px-4'>
        {/* Logo */}
        <Link href={`/${locale}`} className='flex items-center gap-2'>
          <Shield className='text-primary h-8 w-8' />
          <span className='text-xl font-bold'>ICARUS</span>
        </Link>

        {/* Desktop Navigation - Centered */}
        <div className='absolute left-1/2 hidden -translate-x-1/2 items-center gap-6 md:flex'>
          {navigation.map(item => (
            <Link
              key={item.name}
              href={item.href}
              className='text-muted-foreground hover:text-foreground text-sm font-medium transition-colors'
            >
              {item.name}
            </Link>
          ))}
        </div>

        {/* Actions */}
        <div className='ml-auto flex items-center gap-3'>
          {/* Language Selector */}
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant='ghost' size='sm' className='hidden md:flex'>
                {locale === 'fr' ? '🇫🇷' : '🇬🇧'}{' '}
                {locale === 'fr' ? tLocale('fr') : tLocale('en')}
                <ChevronDown className='ml-1 h-4 w-4' />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align='end'>
              <DropdownMenuItem asChild>
                <Link href='/fr'>🇫🇷 {tLocale('fr')}</Link>
              </DropdownMenuItem>
              <DropdownMenuItem asChild>
                <Link href='/en'>🇬🇧 {tLocale('en')}</Link>
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>

          {/* CTA Button */}
          <Button asChild className='hidden md:inline-flex'>
            <Link href={`/${locale}#contact`}>
              {locale === 'fr' ? 'Demander une démo' : 'Request a demo'}
            </Link>
          </Button>

          {/* Mobile Menu Toggle */}
          <Button
            variant='ghost'
            size='sm'
            className='md:hidden'
            onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
          >
            {mobileMenuOpen ? (
              <X className='h-5 w-5' />
            ) : (
              <Menu className='h-5 w-5' />
            )}
          </Button>
        </div>
      </nav>

      {/* Mobile Menu */}
      {mobileMenuOpen && (
        <div className='bg-background border-t md:hidden'>
          <div className='container mx-auto space-y-1 px-4 py-4'>
            {navigation.map(item => (
              <Link
                key={item.name}
                href={item.href}
                className='text-muted-foreground hover:bg-accent hover:text-foreground block rounded-md px-3 py-2 text-base font-medium'
                onClick={() => setMobileMenuOpen(false)}
              >
                {item.name}
              </Link>
            ))}
            <div className='pt-4'>
              <Button asChild className='w-full'>
                <Link href='#contact' onClick={() => setMobileMenuOpen(false)}>
                  {locale === 'fr' ? 'Demander une démo' : 'Request a demo'}
                </Link>
              </Button>
            </div>
          </div>
        </div>
      )}
    </header>
  )
}
