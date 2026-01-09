'use client'

import { motion } from 'framer-motion'
import {
  BarChart3,
  FileText,
  Layers,
  Shield,
  Terminal,
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

const toolIcons = [Terminal, Layers, Shield, Workflow, FileText, BarChart3]

export function TechStackSection() {
  const t = useScopedI18n('pages.home.techStack')

  return (
    <section className='py-20' id='techstack'>
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

        <div className='grid gap-6 md:grid-cols-2 lg:grid-cols-3'>
          {[
            { name: t('tools.0.name'), description: t('tools.0.description') },
            { name: t('tools.1.name'), description: t('tools.1.description') },
            { name: t('tools.2.name'), description: t('tools.2.description') },
            { name: t('tools.3.name'), description: t('tools.3.description') },
            { name: t('tools.4.name'), description: t('tools.4.description') },
            { name: t('tools.5.name'), description: t('tools.5.description') }
          ].map((tool, idx) => {
            const Icon = toolIcons[idx]
            return (
              <motion.div
                key={idx}
                initial={{ opacity: 0, scale: 0.9 }}
                whileInView={{ opacity: 1, scale: 1 }}
                transition={{ duration: 0.5, delay: idx * 0.05 }}
                viewport={{ once: true }}
              >
                <Card className='hover:border-primary h-full transition-all hover:shadow-lg'>
                  <CardHeader>
                    <div className='bg-primary/10 mb-3 flex h-14 w-14 items-center justify-center rounded-lg'>
                      <Icon className='text-primary h-7 w-7' />
                    </div>
                    <CardTitle className='text-lg'>{tool.name}</CardTitle>
                  </CardHeader>
                  <CardContent>
                    <CardDescription>{tool.description}</CardDescription>
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
