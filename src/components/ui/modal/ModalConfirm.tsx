import { AlertTriangle } from 'lucide-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import Button from '@/components/ui/button/Button'
import Modal from '@/components/ui/modal/Modal'

type ModalConfirmProps = {
  isOpen: boolean
  onClose: () => void
  onConfirm: () => void | Promise<void>
  title: string
  description: string
  confirmText: string
  confirmMatch: string
  confirmLabel?: string
  isLoading?: boolean
  variant?: 'danger' | 'warning' | 'primary'
}

const ModalConfirm = ({
  isOpen,
  onClose,
  onConfirm,
  title,
  description,
  confirmText,
  confirmMatch,
  confirmLabel,
  isLoading = false,
  variant = 'danger'
}: ModalConfirmProps) => {
  const { t } = useTranslation()
  const [inputValue, setInputValue] = useState('')

  const handleConfirm = async () => {
    if (inputValue === confirmMatch) {
      await onConfirm()
      setInputValue('')
      onClose()
    }
  }

  const handleClose = () => {
    setInputValue('')
    onClose()
  }

  const isConfirmDisabled = inputValue !== confirmMatch

  const getVariantStyles = () => {
    switch (variant) {
      case 'danger':
        return 'text-red-600 dark:text-red-400'
      case 'warning':
        return 'text-orange-600 dark:text-orange-400'
      case 'primary':
        return 'text-orange-600 dark:text-orange-400'
    }
  }

  const getButtonVariant = (): 'danger' | 'primary' => {
    switch (variant) {
      case 'danger':
      case 'warning':
        return 'danger'
      case 'primary':
        return 'primary'
    }
  }

  return (
    <Modal isOpen={isOpen} onClose={handleClose} title={title} size='md'>
      <div className='space-y-4'>
        <div className='flex items-center gap-3'>
          <div
            className={`flex h-12 w-12 items-center justify-center rounded-full ${
              variant === 'danger'
                ? 'bg-red-100 dark:bg-red-900/20'
                : 'bg-orange-100 dark:bg-orange-900/20'
            }`}
          >
            <AlertTriangle className={`h-6 w-6 ${getVariantStyles()}`} />
          </div>
          <div className='flex-1'>
            <p className='text-sm text-gray-600 dark:text-gray-400'>
              {description}
            </p>
          </div>
        </div>

        <div className='rounded-lg border-2 border-gray-200 bg-gray-50 p-4 dark:border-gray-700 dark:bg-gray-800/50'>
          <p className='mb-3 text-sm font-medium text-gray-700 dark:text-gray-300'>
            {confirmLabel || t('common:confirmAction.typeToConfirm')}
          </p>
          <p className='mb-3 text-sm text-gray-600 dark:text-gray-400'>
            {t('common:confirmAction.pleaseType')}{' '}
            <span className='font-mono font-semibold text-gray-900 dark:text-gray-100'>
              {confirmMatch}
            </span>{' '}
            {t('common:confirmAction.toContinue')}
          </p>
          <input
            type='text'
            value={inputValue}
            onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
              setInputValue(e.target.value)
            }
            placeholder={confirmMatch}
            autoFocus
            disabled={isLoading}
            className='w-full rounded-md border-2 border-gray-200 bg-white px-3 py-2 text-sm text-gray-900 transition-all duration-200 outline-none placeholder:text-gray-400 focus:border-orange-500 disabled:cursor-not-allowed disabled:bg-gray-50 disabled:opacity-50 dark:border-gray-700 dark:bg-gray-900 dark:text-gray-100 dark:placeholder:text-gray-500 dark:focus:border-orange-500 dark:disabled:bg-gray-800'
          />
        </div>

        <div className='flex justify-end gap-3 pt-2'>
          <Button
            type='button'
            variant='ghost'
            onClick={handleClose}
            disabled={isLoading}
          >
            {t('common:cancel')}
          </Button>
          <Button
            type='button'
            variant={getButtonVariant()}
            onClick={handleConfirm}
            disabled={isConfirmDisabled || isLoading}
            isLoading={isLoading}
          >
            {confirmText}
          </Button>
        </div>
      </div>
    </Modal>
  )
}

export default ModalConfirm
