import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { Company, PatchCompanyPayload } from '@/domain/types/Team.ts'
import { patchCompany } from '@/service/teamService'

export const usePatchCompany = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: patchCompany,
    onSuccess: (
      updatedCompany: Company,
      variables: {
        teamId: string
        companyId: string
        payload: PatchCompanyPayload
      }
    ) => {
      const { teamId, companyId, payload } = variables
      queryClient.invalidateQueries({ queryKey: ['team', teamId, 'companies'] })
      queryClient.setQueryData(
        ['team', teamId, 'companies'],
        (oldData: Company[] | undefined) => {
          if (oldData) {
            return oldData.map(company =>
              company.id === companyId
                ? { ...company, ...payload, ...updatedCompany }
                : company
            )
          }
          return oldData
        }
      )
    }
  })
}
