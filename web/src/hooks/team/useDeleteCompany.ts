import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { Company } from '@/domain/types/Team.ts'
import { deleteCompany } from '@/service/teamService'

export const useDeleteCompany = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: deleteCompany,
    onSuccess: (_, variables: { teamId: string; companyId: string }) => {
      const { teamId, companyId } = variables
      queryClient.invalidateQueries({ queryKey: ['team', teamId, 'companies'] })
      queryClient.setQueryData(
        ['team', teamId, 'companies'],
        (oldData: Company[] | undefined) => {
          if (oldData) {
            return oldData.filter(company => company.id !== companyId)
          }
          return oldData
        }
      )
    }
  })
}
