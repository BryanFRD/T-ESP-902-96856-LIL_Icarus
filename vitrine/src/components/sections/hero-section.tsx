'use client'

import { motion } from 'framer-motion'
import { ArrowRight, Check, Shield } from 'lucide-react'
import Link from 'next/link'

import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { useScopedI18n } from '@/i18n/client'

export function HeroSection() {
  const t = useScopedI18n('pages.home.hero')

  return (
    <section className='relative overflow-hidden py-20 md:py-32'>
      {/* Background gradient */}
      <div className='absolute inset-0 -z-10 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:24px_24px]' />
      <div className='from-primary/5 absolute inset-0 -z-10 bg-gradient-to-b via-transparent to-transparent' />

      <div className='container mx-auto px-4'>
        <div className='mx-auto max-w-4xl text-center'>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5 }}
          >
            <Badge variant='secondary' className='mb-6'>
              <Shield className='mr-2 h-3 w-3' />
              {t('badge')}
            </Badge>

            <h1 className='mb-6 text-4xl font-bold tracking-tight md:text-6xl'>
              {t('title')}
            </h1>

            <p className='text-muted-foreground mb-8 text-lg md:text-xl'>
              {t('subtitle')}
            </p>

            <div className='flex flex-col gap-4 sm:flex-row sm:justify-center'>
              <Button size='lg' asChild>
                <Link href='#contact'>
                  {t('getStarted')}
                  <ArrowRight className='ml-2 h-5 w-5' />
                </Link>
              </Button>
              <Button size='lg' variant='outline' asChild>
                <Link href='#solution'>{t('learnMore')}</Link>
              </Button>
            </div>

            {/* Feature badges */}
            <div className='mt-12 flex flex-wrap justify-center gap-4'>
              {[t('features.0'), t('features.1'), t('features.2')].map(
                (feature, idx) => (
                  <div
                    key={idx}
                    className='bg-muted flex items-center gap-2 rounded-full px-4 py-2 text-sm'
                  >
                    <Check className='text-primary h-4 w-4' />
                    <span>{feature}</span>
                  </div>
                )
              )}
            </div>
          </motion.div>

          {/* Hero Image / Mockup */}
          <motion.div
            initial={{ opacity: 0, y: 40 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.7, delay: 0.2 }}
            className='mt-16'
          >
            <div className='bg-muted relative mx-auto aspect-video max-w-5xl overflow-hidden rounded-lg border shadow-2xl'>
              <div className='from-primary/20 to-primary/5 absolute inset-0 bg-gradient-to-tr' />
              <div className='flex h-full items-center justify-center'>
                <Shield className='text-primary/20 h-32 w-32' />
              </div>
              {/* Placeholder for actual screenshot */}
              <div className='text-muted-foreground absolute inset-0 flex items-center justify-center'>
                <span className='text-lg font-medium'>
                  Dashboard Preview Coming Soon
                </span>
              </div>
            </div>
          </motion.div>
        </div>
      </div>
    </section>
  )
}
