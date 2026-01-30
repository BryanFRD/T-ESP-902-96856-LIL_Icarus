import { useState } from 'react'
import { type FileRejection, useDropzone } from 'react-dropzone'

import type { FileUploadVariant } from '@/types/components/upload.type.ts'
import { processErrors } from '@/utils/fileUploadUtils.tsx'

export const useFileUpload = (
  config: FileUploadVariant,
  onFileSelect: (files: File[]) => void,
  disabled: boolean
) => {
  const [files, setFiles] = useState<File[]>([])
  const [errors, setErrors] = useState<string[]>([])

  const onDrop = (acceptedFiles: File[], rejectedFiles: FileRejection[]) => {
    const newErrors = processErrors(
      rejectedFiles,
      config.maxSize,
      config.maxFiles,
      config.allowedTypes
    )
    setErrors(newErrors)

    if (acceptedFiles.length > 0) {
      const finalFiles = config.multiple
        ? [...files, ...acceptedFiles].slice(0, config.maxFiles)
        : [acceptedFiles[0]]

      setFiles(finalFiles)
      onFileSelect(finalFiles)

      if (newErrors.length === 0) {
        setErrors([])
      }
    }
  }

  const removeFile = (index: number) => {
    const newFiles = files.filter((_, i) => i !== index)
    setFiles(newFiles)
    onFileSelect(newFiles)
    setErrors([])
  }

  const dropzoneProps = useDropzone({
    onDrop,
    accept:
      config.allowedTypes.length > 0
        ? Object.fromEntries(config.allowedTypes.map(type => [type, []]))
        : undefined,
    multiple: config.multiple,
    maxFiles: config.maxFiles,
    maxSize: config.maxSize * 1024 * 1024,
    disabled: disabled || (!config.multiple && files.length >= 1)
  })

  return {
    files,
    errors,
    removeFile,
    ...dropzoneProps
  }
}
