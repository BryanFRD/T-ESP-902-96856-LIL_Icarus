import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { UploadAvatarFile } from '@/domain/types/UploadFile.ts'
import { uploadAvatar } from '@/service/userService.ts'
import { getErrorMessage } from '@/utils/apiUtils.ts'

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
