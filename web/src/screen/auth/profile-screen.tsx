import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'

import { PasswordForm } from '@/components/profile/password-form'
import { ProfileForm } from '@/components/profile/profile-form'
import { ImageCropper } from '@/components/ui/image-cropper'
import { useUserContext } from '@/context/user-context.tsx'
import { useAvatarUpload } from '@/hooks/user/use-avatar-upload.ts'
import { useGetAvatar } from '@/hooks/user/use-get-avatar.ts'
import type { ApiError } from '@/utils/api-utils.ts'
import { useDateFormatter } from '@/utils/date-formatter.ts'

const ProfileScreen = () => {
  const { t } = useTranslation()
  const { user } = useUserContext()
  const { mutateAsync: uploadAvatar } = useAvatarUpload()
  const { data: avatarData } = useGetAvatar(user?.avatarUrl)
  const { dateLong } = useDateFormatter()

  const [localAvatarUrl, setLocalAvatarUrl] = useState<string | null>(null)

  const initials = [user?.firstName?.[0], user?.lastName?.[0]]
    .filter(Boolean)
    .join('')
    .toUpperCase()

  const currentAvatarUrl = localAvatarUrl ?? avatarData ?? undefined

  const handleAvatarChange = async (file: File) => {
    setLocalAvatarUrl(URL.createObjectURL(file))
    await toast.promise(uploadAvatar({ avatar: file }), {
      loading: t('messages:loading.uploadingAvatar'),
      success: t('messages:success.avatarUploaded'),
      error: (err: ApiError) => t(err.errorKey)
    })
  }

  return (
    <div className='space-y-8 p-6'>
      <div className='flex items-center gap-x-6'>
        <ImageCropper
          value={currentAvatarUrl}
          fallback={initials}
          onChange={handleAvatarChange}
        />

        <div>
          <h1 className='text-2xl font-bold'>
            {user?.firstName} {user?.lastName}
          </h1>
          <p className='text-muted-foreground'>{user?.email}</p>
          {user?.createdAt && (
            <p className='text-muted-foreground text-sm'>
              {dateLong(user.createdAt)}
            </p>
          )}
        </div>
      </div>

      <div className='grid grid-cols-1 gap-6 lg:grid-cols-[3fr_2fr]'>
        <ProfileForm />
        <PasswordForm />
      </div>
    </div>
  )
}

export default ProfileScreen
