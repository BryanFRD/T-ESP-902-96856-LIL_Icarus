import { useMutation } from '@tanstack/react-query'

import type { RegisterPayload } from '@/domain/types/authentication.ts'
import { register } from '@/service/auth-service.ts'
import { getErrorMessage } from '@/utils/api-utils.ts'

export const useRegister = () => {
  return useMutation<void, Error, RegisterPayload>({
    mutationFn: register,
    onError: error => getErrorMessage(error, 'error.register.failed')
  })
}
