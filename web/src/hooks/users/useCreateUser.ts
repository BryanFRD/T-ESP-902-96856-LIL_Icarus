import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { User } from '@/domain/commonTypes/userTypes'
import { createUser } from '@/services/userService'

export const useCreateUser = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: createUser,
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: ['users'] })
      queryClient.invalidateQueries({ queryKey: ['user', id] })
      queryClient.setQueryData(['user', id], (oldData: User) => {
        if (oldData) {
          return {
            ...oldData,
            created: true
          }
        }
        return oldData
      })
      queryClient.setQueryData(['users'], (oldData: User[]) => {
        if (oldData) {
          return [...oldData, { id, created: true }]
        }
        return oldData
      })
    }
  })
}
