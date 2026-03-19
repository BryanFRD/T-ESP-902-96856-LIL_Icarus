import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { User } from '@/domain/types/user.ts'
import { createUser } from '@/service/user-service.ts'

export const useCreateUser = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: createUser,
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: ['user'] })
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
      queryClient.setQueryData(['user'], (oldData: User[]) => {
        if (oldData) {
          return [...oldData, { id, created: true }]
        }
        return oldData
      })
    }
  })
}
