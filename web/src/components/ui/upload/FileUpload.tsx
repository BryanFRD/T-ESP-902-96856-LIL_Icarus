import { t } from 'i18next'
import { Camera, FileText, ImageIcon, Paperclip, Upload } from 'lucide-react'
import { twMerge } from 'tailwind-merge'

import { useFileUpload } from '@/hooks/useFileUpload'
import type {
  FileUploadProps,
  FileUploadVariant,
  LayoutType
} from '@/types/components/upload.type'

import { ErrorUploadFile } from './ErrorUploadFile'
import {
  AvatarPreview,
  ChatFiles,
  DefaultFiles,
  GalleryFiles
} from './FilePreview'

const getDropzoneClasses = (
  isDragActive: boolean,
  isDragReject: boolean,
  isDragAccept: boolean,
  disabled: boolean,
  layout: LayoutType
): string => {
  const base = 'cursor-pointer transition-all duration-200 ease-in-out'
  const disabledStyle = disabled ? 'cursor-not-allowed opacity-50' : ''

  let stateClasses = ''
  if (isDragReject) {
    stateClasses = 'border-red-500 bg-red-50 dark:bg-red-900/20'
  } else if (isDragAccept) {
    stateClasses = 'border-green-500 bg-green-50 dark:bg-green-900/20'
  } else if (isDragActive) {
    stateClasses = 'border-blue-500 bg-blue-50 dark:bg-blue-900/20'
  } else {
    stateClasses =
      'border-gray-300 dark:border-gray-600 hover:border-gray-400 dark:hover:border-gray-500 bg-white dark:bg-gray-800'
  }

  const layoutClasses = {
    compact: 'border border-dashed rounded-lg p-4 text-center',
    full: 'border-2 border-dashed rounded-lg p-8 text-center',
    grid: 'border-2 border-dashed rounded-lg p-6 text-center',
    circle:
      'border-2 border-dashed rounded-full w-32 h-32 flex flex-col items-center justify-center'
  }

  return twMerge(base, disabledStyle, stateClasses, layoutClasses[layout])
}

const getIconClasses = (
  isDragActive: boolean,
  isDragReject: boolean,
  isDragAccept: boolean,
  layout: LayoutType
): string => {
  let color = 'text-gray-400 dark:text-gray-500'
  if (isDragReject) color = 'text-red-400 dark:text-red-500'
  else if (isDragAccept) color = 'text-green-400 dark:text-green-500'
  else if (isDragActive) color = 'text-blue-400 dark:text-blue-500'

  return twMerge('mx-auto', layout === 'compact' ? 'mb-1' : 'mb-2', color)
}

const variants: Record<string, FileUploadVariant> = {
  avatar: {
    maxFiles: 1,
    maxSize: 2,
    allowedTypes: ['image/jpeg', 'image/png', 'image/webp'],
    multiple: false,
    layout: 'circle',
    placeholder: 'file_upload.avatar.placeholder',
    icon: <Camera className='h-6 w-6' />
  },
  chat: {
    maxFiles: 5,
    maxSize: 10,
    allowedTypes: [
      'image/jpeg',
      'image/png',
      'image/gif',
      'application/pdf',
      'text/plain'
    ],
    multiple: true,
    layout: 'compact',
    placeholder: t('file_upload.chat.placeholder'),
    icon: <Paperclip className='h-4 w-4' />
  },
  document: {
    maxFiles: 10,
    maxSize: 50,
    allowedTypes: [
      'application/pdf',
      'application/msword',
      'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
      'text/plain'
    ],
    multiple: true,
    layout: 'full',
    placeholder: t('file_upload.document.placeholder'),
    icon: <FileText className='h-8 w-8' />
  },
  gallery: {
    maxFiles: 20,
    maxSize: 10,
    allowedTypes: ['image/jpeg', 'image/png', 'image/gif', 'image/webp'],
    multiple: true,
    layout: 'grid',
    placeholder: t('file_upload.gallery.placeholder'),
    icon: <ImageIcon className='h-8 w-8' />
  },
  any: {
    maxFiles: 10,
    maxSize: 25,
    allowedTypes: [],
    multiple: true,
    layout: 'full',
    placeholder: t('file_upload.any.placeholder'),
    icon: <Upload className='h-8 w-8' />
  }
}

const FileUpload = ({
  onFileSelect,
  variant = 'any',
  avatarDefaultPictureUrl = '',
  cropping = false,
  className,
  disabled = false
}: FileUploadProps) => {
  const config = variants[variant]
  const {
    files,
    errors,
    removeFile,
    getRootProps,
    getInputProps,
    isDragActive,
    isDragReject,
    isDragAccept
  } = useFileUpload(config, onFileSelect, disabled)

  if (variant === 'avatar') {
    return (
      <div className={twMerge('w-full', className)}>
        <div
          {...getRootProps()}
          className='inline-block cursor-pointer'
          aria-label={t('file_upload.change_avatar')}
        >
          <input {...getInputProps()} />
          <AvatarPreview
            files={files}
            cropping={cropping}
            removeFile={removeFile}
            defaultPictureUrl={avatarDefaultPictureUrl}
          />
        </div>
        <ErrorUploadFile errors={errors} compact />
      </div>
    )
  }

  return (
    <div className={twMerge('w-full', className)}>
      <div
        {...getRootProps()}
        className={getDropzoneClasses(
          isDragActive,
          isDragReject,
          isDragAccept,
          disabled || (!config.multiple && files.length >= 1),
          config.layout
        )}
      >
        <input {...getInputProps()} />

        <div
          className={getIconClasses(
            isDragActive,
            isDragReject,
            isDragAccept,
            config.layout
          )}
        >
          {config.icon}
        </div>

        <div
          className={config.layout === 'compact' ? 'space-y-1' : 'space-y-2'}
        >
          {isDragActive ? (
            (() => {
              let dragColorClass = ''
              if (isDragReject) {
                dragColorClass = 'text-red-600 dark:text-red-400'
              } else if (isDragAccept) {
                dragColorClass = 'text-green-600 dark:text-green-400'
              } else {
                dragColorClass = 'text-blue-600 dark:text-blue-400'
              }
              return (
                <p
                  className={twMerge(
                    'font-medium',
                    config.layout === 'compact' ? 'text-sm' : 'text-base',
                    dragColorClass
                  )}
                >
                  {isDragReject
                    ? t('file_upload.invalid_file')
                    : t('file_upload.drop_files')}
                </p>
              )
            })()
          ) : (
            <>
              <p
                className={twMerge(
                  'font-medium text-gray-600 dark:text-gray-300',
                  config.layout === 'compact' ? 'text-sm' : 'text-base'
                )}
              >
                {!config.multiple && files.length >= 1
                  ? t('file_upload.file_selected')
                  : t(config.placeholder)}
              </p>
              {config.layout !== 'compact' && config.layout !== 'circle' && (
                <p className='text-sm text-gray-500 dark:text-gray-400'>
                  {config.multiple &&
                    `${t('file_upload.max_files')} ${config.maxFiles} • `}
                  {t('file_upload.max_size')} {config.maxSize}MB
                </p>
              )}
            </>
          )}
        </div>
      </div>

      <ErrorUploadFile errors={errors} />

      {variant === 'chat' && (
        <ChatFiles files={files} removeFile={removeFile} />
      )}
      {variant === 'gallery' && (
        <GalleryFiles files={files} removeFile={removeFile} />
      )}
      {!['avatar', 'chat', 'gallery'].includes(variant) && (
        <DefaultFiles files={files} removeFile={removeFile} />
      )}
    </div>
  )
}

export default FileUpload
