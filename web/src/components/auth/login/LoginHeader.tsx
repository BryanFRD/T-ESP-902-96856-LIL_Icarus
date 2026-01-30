import { useTranslation } from 'react-i18next'

import Logo from '@/components/icon/Logo.tsx'

const LoginHeader = () => {
  const { t } = useTranslation()

  return (
    <div className='mb-8 text-center'>
      <div className='mb-4 inline-flex h-16 w-16 items-center justify-center'>
        <Logo fill={'#ff6900'} className='h-20 w-20' />
      </div>

      <h1 className='dark:from-blue text-3xl font-bold text-gray-900 transition-all duration-300 dark:text-white'>
        {t('welcomeBack')}
      </h1>

      <p className='mt-2 text-gray-900 transition-all duration-300 dark:text-gray-400'>
        {t('signInToDashboard')}
      </p>
    </div>
  )
}

export default LoginHeader
