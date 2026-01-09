import { useMutation, useQueryClient } from '@tanstack/react-query'

import { logout } from '@/services/authService.ts'
import { getErrorMessage } from '@/utils/apiUtils.ts'

export const useLogout = () => {
  const queryClient = useQueryClient()

  return useMutation<boolean, Error>({
    mutationFn: logout,
    onError: error => {
      return getErrorMessage(error, 'error.logout.failed')
    },
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['user', 'me'] })
    }
  })
}
