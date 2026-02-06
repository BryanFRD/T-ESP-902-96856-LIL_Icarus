import { useTranslation } from 'react-i18next'

import Logo from '@/components/icon/Logo.tsx'

const RegisterHeader = () => {
  const { t } = useTranslation()

  return (
    <div className='mb-8 text-center'>
      <div className='mb-4 inline-flex h-16 w-16 items-center justify-center rounded-2xl'>
        <Logo fill={'#ff6900'} className='h-20 w-20' />
      </div>

      <h1 className='dark:from-blue text-3xl font-bold text-gray-900 transition-all duration-300 dark:text-white'>
        {t('auth:signUp.createAccount')}
      </h1>

      {/* Subtitle with light and dark text colors */}
      <p className='mt-2 text-gray-900 transition-all duration-300 dark:text-gray-400'>
        {t('auth:signUp.subtitle')}
      </p>
    </div>
  )
}

export default RegisterHeader
