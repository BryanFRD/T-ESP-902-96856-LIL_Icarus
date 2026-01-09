'use client'

import { motion } from 'framer-motion'
import { Briefcase, Building, GraduationCap, User } from 'lucide-react'

import { Badge } from '@/components/ui/badge'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle
} from '@/components/ui/card'
import { useScopedI18n } from '@/i18n/client'

const caseIcons = [Briefcase, Building, User, GraduationCap]

export function UseCasesSection() {
  const t = useScopedI18n('pages.home.useCases')

  return (
    <section className='bg-muted/50 py-20' id='usecases'>
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

        <div className='grid gap-6 md:grid-cols-2'>
          {[
            {
              title: t('cases.0.title'),
              description: t('cases.0.description'),
              results: t('cases.0.results')
            },
            {
              title: t('cases.1.title'),
              description: t('cases.1.description'),
              results: t('cases.1.results')
            },
            {
              title: t('cases.2.title'),
              description: t('cases.2.description'),
              results: t('cases.2.results')
            },
            {
              title: t('cases.3.title'),
              description: t('cases.3.description'),
              results: t('cases.3.results')
            }
          ].map((item, idx) => {
            const Icon = caseIcons[idx]
            return (
              <motion.div
                key={idx}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.5, delay: idx * 0.1 }}
                viewport={{ once: true }}
              >
                <Card className='h-full'>
                  <CardHeader>
                    <div className='bg-primary/10 mb-3 flex h-12 w-12 items-center justify-center rounded-lg'>
                      <Icon className='text-primary h-6 w-6' />
                    </div>
                    <CardTitle>{item.title}</CardTitle>
                    <CardDescription>{item.description}</CardDescription>
                  </CardHeader>
                  <CardContent>
                    <p className='text-primary text-sm font-medium'>
                      {item.results}
                    </p>
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
