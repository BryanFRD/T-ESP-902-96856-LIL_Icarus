import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { User } from '@/domain/types/user.ts'
import { patchMe } from '@/service/auth-service.ts'

import { useMe } from './use-me.ts'

export const usePatchMe = () => {
  const queryClient = useQueryClient()
  const { data: me } = useMe()

  return useMutation({
    mutationFn: patchMe,
    onSuccess: (_, variables: Partial<User>) => {
      queryClient.setQueryData(['user', me?.id], (oldData: User | undefined) =>
        oldData ? { ...oldData, ...variables } : oldData
      )
      queryClient.setQueryData(['users'], (oldData: User[] | undefined) =>
        oldData?.map(user =>
          user.id === me?.id ? { ...user, ...variables } : user
        )
      )
      queryClient.setQueryData(['user', 'me'], (oldData: User | undefined) =>
        oldData ? { ...oldData, ...variables } : oldData
      )
      queryClient.invalidateQueries({ queryKey: ['users'] })
      queryClient.invalidateQueries({ queryKey: ['user', me?.id] })
      queryClient.invalidateQueries({ queryKey: ['user', 'me'] })
    }
  })
}
