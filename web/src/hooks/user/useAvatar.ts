import { useState } from 'react'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'

import { useUserContext } from '@/context/UserContext.tsx'
import { useAvatarUpload } from '@/hooks/user/useAvatarUpload.ts'
import { useGetAvatar } from '@/hooks/user/useGetAvatar.ts'
import type { ApiError } from '@/utils/apiUtils.ts'

import useToggle from '../common/useToggle.ts'

export const useAvatar = () => {
  const { t } = useTranslation()
  const { user } = useUserContext()
  const { mutateAsync: uploadAvatar } = useAvatarUpload()
  const { data: avatarData } = useGetAvatar(user?.avatarUrl)
  const { value: cropModalOpen, set: setCropModalOpen } = useToggle(false)
  const [fileUploadKey, setFileUploadKey] = useState(Date.now())
  const [selectedImage, setSelectedImage] = useState<{
    file: File
    src: string
  } | null>(null)
  const [localAvatarUrl, setLocalAvatarUrl] = useState<string | null>(null)

  const handleAvatarSelect = (files: File[]) => {
    if (files.length === 0 || !user) return
    const file = files[0]
    if (!file.type.startsWith('image/')) {
      toast.error(t('error.invalidImageType'))
      return
    }
    setSelectedImage({ file, src: URL.createObjectURL(file) })
    setCropModalOpen(true)
  }

  const handleCropComplete = async (croppedFile: File) => {
    setCropModalOpen(false)
    const previewUrl = URL.createObjectURL(croppedFile)
    setLocalAvatarUrl(previewUrl)
    setFileUploadKey(Date.now())
    await toast.promise(uploadAvatar({ avatar: croppedFile }), {
      loading: t('loading.uploadingAvatar'),
      success: t('success.avatarUploaded'),
      error: (err: ApiError) => t(err.errorKey)
    })
    if (selectedImage?.src) URL.revokeObjectURL(selectedImage.src)
    setSelectedImage(null)
  }

  const handleCropCancel = () => {
    setCropModalOpen(false)
    setFileUploadKey(Date.now())
    if (selectedImage?.src) URL.revokeObjectURL(selectedImage.src)
    setSelectedImage(null)
  }

  const currentAvatarUrl = localAvatarUrl ?? avatarData

  return {
    handleAvatarSelect,
    cropModalOpen,
    selectedImage,
    handleCropComplete,
    handleCropCancel,
    currentAvatarUrl,
    fileUploadKey
  }
}
