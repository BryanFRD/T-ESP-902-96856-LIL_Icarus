import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import type { Company } from '@/domain/types/team.ts'
import * as teamService from '@/service/team-service.ts'

vi.mock('@/service/team-service.ts', () => ({
  createCompany: vi.fn()
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

const existingCompany: Company = {
  id: 'comp-1',
  teamId: 'team-1',
  name: 'Existing Corp',
  siret: '11111111111111',
  createdAt: '2024-01-01',
  updatedAt: '2024-01-01'
}

const newCompany: Company = {
  id: 'comp-2',
  teamId: 'team-1',
  name: 'New Corp',
  siret: '22222222222222',
  createdAt: '2024-06-01',
  updatedAt: '2024-06-01'
}

describe('useCreateCompany', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should create a company and append it to the cache', async () => {
    vi.mocked(teamService.createCompany).mockResolvedValue(newCompany)
    const { Wrapper, queryClient } = createWrapper()
    queryClient.setQueryData(['team', 'team-1', 'companies'], [existingCompany])

    const invalidateSpy = vi.spyOn(queryClient, 'invalidateQueries')

    const { useCreateCompany } = await import('../use-create-company.ts')
    const { result } = renderHook(() => useCreateCompany(), {
      wrapper: Wrapper
    })

    result.current.mutate({
      teamId: 'team-1',
      payload: { name: 'New Corp', siret: '22222222222222' }
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
    expect(cached).toHaveLength(2)
    expect(cached?.[1]).toEqual(newCompany)
  })

  it('should initialize cache when no existing data', async () => {
    vi.mocked(teamService.createCompany).mockResolvedValue(newCompany)
    const { Wrapper, queryClient } = createWrapper()

    const { useCreateCompany } = await import('../use-create-company.ts')
    const { result } = renderHook(() => useCreateCompany(), {
      wrapper: Wrapper
    })

    result.current.mutate({
      teamId: 'team-1',
      payload: { name: 'New Corp', siret: '22222222222222' }
    })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))

    const cached = queryClient.getQueryData<Company[]>([
      'team',
      'team-1',
      'companies'
    ])
    expect(cached).toEqual([newCompany])
  })

  it('should set error state on failure', async () => {
    vi.mocked(teamService.createCompany).mockRejectedValue(
      new Error('Duplicate SIRET')
    )
    const { Wrapper } = createWrapper()

    const { useCreateCompany } = await import('../use-create-company.ts')
    const { result } = renderHook(() => useCreateCompany(), {
      wrapper: Wrapper
    })

    result.current.mutate({
      teamId: 'team-1',
      payload: { name: 'X', siret: '0' }
    })

    await waitFor(() => expect(result.current.isError).toBe(true))
    expect(result.current.error?.message).toBe('Duplicate SIRET')
  })
})
