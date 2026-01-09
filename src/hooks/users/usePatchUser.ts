import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { User } from '@/domain/commonTypes/userTypes'
import { patchUser } from '@/services/userService'

import { useMe } from '../auths/useMe'

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
      queryClient.invalidateQueries({ queryKey: ['users'] })
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
      queryClient.setQueryData(['users'], (oldData: User[] | undefined) => {
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
