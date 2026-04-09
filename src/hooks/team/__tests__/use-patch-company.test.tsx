import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import type { Company } from '@/domain/types/team.ts'
import * as teamService from '@/service/team-service.ts'

vi.mock('@/service/team-service.ts', () => ({
  patchCompany: vi.fn()
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

const mockCompany: Company = {
  id: 'comp-1',
  teamId: 'team-1',
  name: 'Old Name',
  siret: '12345678901234',
  createdAt: '2024-01-01',
  updatedAt: '2024-01-01'
}

const updatedCompany: Company = {
  ...mockCompany,
  name: 'New Name',
  updatedAt: '2024-06-01'
}

describe('usePatchCompany', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should patch company and update cache', async () => {
    vi.mocked(teamService.patchCompany).mockResolvedValue(updatedCompany)
    const { Wrapper, queryClient } = createWrapper()
    queryClient.setQueryData(['team', 'team-1', 'companies'], [mockCompany])

    const invalidateSpy = vi.spyOn(queryClient, 'invalidateQueries')

    const { usePatchCompany } = await import('../use-patch-company.ts')
    const { result } = renderHook(() => usePatchCompany(), { wrapper: Wrapper })

    result.current.mutate({
      teamId: 'team-1',
      companyId: 'comp-1',
      payload: { name: 'New Name' }
    })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))

    expect(invalidateSpy).toHaveBeenCalledWith({
      queryKey: ['team', 'team-1', 'companies']
    })

    const cached = queryClient.getQueryData<Company[]>([
      'team',
      'team-1',
      'companies'
    ])
    expect(cached?.[0].name).toBe('New Name')
  })

  it('should not crash when cache is empty', async () => {
    vi.mocked(teamService.patchCompany).mockResolvedValue(updatedCompany)
    const { Wrapper } = createWrapper()

    const { usePatchCompany } = await import('../use-patch-company.ts')
    const { result } = renderHook(() => usePatchCompany(), { wrapper: Wrapper })

    result.current.mutate({
      teamId: 'team-1',
      companyId: 'comp-1',
      payload: { name: 'New Name' }
    })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))
  })

  it('should set error state on failure', async () => {
    vi.mocked(teamService.patchCompany).mockRejectedValue(
      new Error('Not found')
    )
    const { Wrapper } = createWrapper()

    const { usePatchCompany } = await import('../use-patch-company.ts')
    const { result } = renderHook(() => usePatchCompany(), { wrapper: Wrapper })

    result.current.mutate({
      teamId: 'team-1',
      companyId: 'comp-1',
      payload: { name: 'X' }
    })

    await waitFor(() => expect(result.current.isError).toBe(true))
    expect(result.current.error?.message).toBe('Not found')
  })
})
