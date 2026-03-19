import { useTranslation } from 'react-i18next'

import Logo from '@/components/icon/logo.tsx'

export function RegisterHeader() {
  const { t } = useTranslation('auth')

  return (
    <div className='flex flex-col items-center gap-y-4'>
      <Logo fill={'#ff6900'} className='size-20' />

      <div className='space-y-2 text-center'>
        <h1 className='text-3xl font-bold'>{t('register.title')}</h1>
        <h2>{t('register.subtitle')}</h2>
      </div>
    </div>
  )
}
