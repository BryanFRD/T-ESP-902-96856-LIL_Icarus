'use client'

import { motion } from 'framer-motion'
import {
  BarChart3,
  FileText,
  Shield,
  Terminal,
  Users,
  Workflow
} from 'lucide-react'

import { Badge } from '@/components/ui/badge'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle
} from '@/components/ui/card'
import { useScopedI18n } from '@/i18n/client'

export function FeaturesSection() {
  const t = useScopedI18n('pages.home.features')

  const features = [
    {
      title: t('cards.0.title'),
      description: t('cards.0.description'),
      icon: Terminal
    },
    {
      title: t('cards.1.title'),
      description: t('cards.1.description'),
      icon: Users
    },
    {
      title: t('cards.2.title'),
      description: t('cards.2.description'),
      icon: BarChart3
    },
    {
      title: t('cards.3.title'),
      description: t('cards.3.description'),
      icon: Shield
    },
    {
      title: t('cards.4.title'),
      description: t('cards.4.description'),
      icon: Workflow
    },
    {
      title: t('cards.5.title'),
      description: t('cards.5.description'),
      icon: FileText
    }
  ]

  return (
    <section className='bg-muted/50 py-20' id='features'>
      <div className='container mx-auto px-4'>
        <div className='mx-auto max-w-3xl text-center'>
          <Badge variant='outline' className='mb-4'>
            {t('badge')}
          </Badge>
          <h2 className='mb-4 text-3xl font-bold tracking-tight md:text-4xl'>
            {t('title')}
          </h2>
          <p className='text-muted-foreground mb-12 text-lg'>
            {t('description')}
          </p>
        </div>

        <div className='grid gap-6 md:grid-cols-2 lg:grid-cols-3'>
          {features.map((feature, idx) => {
            const Icon = feature.icon
            return (
              <motion.div
                key={idx}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.5, delay: idx * 0.1 }}
                viewport={{ once: true }}
              >
                <Card className='hover:border-primary h-full transition-all hover:shadow-md'>
                  <CardHeader>
                    <div className='bg-primary/10 mb-3 flex h-12 w-12 items-center justify-center rounded-lg'>
                      <Icon className='text-primary h-6 w-6' />
                    </div>
                    <CardTitle className='text-lg'>{feature.title}</CardTitle>
                  </CardHeader>
                  <CardContent>
                    <CardDescription>{feature.description}</CardDescription>
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
