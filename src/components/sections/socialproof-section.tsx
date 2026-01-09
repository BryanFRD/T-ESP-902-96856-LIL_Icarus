'use client'

import NumberFlow from '@number-flow/react'
import { motion } from 'framer-motion'
import { Quote, Star } from 'lucide-react'

import { Badge } from '@/components/ui/badge'
import { Card, CardContent } from '@/components/ui/card'
import { useScopedI18n } from '@/i18n/client'

export function SocialProofSection() {
  const t = useScopedI18n('pages.home.socialProof')

  const stats = [
    { value: '250', label: t('stats.0.label') },
    { value: '1500', label: t('stats.1.label') },
    { value: '70', label: t('stats.2.label') },
    { value: '98', label: t('stats.3.label') }
  ]

  const testimonials = [
    {
      name: t('testimonials.0.name'),
      role: t('testimonials.0.role'),
      content: t('testimonials.0.content')
    },
    {
      name: t('testimonials.1.name'),
      role: t('testimonials.1.role'),
      content: t('testimonials.1.content')
    },
    {
      name: t('testimonials.2.name'),
      role: t('testimonials.2.role'),
      content: t('testimonials.2.content')
    }
  ]

  return (
    <section className='py-20' id='testimonials'>
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

        {/* Stats */}
        <div className='mb-16 grid gap-6 md:grid-cols-4'>
          {stats.map((stat, idx) => (
            <motion.div
              key={idx}
              initial={{ opacity: 0, scale: 0.9 }}
              whileInView={{ opacity: 1, scale: 1 }}
              transition={{ duration: 0.5, delay: idx * 0.1 }}
              viewport={{ once: true }}
            >
              <Card>
                <CardContent className='flex flex-col items-center justify-center p-6 text-center'>
                  <div className='text-primary mb-2 text-4xl font-bold'>
                    <NumberFlow value={Number.parseInt(stat.value, 10)} />
                    {idx > 1 && '%'}
                    {idx <= 1 && '+'}
                  </div>
                  <p className='text-muted-foreground text-sm'>{stat.label}</p>
                </CardContent>
              </Card>
            </motion.div>
          ))}
        </div>

        {/* Testimonials */}
        <div className='grid gap-6 md:grid-cols-3'>
          {testimonials.map((testimonial, idx) => (
            <motion.div
              key={idx}
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5, delay: idx * 0.1 }}
              viewport={{ once: true }}
            >
              <Card className='h-full'>
                <CardContent className='p-6'>
                  <Quote className='text-primary/20 mb-4 h-8 w-8' />
                  <p className='mb-6 text-sm italic'>{testimonial.content}</p>
                  <div className='flex items-center justify-between border-t pt-4'>
                    <div>
                      <p className='font-semibold'>{testimonial.name}</p>
                      <p className='text-muted-foreground text-xs'>
                        {testimonial.role}
                      </p>
                    </div>
                    <div className='flex gap-1'>
                      {Array.from({ length: 5 }).map((_, i) => (
                        <Star
                          key={i}
                          className='fill-primary text-primary h-4 w-4'
                        />
                      ))}
                    </div>
                  </div>
                </CardContent>
              </Card>
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  )
}
