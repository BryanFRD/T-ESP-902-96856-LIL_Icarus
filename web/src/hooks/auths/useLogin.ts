import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { LoginPayload } from '@/domain/commonTypes/authType'
import { login } from '@/services/authService'
import { getErrorMessage } from '@/utils/apiUtils'

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
