import { zodResolver } from '@hookform/resolvers/zod'
import { LucideMail } from 'lucide-react'
import { useForm } from 'react-hook-form'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'

import InputField from '@/components/ui/InputField.tsx'
import Button from '@/components/ui/button/Button.tsx'
import Modal from '@/components/ui/modal/Modal.tsx'
import { getPasswordSchema } from '@/domain/validation/resetPasswordSchema.ts'
import { useSendPasswordResetRequest } from '@/hooks/auth/useSendPasswordResetRequest.ts'
import type { ApiError } from '@/utils/apiUtils.ts'

const ModalSendResetPassword = ({
  isOpen,
  onClose
}: {
  isOpen: boolean
  onClose: () => void
}) => {
  const { t } = useTranslation()
  const { mutateAsync } = useSendPasswordResetRequest()
  const { register, handleSubmit } = useForm({
    resolver: zodResolver(getPasswordSchema(t))
  })

  async function onSubmit(data: { resetEmail: string }) {
    await toast.promise(
      mutateAsync({ resetEmail: data.resetEmail }).then(() => {
        onClose()
      }),
      {
        loading: t('messages:loading.sendResetPassword'),
        success: t('auth:password.forgotemailSent'),
        error: (err: ApiError) => t(err.errorKey)
      }
    )
  }

  return (
    <Modal
      size={'xl'}
      title={t('auth:password.forgottitle')}
      isOpen={isOpen}
      onClose={onClose}
      closeButtonLabel={t('auth:password.forgotonClose')}
    >
      <form
        onSubmit={handleSubmit(onSubmit)}
        className={'flex flex-col gap-6 p-2'}
      >
        <p className='text-gray-700 dark:text-gray-300'>
          {t('auth:password.forgotmessage')}
        </p>
        <InputField
          label={t('auth:password.forgotlabel')}
          type={'email'}
          icon={LucideMail}
          placeholder={t('auth:password.forgotplaceholder')}
          register={register}
          name={'resetEmail'}
        />
        <div className='mt-4 flex justify-end'>
          <Button type='submit' variant='primary'>
            {t('auth:password.forgotsendButton')}
          </Button>
        </div>
      </form>
    </Modal>
  )
}

export default ModalSendResetPassword
