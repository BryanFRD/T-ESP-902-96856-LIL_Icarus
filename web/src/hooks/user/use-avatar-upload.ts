import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { UploadAvatarFile } from '@/domain/types/upload-file.ts'
import { uploadAvatar } from '@/service/user-service.ts'
import { getErrorMessage } from '@/utils/api-utils.ts'

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
