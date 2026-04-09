import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import type { Company } from '@/domain/types/team.ts'
import * as teamService from '@/service/team-service.ts'

vi.mock('@/service/team-service.ts', () => ({
  getTeamCompanies: vi.fn()
}))

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false }, mutations: { retry: false } }
  })
  function Wrapper({ children }: { children: ReactNode }) {
    return (
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    )
  }
  return { Wrapper, queryClient }
}

const mockCompanies: Company[] = [
  {
    id: 'comp-1',
    teamId: 'team-1',
    name: 'Acme Corp',
    siret: '12345678901234',
    createdAt: '2024-01-01',
    updatedAt: '2024-01-01'
  }
]

describe('useTeamCompany', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should fetch companies for a given team', async () => {
    vi.mocked(teamService.getTeamCompanies).mockResolvedValue(mockCompanies)
    const { Wrapper } = createWrapper()

    const { useTeamCompany } = await import('../use-team-company.ts')
    const { result } = renderHook(() => useTeamCompany('team-1'), {
      wrapper: Wrapper
    })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(result.current.data).toEqual(mockCompanies)
    expect(teamService.getTeamCompanies).toHaveBeenCalledWith('team-1')
  })

  it('should not fetch when teamId is empty', async () => {
    const { Wrapper } = createWrapper()

    const { useTeamCompany } = await import('../use-team-company.ts')
    const { result } = renderHook(() => useTeamCompany(''), {
      wrapper: Wrapper
    })

    await waitFor(() => expect(result.current.fetchStatus).toBe('idle'))
    expect(teamService.getTeamCompanies).not.toHaveBeenCalled()
  })

  it('should set error state when the fetch fails', async () => {
    vi.mocked(teamService.getTeamCompanies).mockRejectedValue(
      new Error('Network error')
    )
    const { Wrapper } = createWrapper()

    const { useTeamCompany } = await import('../use-team-company.ts')
    const { result } = renderHook(() => useTeamCompany('team-1'), {
      wrapper: Wrapper
    })

    await waitFor(() => expect(result.current.isError).toBe(true))
    expect(result.current.error?.message).toBe('Network error')
  })
})
