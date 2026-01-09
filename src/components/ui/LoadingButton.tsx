import { Loader2 } from 'lucide-react'
import { useTranslation } from 'react-i18next'

import type { LoadingButtonProps } from '@/types/global.ts'

const LoadingButton = ({
  isLoading,
  children,
  loadingText,
  className = '',
  disabled = false,
  type = 'button'
}: LoadingButtonProps) => {
  const { t } = useTranslation()
  const text = loadingText ?? t('loading.default')

  return (
    <button
      type={type}
      disabled={isLoading || disabled}
      className={`w-full transform cursor-pointer rounded-xl bg-gradient-to-r from-red-500 to-orange-500 px-6 py-3 font-semibold text-white shadow-lg transition-all duration-200 hover:scale-[1.02] focus:ring-4 focus:ring-blue-200 focus:outline-none disabled:transform-none disabled:cursor-not-allowed disabled:opacity-50 ${className}`}
    >
      {isLoading ? (
        <div className='flex items-center justify-center gap-2'>
          <Loader2 className='h-5 w-5 animate-spin' />
          {text}
        </div>
      ) : (
        children
      )}
    </button>
  )
}

export default LoadingButton
