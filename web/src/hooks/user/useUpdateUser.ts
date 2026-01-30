import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { User } from '@/domain/types/User.ts'
import { useMe } from '@/hooks/auth/useMe'
import { updateUser } from '@/service/userService'

export const useUpdateUser = () => {
  const queryClient = useQueryClient()
  const { data: me } = useMe()

  return useMutation({
    mutationFn: updateUser,
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: ['user'] })
      queryClient.invalidateQueries({ queryKey: ['user', id] })
      queryClient.setQueryData(['user', id], (oldData: User) => {
        if (oldData) {
          return {
            ...oldData,
            updated: true
          }
        }
        return oldData
      })
      queryClient.setQueryData(['user'], (oldData: User[]) => {
        if (oldData) {
          return oldData.map(user => {
            if (user.id === id) {
              return {
                ...user,
                updated: true
              }
            }
            return user
          })
        }
        return oldData
      })
      if (me?.id === id) {
        queryClient.invalidateQueries({ queryKey: ['user', 'me'] })
        queryClient.setQueryData(
          ['user', 'me'],
          (oldData: User | undefined) => {
            if (oldData) {
              return {
                ...oldData,
                updated: true
              }
            }
            return oldData
          }
        )
      }
    }
  })
}
