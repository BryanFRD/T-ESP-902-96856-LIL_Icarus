import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { Company, CreateCompanyPayload } from '@/domain/types/Team.ts'
import { createCompany } from '@/service/teamService'

export const useCreateCompany = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: createCompany,
    onSuccess: (
      newCompany: Company,
      variables: { teamId: string; payload: CreateCompanyPayload }
    ) => {
      const { teamId } = variables
      queryClient.invalidateQueries({ queryKey: ['team', teamId, 'companies'] })
      queryClient.setQueryData(
        ['team', teamId, 'companies'],
        (oldData: Company[] | undefined) => {
          if (oldData) {
            return [...oldData, newCompany]
          }
          return [newCompany]
        }
      )
    }
  })
}
