import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import type { Team } from '@/domain/types/team.ts'
import * as teamService from '@/service/team-service.ts'

vi.mock('@/service/team-service.ts', () => ({
  patchTeam: vi.fn()
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

const mockTeam: Team = {
  id: 'team-1',
  name: 'Old Name',
  createdAt: '2024-01-01',
  updatedAt: '2024-01-01'
}

const updatedTeam: Team = {
  ...mockTeam,
  name: 'New Name',
  updatedAt: '2024-06-01'
}

describe('usePatchTeam', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should update team and set cache optimistically', async () => {
    vi.mocked(teamService.patchTeam).mockResolvedValue(updatedTeam)
    const { Wrapper, queryClient } = createWrapper()

    queryClient.setQueryData(['team', 'team-1'], mockTeam)
    queryClient.setQueryData(['team', 'me'], [mockTeam])

    const invalidateSpy = vi.spyOn(queryClient, 'invalidateQueries')

    const { usePatchTeam } = await import('../use-patch-team.ts')
    const { result } = renderHook(() => usePatchTeam(), { wrapper: Wrapper })

    result.current.mutate({ teamId: 'team-1', payload: { name: 'New Name' } })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))

    expect(vi.mocked(teamService.patchTeam).mock.calls[0][0]).toEqual({
      teamId: 'team-1',
      payload: { name: 'New Name' }
    })

    expect(invalidateSpy).toHaveBeenCalledWith({ queryKey: ['team', 'me'] })
    expect(invalidateSpy).toHaveBeenCalledWith({ queryKey: ['team', 'team-1'] })

    const cachedTeam = queryClient.getQueryData<Team>(['team', 'team-1'])
    expect(cachedTeam?.name).toBe('New Name')

    const cachedTeams = queryClient.getQueryData<Team[]>(['team', 'me'])
    expect(cachedTeams?.[0].name).toBe('New Name')
  })

  it('should handle missing cache data gracefully', async () => {
    vi.mocked(teamService.patchTeam).mockResolvedValue(updatedTeam)
    const { Wrapper, queryClient } = createWrapper()

    const { usePatchTeam } = await import('../use-patch-team.ts')
    const { result } = renderHook(() => usePatchTeam(), { wrapper: Wrapper })

    result.current.mutate({ teamId: 'team-1', payload: { name: 'New Name' } })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))

    const cachedTeam = queryClient.getQueryData<Team>(['team', 'team-1'])
    expect(cachedTeam).toEqual(updatedTeam)
  })

  it('should set error state on failure', async () => {
    vi.mocked(teamService.patchTeam).mockRejectedValue(new Error('Forbidden'))
    const { Wrapper } = createWrapper()

    const { usePatchTeam } = await import('../use-patch-team.ts')
    const { result } = renderHook(() => usePatchTeam(), { wrapper: Wrapper })

    result.current.mutate({ teamId: 'team-1', payload: { name: 'X' } })

    await waitFor(() => expect(result.current.isError).toBe(true))
    expect(result.current.error?.message).toBe('Forbidden')
  })
})
