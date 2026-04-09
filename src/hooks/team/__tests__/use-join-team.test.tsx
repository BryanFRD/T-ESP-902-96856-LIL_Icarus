import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import * as teamService from '@/service/team-service.ts'

vi.mock('@/service/team-service.ts', () => ({
  joinTeam: vi.fn()
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

describe('useJoinTeam', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call joinTeam service with the provided token', async () => {
    vi.mocked(teamService.joinTeam).mockResolvedValue(undefined)
    const { Wrapper, queryClient } = createWrapper()
    const invalidateSpy = vi.spyOn(queryClient, 'invalidateQueries')

    const { useJoinTeam } = await import('../use-join-team.ts')
    const { result } = renderHook(() => useJoinTeam(), { wrapper: Wrapper })

    result.current.mutate('invite-token-123')

    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(vi.mocked(teamService.joinTeam).mock.calls[0][0]).toBe(
      'invite-token-123'
    )
    expect(invalidateSpy).toHaveBeenCalledWith({ queryKey: ['team', 'me'] })
  })

  it('should set error state when the service rejects', async () => {
    vi.mocked(teamService.joinTeam).mockRejectedValue(
      new Error('Invalid token')
    )
    const { Wrapper } = createWrapper()

    const { useJoinTeam } = await import('../use-join-team.ts')
    const { result } = renderHook(() => useJoinTeam(), { wrapper: Wrapper })

    result.current.mutate('bad-token')

    await waitFor(() => expect(result.current.isError).toBe(true))
    expect(result.current.error?.message).toBe('Invalid token')
  })
})
