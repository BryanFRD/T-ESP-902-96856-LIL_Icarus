import { Lock, Shield, Zap } from 'lucide-react'
import { useTranslation } from 'react-i18next'

import Logo from '@/components/icon/Logo.tsx'

import FeatureItem from './FeatureItem.tsx'

const BrandingSide = () => {
  const { t } = useTranslation()

  return (
    <div className='relative hidden flex-col justify-between overflow-hidden p-12 text-gray-900 transition-all duration-300 md:flex md:w-1/2 dark:text-white'>
      <div className='relative z-10'>
        <div className='mb-8 flex items-center gap-3'>
          <Logo fill={'#ff6900'} className='h-16 w-16' />
          <h1 className='text-3xl font-bold'>{t('app:name')}</h1>
        </div>
        <h2 className='mb-4 text-4xl leading-tight font-bold'>
          {t('app:branding.advancedPenetration')}
          <br />
          {t('app:branding.testingPlatform')}
        </h2>
        <p className='text-lg text-blue-900 transition-all duration-300 dark:text-blue-100'>
          {t('app:branding.secureInfra')}
        </p>
      </div>

      <div className='relative z-10 space-y-6'>
        <FeatureItem
          icon={Zap}
          title={t('app:branding.realtimeScanning')}
          description={t('app:branding.realtimeScanningDesc')}
        />
        <FeatureItem
          icon={Lock}
          title={t('app:branding.enterpriseSecurity')}
          description={t('app:branding.enterpriseSecurityDesc')}
        />
        <FeatureItem
          icon={Shield}
          title={t('app:branding.advancedReporting')}
          description={t('app:branding.advancedReportingDesc')}
        />
      </div>

      <div className='relative z-10'>
        <blockquote className='text-lg text-gray-800 italic transition-all duration-300 dark:text-white/90'>
          {t('app:branding.securityQuote')}
        </blockquote>
        <cite className='mt-2 block text-sm text-blue-700 transition-all duration-300 dark:text-blue-200'>
          — {t('app:branding.securityExpert')}
        </cite>
      </div>
    </div>
  )
}

export default BrandingSide
