import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { User } from '@/domain/types/User.ts'
import { useMe } from '@/hooks/auth/useMe'
import { patchUser } from '@/service/userService'

export const usePatchUser = () => {
  const queryClient = useQueryClient()
  const { data: me } = useMe()

  return useMutation({
    mutationFn: patchUser,
    onSuccess: (
      updatedUser: User,
      variables: { id: string; payload: Partial<User> }
    ) => {
      const { id, payload } = variables
      queryClient.invalidateQueries({ queryKey: ['user'] })
      queryClient.invalidateQueries({ queryKey: ['user', id] })
      queryClient.setQueryData(['user', id], (oldData: User | undefined) => {
        if (oldData) {
          return {
            ...oldData,
            ...payload,
            ...updatedUser
          }
        }
        return updatedUser
      })
      queryClient.setQueryData(['user'], (oldData: User[] | undefined) => {
        if (oldData) {
          return oldData.map(user =>
            user.id === id ? { ...user, ...payload, ...updatedUser } : user
          )
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
                ...payload,
                ...updatedUser
              }
            }
            return updatedUser
          }
        )
      }
    }
  })
}
