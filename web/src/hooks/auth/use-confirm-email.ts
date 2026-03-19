import { useMutation } from '@tanstack/react-query'

import { confirmEmail } from '@/service/auth-service.ts'
import { getErrorMessage } from '@/utils/api-utils.ts'

export const useConfirmEmail = () => {
  return useMutation<boolean, Error, string>({
    mutationFn: confirmEmail,
    onError: error => {
      getErrorMessage(error)
    },
    onSuccess: () => {}
  })
}
