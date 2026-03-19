import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { LoginPayload } from '@/domain/types/authentication.ts'
import { login } from '@/service/auth-service.ts'
import { getErrorMessage } from '@/utils/api-utils.ts'

export const useLogin = () => {
  const queryClient = useQueryClient()

  return useMutation<void, Error, LoginPayload>({
    mutationFn: login,
    onError: error => {
      return getErrorMessage(error, 'error.login.failed')
    },
    onSuccess: async () => {
      await queryClient.invalidateQueries({
        queryKey: ['user', 'me']
      })
    }
  })
}
