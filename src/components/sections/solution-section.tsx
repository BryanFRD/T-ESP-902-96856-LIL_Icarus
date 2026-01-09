'use client'

import { motion } from 'framer-motion'
import { Check, Clock, Lock, Puzzle, Sparkles } from 'lucide-react'
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

export function SolutionSection() {
  const t = useScopedI18n('pages.home.solution')

  const benefitIcons = [Lock, Clock, Puzzle]

  return (
    <section className='bg-muted/50 py-20' id='solution'>
      <div className='container mx-auto px-4'>
        <div className='mx-auto max-w-3xl text-center'>
          <Badge className='mb-4'>
            <Sparkles className='mr-2 h-3 w-3' />
            {t('badge')}
          </Badge>
          <h2 className='mb-4 text-3xl font-bold tracking-tight md:text-4xl'>
            {t('title')}
          </h2>
          <p className='text-muted-foreground mb-8 text-lg'>{t('subtitle')}</p>
          <Button size='lg' asChild>
            <Link href='#contact'>{t('cta')}</Link>
          </Button>
        </div>

        {/* Main Features */}
        <div className='mt-16 grid gap-6 md:grid-cols-2'>
          {[
            t('features.0'),
            t('features.1'),
            t('features.2'),
            t('features.3')
          ].map((feature, idx) => (
            <motion.div
              key={idx}
              initial={{ opacity: 0, x: idx % 2 === 0 ? -20 : 20 }}
              whileInView={{ opacity: 1, x: 0 }}
              transition={{ duration: 0.5, delay: idx * 0.1 }}
              viewport={{ once: true }}
              className='bg-card flex items-start gap-3 rounded-lg border p-6'
            >
              <Check className='text-primary mt-1 h-5 w-5 shrink-0' />
              <p className='text-muted-foreground text-sm'>{feature}</p>
            </motion.div>
          ))}
        </div>

        {/* Benefits */}
        <div className='mt-16 grid gap-6 md:grid-cols-3'>
          {[
            {
              title: t('benefits.0.title'),
              description: t('benefits.0.description')
            },
            {
              title: t('benefits.1.title'),
              description: t('benefits.1.description')
            },
            {
              title: t('benefits.2.title'),
              description: t('benefits.2.description')
            }
          ].map((benefit, idx) => {
            const Icon = benefitIcons[idx]
            return (
              <Card key={idx} className='border-primary/20 bg-primary/5'>
                <CardHeader>
                  <div className='bg-primary/10 mb-2 flex h-12 w-12 items-center justify-center rounded-lg'>
                    <Icon className='text-primary h-6 w-6' />
                  </div>
                  <CardTitle>{benefit.title}</CardTitle>
                </CardHeader>
                <CardContent>
                  <CardDescription>{benefit.description}</CardDescription>
                </CardContent>
              </Card>
            )
          })}
        </div>
      </div>
    </section>
  )
}
