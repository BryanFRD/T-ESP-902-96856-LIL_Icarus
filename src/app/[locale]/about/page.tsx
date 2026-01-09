'use client'

import { motion } from 'framer-motion'
import { Building2, Globe2, Heart, Shield, Sparkles, Users } from 'lucide-react'

import { Badge } from '@/components/ui/badge'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle
} from '@/components/ui/card'
import { useScopedI18n } from '@/i18n/client'

export default function AboutPage() {
  const t = useScopedI18n('pages.about')

  const values = [
    {
      title: t('values.0.title'),
      description: t('values.0.description'),
      icon: Shield
    },
    {
      title: t('values.1.title'),
      description: t('values.1.description'),
      icon: Users
    },
    {
      title: t('values.2.title'),
      description: t('values.2.description'),
      icon: Sparkles
    },
    {
      title: t('values.3.title'),
      description: t('values.3.description'),
      icon: Heart
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
          <p className='text-muted-foreground text-lg'>{t('subtitle')}</p>
        </div>
      </section>

      {/* Mission */}
      <section className='container mx-auto mt-20 px-4'>
        <div className='grid gap-12 lg:grid-cols-2 lg:items-center'>
          <motion.div
            initial={{ opacity: 0, x: -20 }}
            whileInView={{ opacity: 1, x: 0 }}
            transition={{ duration: 0.5 }}
            viewport={{ once: true }}
          >
            <h2 className='mb-4 text-3xl font-bold'>{t('mission.title')}</h2>
            <p className='text-muted-foreground mb-4'>
              {t('mission.description')}
            </p>
            <p className='text-muted-foreground'>{t('mission.detail')}</p>
          </motion.div>

          <motion.div
            initial={{ opacity: 0, x: 20 }}
            whileInView={{ opacity: 1, x: 0 }}
            transition={{ duration: 0.5 }}
            viewport={{ once: true }}
          >
            <Card>
              <CardHeader>
                <CardTitle>{t('mission.stats.title')}</CardTitle>
              </CardHeader>
              <CardContent className='grid grid-cols-3 gap-6'>
                <div className='text-center'>
                  <Building2 className='text-primary mx-auto mb-2 h-8 w-8' />
                  <div className='text-2xl font-bold'>
                    {t('mission.stats.founded')}
                  </div>
                  <div className='text-muted-foreground text-sm'>
                    {t('mission.stats.foundedLabel')}
                  </div>
                </div>
                <div className='text-center'>
                  <Users className='text-primary mx-auto mb-2 h-8 w-8' />
                  <div className='text-2xl font-bold'>
                    {t('mission.stats.users')}
                  </div>
                  <div className='text-muted-foreground text-sm'>
                    {t('mission.stats.usersLabel')}
                  </div>
                </div>
                <div className='text-center'>
                  <Globe2 className='text-primary mx-auto mb-2 h-8 w-8' />
                  <div className='text-2xl font-bold'>
                    {t('mission.stats.countries')}
                  </div>
                  <div className='text-muted-foreground text-sm'>
                    {t('mission.stats.countriesLabel')}
                  </div>
                </div>
              </CardContent>
            </Card>
          </motion.div>
        </div>
      </section>

      {/* Values */}
      <section className='container mx-auto mt-20 px-4'>
        <div className='mx-auto mb-12 max-w-3xl text-center'>
          <h2 className='mb-4 text-3xl font-bold'>{t('valuesTitle')}</h2>
          <p className='text-muted-foreground text-lg'>{t('valuesSubtitle')}</p>
        </div>

        <div className='grid gap-6 md:grid-cols-2 lg:grid-cols-4'>
          {values.map((value, idx) => {
            const Icon = value.icon
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
                    <CardTitle className='text-lg'>{value.title}</CardTitle>
                  </CardHeader>
                  <CardContent>
                    <CardDescription>{value.description}</CardDescription>
                  </CardContent>
                </Card>
              </motion.div>
            )
          })}
        </div>
      </section>

      {/* Team */}
      {/* <section className='container mx-auto mt-20 px-4'>
        <div className='bg-muted/50 mx-auto max-w-3xl rounded-lg p-8 text-center'>
          <h2 className='mb-4 text-2xl font-bold'>{t('team.title')}</h2>
          <p className='text-muted-foreground mb-6'>{t('team.description')}</p>
          <p className='text-muted-foreground text-sm'>{t('team.contact')}</p>
        </div>
      </section> */}
    </div>
  )
}
