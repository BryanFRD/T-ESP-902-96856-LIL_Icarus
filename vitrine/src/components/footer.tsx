import { Shield } from 'lucide-react'
import Link from 'next/link'

export function Footer({ locale }: { locale: string }) {
  const currentYear = new Date().getFullYear()

  return (
    <footer className='bg-muted/50 border-t'>
      <div className='container mx-auto px-4 py-12'>
        <div className='grid gap-8 md:grid-cols-4'>
          {/* Brand */}
          <div className='space-y-4'>
            <Link href={`/${locale}`} className='flex items-center gap-2'>
              <Shield className='text-primary h-6 w-6' />
              <span className='text-lg font-bold'>ICARUS</span>
            </Link>
            <p className='text-muted-foreground text-sm'>
              {locale === 'fr'
                ? 'La plateforme souveraine et collaborative pour des pentests professionnels.'
                : 'The sovereign and collaborative platform for professional pentesting.'}
            </p>
          </div>

          {/* Product */}
          <div className='space-y-4'>
            <h3 className='font-semibold'>
              {locale === 'fr' ? 'Produit' : 'Product'}
            </h3>
            <ul className='text-muted-foreground space-y-2 text-sm'>
              <li>
                <Link
                  href={`/${locale}/product`}
                  className='hover:text-foreground'
                >
                  {locale === 'fr' ? 'Fonctionnalités' : 'Features'}
                </Link>
              </li>
              <li>
                <Link href='#pricing' className='hover:text-foreground'>
                  {locale === 'fr' ? 'Tarifs' : 'Pricing'}
                </Link>
              </li>
              <li>
                <Link href='#faq' className='hover:text-foreground'>
                  FAQ
                </Link>
              </li>
            </ul>
          </div>

          {/* Company */}
          <div className='space-y-4'>
            <h3 className='font-semibold'>
              {locale === 'fr' ? 'Entreprise' : 'Company'}
            </h3>
            <ul className='text-muted-foreground space-y-2 text-sm'>
              <li>
                <Link
                  href={`/${locale}/about`}
                  className='hover:text-foreground'
                >
                  {locale === 'fr' ? 'À propos' : 'About'}
                </Link>
              </li>
              <li>
                <Link href='#contact' className='hover:text-foreground'>
                  Contact
                </Link>
              </li>
            </ul>
          </div>

          {/* Legal */}
          <div className='space-y-4'>
            <h3 className='font-semibold'>Legal</h3>
            <ul className='text-muted-foreground space-y-2 text-sm'>
              <li>
                <a href='#' className='hover:text-foreground'>
                  {locale === 'fr'
                    ? 'Politique de confidentialité'
                    : 'Privacy Policy'}
                </a>
              </li>
              <li>
                <a href='#' className='hover:text-foreground'>
                  {locale === 'fr'
                    ? "Conditions d'utilisation"
                    : 'Terms of Service'}
                </a>
              </li>
              <li>
                <a href='#' className='hover:text-foreground'>
                  {locale === 'fr' ? 'Mentions légales' : 'Legal Notice'}
                </a>
              </li>
            </ul>
          </div>
        </div>

        <div className='text-muted-foreground mt-12 border-t pt-8 text-center text-sm'>
          <p>
            © {currentYear} ICARUS.{' '}
            {locale === 'fr' ? 'Tous droits réservés.' : 'All rights reserved.'}{' '}
            {locale === 'fr' ? 'Hébergé en France 🇫🇷' : 'Hosted in France 🇫🇷'}
          </p>
        </div>
      </div>
    </footer>
  )
}
