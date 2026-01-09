import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { UploadAvatarFile } from '@/domain/commonTypes/uploadFileType'
import { uploadAvatar } from '@/services/userService'
import { getErrorMessage } from '@/utils/apiUtils'

export const useAvatarUpload = () => {
  const queryClient = useQueryClient()

  return useMutation<string, Error, UploadAvatarFile>({
    mutationFn: uploadAvatar,
    onError: error => getErrorMessage(error, 'error.upload.avatar.failed'),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['user', 'me']
      })
    }
  })
}
