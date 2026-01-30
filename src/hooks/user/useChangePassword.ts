import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { User } from '@/domain/types/User.ts'
import type { ChangePasswordPayload } from '@/service/authService'
import { changePassword } from '@/service/authService'

import { useMe } from '../auth/useMe'

export const useChangePassword = () => {
  const queryClient = useQueryClient()
  const { data: me } = useMe()

  return useMutation({
    mutationFn: (payload: ChangePasswordPayload) => changePassword(payload),
    onSuccess: () => {
      queryClient.setQueryData(
        ['user', me?.id],
        (oldData: User | undefined) => oldData
      )
      queryClient.setQueryData(
        ['users'],
        (oldData: User[] | undefined) => oldData
      )
      queryClient.setQueryData(
        ['user', 'me'],
        (oldData: User | undefined) => oldData
      )
      queryClient.invalidateQueries({ queryKey: ['users'] })
      queryClient.invalidateQueries({ queryKey: ['user', me?.id] })
      queryClient.invalidateQueries({ queryKey: ['user', 'me'] })
    }
  })
}
