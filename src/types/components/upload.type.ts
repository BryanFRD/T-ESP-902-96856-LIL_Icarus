import type { ReactNode } from 'react'

export interface FileUploadProps {
  onFileSelect: (files: File[]) => void
  variant?: 'avatar' | 'chat' | 'document' | 'gallery' | 'any'
  avatarDefaultPictureUrl?: string
  cropping?: boolean
  className?: string
  disabled?: boolean
}

export type FileUploadVariant = {
  maxFiles: number
  maxSize: number
  allowedTypes: string[]
  multiple: boolean
  layout: 'compact' | 'full' | 'grid' | 'circle'
  placeholder: string
  icon: ReactNode
}

export type LayoutType = FileUploadVariant['layout']
