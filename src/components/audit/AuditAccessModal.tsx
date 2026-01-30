import { zodResolver } from '@hookform/resolvers/zod'
import { Lock } from 'lucide-react'
import { useForm } from 'react-hook-form'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'
import { useNavigate } from 'react-router-dom'

import InputField from '@/components/ui/InputField.tsx'
import Button from '@/components/ui/button/Button.tsx'
import Modal from '@/components/ui/modal/Modal.tsx'
import {
  type VerifyAuditAccessSchemaType,
  verifyAuditAccessSchema
} from '@/domain/validation/auditSchema.ts'
import { useVerifyAuditAccess } from '@/hooks/audit/useVerifyAuditAccess.ts'
import ROUTES, { getRoutePath } from '@/router/routesName.ts'
import type { ApiError } from '@/utils/apiUtils.ts'

interface AuditAccessModalProps {
  isOpen: boolean
}

const AuditAccessModal = ({ isOpen }: AuditAccessModalProps) => {
  const { t } = useTranslation()
  const navigate = useNavigate()
  const { mutateAsync: verifyAccess } = useVerifyAuditAccess()

  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting }
  } = useForm<VerifyAuditAccessSchemaType>({
    resolver: zodResolver(verifyAuditAccessSchema(t))
  })

  const handleClose = () => {
    navigate(getRoutePath(ROUTES.DASHBOARD))
  }

  async function onSubmit(data: VerifyAuditAccessSchemaType) {
    await toast.promise(verifyAccess(data.password), {
      loading: t('audit.verifying'),
      success: t('audit.accessGranted'),
      error: (err: ApiError) => t(err.errorKey || 'audit.password.invalid')
    })
  }

  return (
    <Modal
      size='md'
      title={t('audit.access.title')}
      isOpen={isOpen}
      onClose={handleClose}
      closeButtonLabel={t('common.cancel')}
    >
      <form
        onSubmit={handleSubmit(onSubmit)}
        className='flex flex-col gap-6 p-2'
      >
        <div className='text-center text-gray-600 dark:text-gray-300'>
          <p>{t('audit.access.message')}</p>
        </div>

        <InputField
          label={t('audit.password.label')}
          type='password'
          icon={Lock}
          placeholder={t('audit.password.placeholder')}
          register={register}
          name='password'
          error={errors.password?.message}
        />

        <div className='mt-2 flex justify-end gap-2'>
          <Button type='button' variant='ghost' onClick={handleClose}>
            {t('common.cancel')}
          </Button>
          <Button type='submit' variant='primary' disabled={isSubmitting}>
            {t('audit.access.submit')}
          </Button>
        </div>
      </form>
    </Modal>
  )
}

export default AuditAccessModal
