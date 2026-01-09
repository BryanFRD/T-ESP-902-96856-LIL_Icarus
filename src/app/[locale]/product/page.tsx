'use client'

import { motion } from 'framer-motion'
import { Check, FileText, Shield, Terminal, Users } from 'lucide-react'
import Link from 'next/link'

import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle
} from '@/components/ui/card'
import { useScopedI18n } from '@/i18n/client'

export default function ProductPage() {
  const t = useScopedI18n('pages.product')

  const benefits = [
    t('benefits.0'),
    t('benefits.1'),
    t('benefits.2'),
    t('benefits.3'),
    t('benefits.4')
  ]

  const detailedFeatures = [
    {
      title: t('features.0.title'),
      description: t('features.0.description'),
      items: [
        t('features.0.items.0'),
        t('features.0.items.1'),
        t('features.0.items.2'),
        t('features.0.items.3')
      ],
      icon: Terminal
    },
    {
      title: t('features.1.title'),
      description: t('features.1.description'),
      items: [
        t('features.1.items.0'),
        t('features.1.items.1'),
        t('features.1.items.2'),
        t('features.1.items.3')
      ],
      icon: Users
    },
    {
      title: t('features.2.title'),
      description: t('features.2.description'),
      items: [
        t('features.2.items.0'),
        t('features.2.items.1'),
        t('features.2.items.2'),
        t('features.2.items.3')
      ],
      icon: FileText
    },
    {
      title: t('features.3.title'),
      description: t('features.3.description'),
      items: [
        t('features.3.items.0'),
        t('features.3.items.1'),
        t('features.3.items.2'),
        t('features.3.items.3')
      ],
      icon: Shield
    }
  ]

  return (
    <div className='py-20'>
      {/* Hero */}
      <section className='container mx-auto px-4'>
        <div className='mx-auto max-w-3xl text-center'>
          <Badge variant='outline' className='mb-4'>
            {t('badge')}
          </Badge>
          <h1 className='mb-4 text-4xl font-bold tracking-tight md:text-5xl'>
            {t('title')}
          </h1>
          <p className='text-muted-foreground mb-8 text-lg'>{t('subtitle')}</p>

          <div className='flex flex-col gap-4 sm:flex-row sm:justify-center'>
            <Button size='lg' asChild>
              <Link href='#contact'>{t('ctaTry')}</Link>
            </Button>
            <Button size='lg' variant='outline' asChild>
              <Link href='#features'>{t('ctaDemo')}</Link>
            </Button>
          </div>
        </div>
      </section>

      {/* Benefits */}
      <section className='container mx-auto mt-20 px-4'>
        <div className='mx-auto max-w-3xl text-center'>
          <h2 className='mb-12 text-3xl font-bold'>{t('benefitsTitle')}</h2>
        </div>

        <div className='mx-auto max-w-3xl space-y-4'>
          {benefits.map((benefit, idx) => (
            <motion.div
              key={idx}
              initial={{ opacity: 0, x: -20 }}
              whileInView={{ opacity: 1, x: 0 }}
              transition={{ duration: 0.5, delay: idx * 0.1 }}
              viewport={{ once: true }}
              className='bg-card flex items-start gap-3 rounded-lg border p-4'
            >
              <Check className='text-primary mt-1 h-5 w-5 shrink-0' />
              <p className='text-muted-foreground'>{benefit}</p>
            </motion.div>
          ))}
        </div>
      </section>

      {/* Detailed Features */}
      <section className='container mx-auto mt-20 px-4' id='features'>
        <div className='mx-auto mb-12 max-w-3xl text-center'>
          <h2 className='mb-4 text-3xl font-bold'>{t('featuresTitle')}</h2>
          <p className='text-muted-foreground text-lg'>
            {t('featuresSubtitle')}
          </p>
        </div>

        <div className='space-y-12'>
          {detailedFeatures.map((feature, idx) => {
            const Icon = feature.icon
            return (
              <motion.div
                key={idx}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.5 }}
                viewport={{ once: true }}
              >
                <Card>
                  <CardHeader>
                    <div className='flex items-center gap-4'>
                      <div className='bg-primary/10 flex h-14 w-14 items-center justify-center rounded-lg'>
                        <Icon className='text-primary h-7 w-7' />
                      </div>
                      <div>
                        <CardTitle className='text-2xl'>
                          {feature.title}
                        </CardTitle>
                        <CardDescription className='mt-1'>
                          {feature.description}
                        </CardDescription>
                      </div>
                    </div>
                  </CardHeader>
                  <CardContent>
                    <ul className='grid gap-3 md:grid-cols-2'>
                      {feature.items.map((item, itemIdx) => (
                        <li key={itemIdx} className='flex gap-3'>
                          <Check className='text-primary mt-0.5 h-5 w-5 shrink-0' />
                          <span className='text-muted-foreground text-sm'>
                            {item}
                          </span>
                        </li>
                      ))}
                    </ul>
                  </CardContent>
                </Card>
              </motion.div>
            )
          })}
        </div>
      </section>

      {/* CTA */}
      <section className='container mx-auto mt-20 px-4'>
        <div className='bg-primary/5 mx-auto max-w-3xl rounded-lg p-8 text-center'>
          <h2 className='mb-4 text-3xl font-bold'>{t('cta.title')}</h2>
          <p className='text-muted-foreground mb-6 text-lg'>
            {t('cta.description')}
          </p>
          <Button size='lg' asChild>
            <Link href='#contact'>{t('cta.button')}</Link>
          </Button>
        </div>
      </section>
    </div>
  )
}
