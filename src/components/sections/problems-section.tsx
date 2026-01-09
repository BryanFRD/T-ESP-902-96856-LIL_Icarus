'use client'

import { motion } from 'framer-motion'
import { AlertCircle, FileX2, MessageSquareOff, Settings } from 'lucide-react'

import { Badge } from '@/components/ui/badge'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle
} from '@/components/ui/card'
import { useScopedI18n } from '@/i18n/client'

const icons = [Settings, MessageSquareOff, FileX2, AlertCircle]

export function ProblemsSection() {
  const t = useScopedI18n('pages.home.problems')

  return (
    <section className='py-20' id='problems'>
      <div className='container mx-auto px-4'>
        <div className='mx-auto max-w-3xl text-center'>
          <Badge variant='outline' className='mb-4'>
            {t('badge')}
          </Badge>
          <h2 className='mb-4 text-3xl font-bold tracking-tight md:text-4xl'>
            {t('title')}
          </h2>
          <p className='text-muted-foreground mb-12 text-lg'>{t('subtitle')}</p>
        </div>

        <div className='grid gap-6 md:grid-cols-2 lg:grid-cols-4'>
          {[
            {
              title: t('items.0.title'),
              description: t('items.0.description')
            },
            {
              title: t('items.1.title'),
              description: t('items.1.description')
            },
            {
              title: t('items.2.title'),
              description: t('items.2.description')
            },
            { title: t('items.3.title'), description: t('items.3.description') }
          ].map((item, idx) => {
            const Icon = icons[idx]
            return (
              <motion.div
                key={idx}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.5, delay: idx * 0.1 }}
                viewport={{ once: true }}
              >
                <Card className='border-destructive/20 bg-destructive/5 h-full'>
                  <CardHeader>
                    <div className='bg-destructive/10 mb-2 flex h-12 w-12 items-center justify-center rounded-lg'>
                      <Icon className='text-destructive h-6 w-6' />
                    </div>
                    <CardTitle className='text-lg'>{item.title}</CardTitle>
                  </CardHeader>
                  <CardContent>
                    <CardDescription>{item.description}</CardDescription>
                  </CardContent>
                </Card>
              </motion.div>
            )
          })}
        </div>
      </div>
    </section>
  )
}
