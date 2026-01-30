import { motion } from 'framer-motion'
import { t } from 'i18next'

import ChangePasswordForm from '@/components/profile/ChangePasswordForm.tsx'
import ImageCropModal from '@/components/profile/ImageCropModal.tsx'
import ProfileForm from '@/components/profile/ProfileForm.tsx'
import FileUpload from '@/components/ui/upload/FileUpload.tsx'
import { useUserContext } from '@/context/UserContext.tsx'
import { useAvatar } from '@/hooks/user/useAvatar.ts'
import { useDateFormatter } from '@/utils/dateFormatter.ts'

const ProfileScreen = () => {
  const {
    handleAvatarSelect,
    cropModalOpen,
    selectedImage,
    handleCropComplete,
    handleCropCancel,
    currentAvatarUrl,
    fileUploadKey
  } = useAvatar()

  const { user } = useUserContext()
  const { dateLongTimeNative, dateLong } = useDateFormatter()

  function displayCreatedAt() {
    if (!user?.createdAt) return t('profileScreen.noCreationDate')
    return dateLong(user.createdAt)
  }

  function displayLastUpdated() {
    if (!user?.updatedAt) return t('profileScreen.noUpdateDate')
    return dateLongTimeNative(user.updatedAt)
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.4 }}
      className='mx-auto max-w-5xl px-4 py-8'
    >
      <div className='grid grid-cols-1 gap-8 lg:grid-cols-3'>
        <div className='lg:col-span-1'>
          <div className='rounded-lg border border-gray-100 bg-white p-6 dark:border-gray-700 dark:bg-gray-800'>
            <div className='flex flex-col items-center'>
              <FileUpload
                key={fileUploadKey}
                variant='avatar'
                avatarDefaultPictureUrl={currentAvatarUrl}
                cropping={true}
                onFileSelect={handleAvatarSelect}
                className='mb-6'
              />

              <div className='w-full'>
                <h3 className='mb-3 text-lg font-semibold text-gray-800 dark:text-gray-200'>
                  {t('profileScreen.profileDetails')}
                </h3>

                <div className='space-y-4'>
                  <div>
                    <p className='text-sm text-gray-500 dark:text-gray-400'>
                      {t('profileScreen.createdOn')}
                    </p>
                    <p className='font-medium text-gray-700 dark:text-gray-300'>
                      {displayCreatedAt()}
                    </p>
                  </div>

                  <div>
                    <p className='text-sm text-gray-500 dark:text-gray-400'>
                      {t('profileScreen.lastUpdated')}
                    </p>
                    <p className='font-medium text-gray-700 dark:text-gray-300'>
                      {displayLastUpdated()}
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div className='lg:col-span-2'>
          <div className='rounded-lg border border-gray-100 bg-white p-6 dark:border-gray-700 dark:bg-gray-800'>
            <ProfileForm />
          </div>
          <div className='mt-8 rounded-lg border border-gray-100 bg-white p-6 dark:border-gray-700 dark:bg-gray-800'>
            <ChangePasswordForm />
          </div>
        </div>
      </div>

      {selectedImage && (
        <ImageCropModal
          isOpen={cropModalOpen}
          onClose={handleCropCancel}
          imageSrc={selectedImage.src}
          fileName={selectedImage.file.name}
          onCropComplete={handleCropComplete}
        />
      )}
    </motion.div>
  )
}

export default ProfileScreen
