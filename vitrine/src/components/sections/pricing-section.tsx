'use client'

import { Check } from 'lucide-react'
import { useState } from 'react'

import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle
} from '@/components/ui/card'
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { useScopedI18n } from '@/i18n/client'

export function PricingSection() {
  const t = useScopedI18n('pages.home.pricing')
  const tBadge = useScopedI18n('components.pricingSection')
  const [billingCycle, setBillingCycle] = useState<'monthly' | 'yearly'>(
    'yearly'
  )

  const tiers = [
    {
      name: t('tiers.starter.name'),
      description: t('tiers.starter.description'),
      monthlyPrice: 49,
      yearlyPrice: 39,
      features: [
        t('tiers.starter.features.0'),
        t('tiers.starter.features.1'),
        t('tiers.starter.features.2'),
        t('tiers.starter.features.3'),
        t('tiers.starter.features.4')
      ],
      cta: t('tiers.starter.cta')
    },
    {
      name: t('tiers.pro.name'),
      description: t('tiers.pro.description'),
      monthlyPrice: 129,
      yearlyPrice: 99,
      popular: true,
      features: [
        t('tiers.pro.features.0'),
        t('tiers.pro.features.1'),
        t('tiers.pro.features.2'),
        t('tiers.pro.features.3'),
        t('tiers.pro.features.4'),
        t('tiers.pro.features.5'),
        t('tiers.pro.features.6')
      ],
      cta: t('tiers.pro.cta')
    },
    {
      name: t('tiers.enterprise.name'),
      description: t('tiers.enterprise.description'),
      monthlyPrice: undefined,
      yearlyPrice: undefined,
      features: [
        t('tiers.enterprise.features.0'),
        t('tiers.enterprise.features.1'),
        t('tiers.enterprise.features.2'),
        t('tiers.enterprise.features.3'),
        t('tiers.enterprise.features.4'),
        t('tiers.enterprise.features.5'),
        t('tiers.enterprise.features.6')
      ],
      cta: t('tiers.enterprise.cta')
    }
  ]

  return (
    <section className='py-20' id='pricing'>
      <div className='container mx-auto px-4'>
        <div className='mx-auto max-w-3xl text-center'>
          <h2 className='mb-4 text-3xl font-bold tracking-tight md:text-4xl'>
            {t('title')}
          </h2>
          <p className='text-muted-foreground mb-8 text-lg'>{t('subtitle')}</p>

          {/* Billing Toggle */}
          <Tabs
            defaultValue='yearly'
            className='mx-auto mb-12 inline-block'
            onValueChange={value =>
              setBillingCycle(value as 'monthly' | 'yearly')
            }
          >
            <TabsList>
              <TabsTrigger value='monthly'>
                {t('frequencies.monthly')}
              </TabsTrigger>
              <TabsTrigger value='yearly'>
                {t('frequencies.yearly')}
                <Badge variant='secondary' className='ml-2'>
                  {tBadge('discount', { param: '20' })}
                </Badge>
              </TabsTrigger>
            </TabsList>
          </Tabs>
        </div>

        <div className='grid gap-8 lg:grid-cols-3'>
          {tiers.map((tier, idx) => (
            <Card
              key={idx}
              className={
                tier.popular
                  ? 'border-primary ring-primary/20 pt-0 shadow-lg ring-2'
                  : ''
              }
            >
              {tier.popular && (
                <div className='bg-primary text-primary-foreground rounded-t-lg px-4 py-2 text-center text-sm font-medium'>
                  {tBadge('popular')}
                </div>
              )}
              <CardHeader>
                <CardTitle>{tier.name}</CardTitle>
                <CardDescription>{tier.description}</CardDescription>
                <div className='mt-4'>
                  {tier.monthlyPrice ? (
                    <>
                      <span className='text-4xl font-bold'>
                        €
                        {billingCycle === 'yearly'
                          ? tier.yearlyPrice
                          : tier.monthlyPrice}
                      </span>
                      <span className='text-muted-foreground'>
                        {' '}
                        /{tBadge('perMonth')}
                      </span>
                    </>
                  ) : (
                    <span className='text-4xl font-bold'>
                      {t('tiers.enterprise.price')}
                    </span>
                  )}
                </div>
              </CardHeader>
              <CardContent>
                <ul className='mb-6 space-y-3'>
                  {tier.features.map((feature, featureIdx) => (
                    <li key={featureIdx} className='flex gap-3'>
                      <Check className='text-primary h-5 w-5 shrink-0' />
                      <span className='text-sm'>{feature}</span>
                    </li>
                  ))}
                </ul>
                <Button
                  className='w-full'
                  variant={tier.popular ? 'default' : 'outline'}
                  asChild
                >
                  <a href='#contact'>{tier.cta}</a>
                </Button>
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    </section>
  )
}
