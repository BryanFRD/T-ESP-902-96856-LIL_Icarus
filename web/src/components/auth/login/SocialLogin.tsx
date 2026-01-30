import { useTranslation } from 'react-i18next'
import { SocialIcon } from 'react-social-icons'

import SocialButton from '@/components/auth/login/SocialButton.tsx'

const SocialLogin = () => {
  const { t } = useTranslation()

  const openExternal = (url: string) => {
    globalThis.location.href = url
  }

  return (
    <div>
      <div className='relative my-8'>
        <div className='absolute inset-0 flex items-center'>
          {/* Light border with dark variant */}
          <div className='w-full border-t border-gray-300 transition-all duration-300 dark:border-gray-700'></div>
        </div>
        <div className='relative flex justify-center text-sm'>
          {/* Light background and text with dark variants */}
          <span className='bg-white px-4 text-gray-500 transition-all duration-300 dark:bg-gray-900 dark:text-gray-400'>
            {t('social.orContinueWith')}
          </span>
        </div>
      </div>

      <div className='grid grid-cols-2 gap-4'>
        <SocialButton
          icon={<SocialIcon network={'google'} />}
          label={t('social.google')}
          onClick={() => openExternal('https://google.com')}
        />
        <SocialButton
          icon={<SocialIcon network={'github'} />}
          label={t('social.github')}
          onClick={() => openExternal('https://github.com')}
        />
      </div>
    </div>
  )
}

export default SocialLogin
