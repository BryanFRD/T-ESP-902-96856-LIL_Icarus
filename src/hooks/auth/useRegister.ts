import { useMutation } from '@tanstack/react-query'

import type { RegisterPayload } from '@/domain/types/Authentication.ts'
import { register } from '@/service/authService'
import { getErrorMessage } from '@/utils/apiUtils.ts'

export const useRegister = () => {
  return useMutation<void, Error, RegisterPayload>({
    mutationFn: register,
    onError: error => getErrorMessage(error, 'error.register.failed')
  })
}
