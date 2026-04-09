import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import type { Invitation } from '@/domain/types/team.ts'
import * as teamService from '@/service/team-service.ts'

vi.mock('@/service/team-service.ts', () => ({
  cancelInvitation: vi.fn()
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

const invitation1: Invitation = {
  id: 'inv-1',
  teamId: 'team-1',
  token: 'token-1',
  maxUses: 5,
  usedCount: 0,
  isCanceled: false,
  createdAt: '2024-01-01',
  updatedAt: '2024-01-01'
}

const invitation2: Invitation = {
  id: 'inv-2',
  teamId: 'team-1',
  token: 'token-2',
  maxUses: 3,
  usedCount: 1,
  isCanceled: false,
  createdAt: '2024-01-01',
  updatedAt: '2024-01-01'
}

describe('useCancelInvitation', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should cancel invitation and remove it from cache', async () => {
    vi.mocked(teamService.cancelInvitation).mockResolvedValue(undefined)
    const { Wrapper, queryClient } = createWrapper()
    queryClient.setQueryData(
      ['team', 'team-1', 'invitations'],
      [invitation1, invitation2]
    )

    const invalidateSpy = vi.spyOn(queryClient, 'invalidateQueries')

    const { useCancelInvitation } = await import('../use-cancel-invitation.ts')
    const { result } = renderHook(() => useCancelInvitation(), {
      wrapper: Wrapper
    })

    result.current.mutate({ teamId: 'team-1', invitationId: 'inv-1' })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))

    expect(invalidateSpy).toHaveBeenCalledWith({
      queryKey: ['team', 'team-1', 'invitations']
    })

    const cached = queryClient.getQueryData<Invitation[]>([
      'team',
      'team-1',
      'invitations'
    ])
    expect(cached).toHaveLength(1)
    expect(cached?.[0].id).toBe('inv-2')
  })

  it('should handle empty cache gracefully', async () => {
    vi.mocked(teamService.cancelInvitation).mockResolvedValue(undefined)
    const { Wrapper, queryClient } = createWrapper()

    const { useCancelInvitation } = await import('../use-cancel-invitation.ts')
    const { result } = renderHook(() => useCancelInvitation(), {
      wrapper: Wrapper
    })

    result.current.mutate({ teamId: 'team-1', invitationId: 'inv-1' })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))

    const cached = queryClient.getQueryData<Invitation[]>([
      'team',
      'team-1',
      'invitations'
    ])
    expect(cached).toBeUndefined()
  })

  it('should set error state on failure', async () => {
    vi.mocked(teamService.cancelInvitation).mockRejectedValue(
      new Error('Not found')
    )
    const { Wrapper } = createWrapper()

    const { useCancelInvitation } = await import('../use-cancel-invitation.ts')
    const { result } = renderHook(() => useCancelInvitation(), {
      wrapper: Wrapper
    })

    result.current.mutate({ teamId: 'team-1', invitationId: 'inv-999' })

    await waitFor(() => expect(result.current.isError).toBe(true))
    expect(result.current.error?.message).toBe('Not found')
  })
})
