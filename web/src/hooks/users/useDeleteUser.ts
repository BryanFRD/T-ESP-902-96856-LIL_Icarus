import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { User } from '@/domain/commonTypes/userTypes'
import { deleteUser } from '@/services/userService'

import { useMe } from '../auths/useMe'

export const useDeleteUser = () => {
  const queryClient = useQueryClient()
  const { data: me } = useMe()

  return useMutation({
    mutationFn: deleteUser,
    onSuccess: (_, id) => {
      queryClient.invalidateQueries({ queryKey: ['users'] })
      queryClient.invalidateQueries({ queryKey: ['user', id] })
      queryClient.setQueryData(['user', id], (oldData: User) => {
        if (oldData) {
          return {
            ...oldData,
            deleted: true
          }
        }
        return oldData
      })
      queryClient.setQueryData(['users'], (oldData: User[]) => {
        if (oldData) {
          return oldData.filter(user => user.id !== id)
        }
        return oldData
      })

      if (me?.id === id) {
        queryClient.invalidateQueries({ queryKey: ['user', 'me'] })
        queryClient.setQueryData(['user', 'me'], null)
      }
    }
  })
}
