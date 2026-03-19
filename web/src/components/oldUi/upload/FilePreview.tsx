import { t } from 'i18next'
import { Camera, X } from 'lucide-react'

import {
  formatFileSize,
  getFileIcon,
  getPreview
} from '@/utils/file-upload-utils.tsx'

import Button from '../button/Button.tsx'

export const AvatarPreview = ({
  files,
  removeFile,
  cropping = false,
  defaultPictureUrl
}: {
  files: File[]
  removeFile: (index: number) => void
  cropping?: boolean
  defaultPictureUrl?: string
}) => {
  const file = files[0]
  const preview = file ? getPreview(file) : null

  return (
    <div className='relative h-32 w-32'>
      <div className='h-32 w-32 overflow-hidden rounded-full border-2 border-gray-200 bg-gray-100 dark:border-gray-600 dark:bg-gray-700'>
        {(() => {
          if (preview) {
            return (
              <img
                src={preview}
                alt={t('ui:fileUpload.avatar.preview')}
                className='h-full w-full object-cover'
              />
            )
          } else if (defaultPictureUrl) {
            return (
              <img
                src={defaultPictureUrl}
                alt={t('ui:fileUpload.avatar.default')}
                className='h-full w-full object-cover'
              />
            )
          } else {
            return (
              <div className='flex h-full w-full items-center justify-center'>
                <Camera className='h-8 w-8 text-gray-400 dark:text-gray-500' />
              </div>
            )
          }
        })()}
      </div>
      {files.length > 0 && !cropping && (
        <Button
          onClick={() => removeFile(0)}
          variant='danger'
          className='absolute -top-2 -right-2 flex h-6 w-6 items-center justify-center rounded-full'
        >
          <X className='absolute h-3 w-3' />
        </Button>
      )}
    </div>
  )
}

export const ChatFiles = ({
  files,
  removeFile
}: {
  files: File[]
  removeFile: (index: number) => void
}) => {
  if (files.length === 0) return null

  return (
    <div className='mt-2 space-y-1'>
      {files.map((file, index) => {
        const preview = getPreview(file)
        return (
          <div
            key={file.name}
            className='flex items-center space-x-2 rounded-md bg-gray-50 p-2 text-sm dark:bg-gray-700'
          >
            {preview ? (
              <img
                src={preview}
                alt={file.name}
                className='h-8 w-8 flex-shrink-0 rounded object-cover'
              />
            ) : (
              getFileIcon(file)
            )}
            <span className='flex-1 truncate text-gray-700 dark:text-gray-300'>
              {file.name}
            </span>
            <Button onClick={() => removeFile(index)} variant='ghost'>
              <X className='h-4 w-4' />
            </Button>
          </div>
        )
      })}
    </div>
  )
}

export const GalleryFiles = ({
  files,
  removeFile
}: {
  files: File[]
  removeFile: (index: number) => void
}) => {
  if (files.length === 0) return null

  return (
    <div className='mt-4 grid grid-cols-3 gap-3 md:grid-cols-4 lg:grid-cols-6'>
      {files.map((file, index) => {
        const preview = getPreview(file)
        return (
          <div key={file.name} className='group relative aspect-square'>
            <div className='h-full w-full overflow-hidden rounded-lg border border-gray-200 bg-gray-100 dark:border-gray-600 dark:bg-gray-700'>
              {preview ? (
                <img
                  src={preview}
                  alt={file.name}
                  className='h-full w-full object-cover'
                />
              ) : (
                <div className='flex h-full w-full items-center justify-center'>
                  {getFileIcon(file)}
                </div>
              )}
            </div>

            <Button
              onClick={() => removeFile(index)}
              variant='danger'
              className='absolute -top-2 -right-2 flex h-6 w-6 items-center justify-center rounded-full opacity-0 transition-all group-hover:opacity-100'
            >
              <X className='absolute h-3 w-3' />
            </Button>
            <div className='absolute right-0 bottom-0 left-0 truncate bg-black/50 p-1 text-xs text-white opacity-0 transition-opacity group-hover:opacity-100'>
              {file.name}
            </div>
          </div>
        )
      })}
    </div>
  )
}

export const DefaultFiles = ({
  files,
  removeFile
}: {
  files: File[]
  removeFile: (index: number) => void
}) => {
  if (files.length === 0) return null

  return (
    <div className='mt-4 space-y-3'>
      <h4 className='text-sm font-medium text-gray-700 dark:text-gray-300'>
        {t('ui:fileUpload.selected_files')} ({files.length})
      </h4>
      <div className='space-y-2'>
        {files.map((file, index) => {
          const preview = getPreview(file)
          return (
            <div
              key={file.name}
              className='flex items-center justify-between rounded-lg border border-gray-200 bg-gray-50 p-3 dark:border-gray-600 dark:bg-gray-700'
            >
              <div className='flex min-w-0 flex-1 items-center space-x-3'>
                {preview ? (
                  <img
                    src={preview}
                    alt={file.name}
                    className='h-10 w-10 flex-shrink-0 rounded object-cover'
                  />
                ) : (
                  <div className='flex-shrink-0'>{getFileIcon(file)}</div>
                )}
                <div className='min-w-0 flex-1'>
                  <p className='truncate text-sm font-medium text-gray-700 dark:text-gray-300'>
                    {file.name}
                  </p>
                  <p className='text-xs text-gray-500 dark:text-gray-400'>
                    {formatFileSize(file.size)}
                  </p>
                </div>
              </div>
              <Button
                onClick={() => removeFile(index)}
                className='ml-2 flex-shrink-0 rounded-full p-2 transition-colors hover:bg-gray-200 dark:hover:bg-gray-600'
                variant='ghost'
              >
                <X className='h-4 w-4 text-gray-500 dark:text-gray-400' />
              </Button>
            </div>
          )
        })}
      </div>
    </div>
  )
}
