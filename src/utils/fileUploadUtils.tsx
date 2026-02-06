import { t } from 'i18next'
import { File, FileText, Image } from 'lucide-react'
import type { ReactNode } from 'react'
import type { FileRejection } from 'react-dropzone'

export const getFileIcon = (file: File): ReactNode => {
  if (file.type.startsWith('image/')) {
    return <Image className='h-5 w-5 text-blue-500 dark:text-blue-400' />
  }
  if (file.type.includes('pdf')) {
    return <FileText className='h-5 w-5 text-red-500 dark:text-red-400' />
  }
  return <File className='h-5 w-5 text-gray-500 dark:text-gray-400' />
}

export const getPreview = (file: File): string | null => {
  if (file.type.startsWith('image/')) {
    return URL.createObjectURL(file)
  }
  return null
}

export const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Number.parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}

export const processErrors = (
  rejectedFiles: FileRejection[],
  maxSize: number,
  maxFiles: number,
  allowedTypes: string[] = []
): string[] => {
  const newErrors: string[] = []

  for (const { file, errors } of rejectedFiles) {
    for (const error of errors) {
      if (error.code === 'file-too-large') {
        newErrors.push(
          t('ui:fileUpload.errors.too_large', {
            fileName: file.name,
            size: formatFileSize(maxSize * 1024 * 1024)
          })
        )
      } else if (error.code === 'file-invalid-type') {
        newErrors.push(
          t('ui:fileUpload.errors.invalid_type', {
            fileName: file.name,
            allowedTypes
          })
        )
      } else if (error.code === 'too-many-files') {
        newErrors.push(t('ui:fileUpload.errors.too_many_files', { maxFiles }))
      }
    }
  }

  return newErrors
}

export const getDisplaySize = (sizeInMB: number): string => {
  return ` (max ${sizeInMB}MB)`
}
