import { X } from 'lucide-react'
import { type ReactNode, useEffect } from 'react'
import { twMerge } from 'tailwind-merge'

import useInteractionOutside from '@/hooks/common/use-interaction-outside.ts'

import Button from '../button/Button.tsx'

interface ModalProps {
  isOpen: boolean
  onClose?: () => void
  title: string
  children: ReactNode
  size?: 'sm' | 'md' | 'lg' | 'xl'
  footer?: ReactNode
  className?: string
  closeButtonLabel?: string
  disableOutsideClick?: boolean
  disableEscapeKey?: boolean
}

const sizeMap = {
  sm: 'max-w-sm',
  md: 'max-w-md',
  lg: 'max-w-lg',
  xl: 'max-w-xl'
}

const Modal = ({
  isOpen,
  onClose = () => {},
  title,
  children,
  size = 'md',
  footer,
  className = '',
  closeButtonLabel = 'Fermer',
  disableOutsideClick = false,
  disableEscapeKey = false
}: ModalProps) => {
  const { ref } = useInteractionOutside<HTMLDivElement>(
    disableOutsideClick ? () => {} : onClose,
    {
      enabled: isOpen && !disableOutsideClick,
      eventType: 'click'
    }
  )

  useEffect(() => {
    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === 'Escape' && !disableEscapeKey) {
        onClose()
      }
    }

    if (isOpen) {
      document.body.style.overflow = 'hidden'
      document.addEventListener('keydown', handleEscape)
    }

    return () => {
      document.body.style.overflow = 'auto'
      document.removeEventListener('keydown', handleEscape)
    }
  }, [isOpen, onClose, disableEscapeKey])

  if (!isOpen) return null

  return (
    <div
      className='fixed inset-0 z-50 flex items-center justify-center bg-black/40 p-4 backdrop-blur-sm dark:bg-black/60'
      role='dialog'
      aria-modal='true'
      aria-labelledby='modal-title'
    >
      <div
        ref={ref}
        className={twMerge(
          'flex max-h-[90vh] w-full flex-col rounded-xl bg-white shadow-xl dark:bg-gray-900',
          sizeMap[size],
          className
        )}
        tabIndex={-1}
      >
        <div className='flex-shrink-0 rounded-t-xl border-b border-gray-200 bg-white px-6 py-4 dark:border-gray-700 dark:bg-gray-900'>
          <div className='flex items-center justify-between'>
            <h2
              id='modal-title'
              className='pr-8 text-lg font-bold text-gray-900 dark:text-gray-100'
            >
              {title}
            </h2>
            <Button
              onClick={onClose}
              variant='ghost'
              size='sm'
              type='button'
              aria-label={closeButtonLabel}
              className='flex-shrink-0'
            >
              <X size={16} aria-hidden='true' />
            </Button>
          </div>
        </div>

        <div className='min-h-0 flex-1 overflow-y-auto px-6 py-4'>
          {children}
        </div>

        {footer && (
          <div className='flex-shrink-0 rounded-b-xl border-t border-gray-200 bg-white px-6 py-4 dark:border-gray-700 dark:bg-gray-900'>
            {footer}
          </div>
        )}
      </div>
    </div>
  )
}

export default Modal
