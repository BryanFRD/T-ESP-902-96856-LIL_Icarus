import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { User } from '@/domain/commonTypes/userTypes'
import { patchMe } from '@/services/authService'

import { useMe } from '../auths/useMe'

export const usePatchMe = () => {
  const queryClient = useQueryClient()
  const { data: me } = useMe()

  return useMutation({
    mutationFn: patchMe,
    onSuccess: (updatedUser: User, variables: Partial<User>) => {
      const payload = variables
      queryClient.invalidateQueries({ queryKey: ['users'] })
      queryClient.invalidateQueries({ queryKey: ['user', me?.id] })
      queryClient.setQueryData(
        ['user', me?.id],
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
      queryClient.setQueryData(['users'], (oldData: User[] | undefined) => {
        if (oldData) {
          return oldData.map(user =>
            user.id === me?.id ? { ...user, ...payload, ...updatedUser } : user
          )
        }
        return oldData
      })
      queryClient.invalidateQueries({ queryKey: ['user', 'me'] })
      queryClient.setQueryData(['user', 'me'], (oldData: User | undefined) => {
        if (oldData) {
          return {
            ...oldData,
            ...payload,
            ...updatedUser
          }
        }
        return updatedUser
      })
    }
  })
}
