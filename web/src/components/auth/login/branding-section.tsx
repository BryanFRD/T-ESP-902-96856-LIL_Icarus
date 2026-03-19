import { Lock, Shield, Zap } from 'lucide-react'
import { useTranslation } from 'react-i18next'

import Logo from '@/components/icon/logo.tsx'

import { FeatureItem } from './feature-item.tsx'

export function BrandingSection() {
  const { t } = useTranslation(['app', 'auth'])

  return (
    <div className='hidden space-y-6 md:block'>
      <div className='flex items-center gap-x-2'>
        <Logo fill={'#ff6900'} className='size-14' />
        <h1 className='text-3xl font-bold'>{t('app:name')}</h1>
      </div>

      <div>
        <h2 className='text-4xl leading-tight font-bold'>
          {t('auth:login.branding.title')}
        </h2>
        <h2 className='text-3xl leading-tight font-bold'>
          {t('auth:login.branding.subtitle')}
        </h2>
      </div>

      <p className='text-muted-foreground text-lg'>
        {t('auth:login.branding.secureInfrastructure')}
      </p>

      <FeatureItem
        icon={Zap}
        title={t('auth:login.branding.features.realtimeScanning.title')}
        description={t(
          'auth:login.branding.features.realtimeScanning.description'
        )}
      />
      <FeatureItem
        icon={Lock}
        title={t('auth:login.branding.features.entrepriseSecurity.title')}
        description={t(
          'auth:login.branding.features.entrepriseSecurity.description'
        )}
      />
      <FeatureItem
        icon={Shield}
        title={t('auth:login.branding.features.advancedReporting.title')}
        description={t(
          'auth:login.branding.features.advancedReporting.description'
        )}
      />

      <div>
        <blockquote className='italic'>
          {t('auth:login.branding.security.quote')}
        </blockquote>
        <cite className='text-muted-foreground text-sm'>
          — {t('auth:login.branding.security.author')}
        </cite>
      </div>
    </div>
  )
}
