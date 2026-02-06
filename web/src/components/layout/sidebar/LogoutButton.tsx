import { LogOutIcon } from 'lucide-react'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'
import { useNavigate } from 'react-router-dom'

import { useUserContext } from '@/context/UserContext.tsx'
import type { ApiError } from '@/utils/apiUtils.ts'

import Button from '../../ui/button/Button.tsx'

const LogoutButton = () => {
  const { logout } = useUserContext()
  const { t } = useTranslation()
  const navigate = useNavigate()
  return (
    <Button
      size='md'
      onClick={() =>
        toast
          .promise(logout(), {
            success: t('messages:success.logout'),
            error: (err: ApiError) => t(err.errorKey),
            loading: t('messages:loading.logout')
          })
          .then(() => navigate('/'))
      }
      className='flex justify-start gap-4'
      variant='ghost'
      rounded='none'
      fullWidth
    >
      <LogOutIcon
        size={16}
        className='shrink-0 text-red-500/80 group-hover:text-red-600'
      />
      <span className='text-sm text-red-500/80'>
        {t('auth:actions.logout')}
      </span>
    </Button>
  )
}

export default LogoutButton
