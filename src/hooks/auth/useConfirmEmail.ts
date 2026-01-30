import { useMutation } from '@tanstack/react-query'

import { confirmEmail } from '@/service/authService.ts'
import { getErrorMessage } from '@/utils/apiUtils.ts'

export const useConfirmEmail = () => {
  return useMutation<boolean, Error, string>({
    mutationFn: confirmEmail,
    onError: error => {
      getErrorMessage(error)
    },
    onSuccess: () => {}
  })
}
