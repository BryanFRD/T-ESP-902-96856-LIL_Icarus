import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import type { Invitation } from '@/domain/types/team.ts'
import * as teamService from '@/service/team-service.ts'

vi.mock('@/service/team-service.ts', () => ({
  createInvitation: vi.fn()
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

const existingInvitation: Invitation = {
  id: 'inv-1',
  teamId: 'team-1',
  token: 'old-token',
  maxUses: 5,
  usedCount: 0,
  isCanceled: false,
  createdAt: '2024-01-01',
  updatedAt: '2024-01-01'
}

const newInvitation: Invitation = {
  id: 'inv-2',
  teamId: 'team-1',
  token: 'new-token',
  maxUses: 10,
  usedCount: 0,
  isCanceled: false,
  createdAt: '2024-06-01',
  updatedAt: '2024-06-01'
}

describe('useCreateInvitation', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should create invitation and append it to the cache', async () => {
    vi.mocked(teamService.createInvitation).mockResolvedValue(newInvitation)
    const { Wrapper, queryClient } = createWrapper()
    queryClient.setQueryData(
      ['team', 'team-1', 'invitations'],
      [existingInvitation]
    )

    const invalidateSpy = vi.spyOn(queryClient, 'invalidateQueries')

    const { useCreateInvitation } = await import('../use-create-invitation.ts')
    const { result } = renderHook(() => useCreateInvitation(), {
      wrapper: Wrapper
    })

    result.current.mutate({
      teamId: 'team-1',
      payload: { maxUses: 10, expiresAt: null }
    })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))

    expect(invalidateSpy).toHaveBeenCalledWith({
      queryKey: ['team', 'team-1', 'invitations']
    })

    const cached = queryClient.getQueryData<Invitation[]>([
      'team',
      'team-1',
      'invitations'
    ])
    expect(cached).toHaveLength(2)
    expect(cached?.[1]).toEqual(newInvitation)
  })

  it('should initialize cache when no existing data', async () => {
    vi.mocked(teamService.createInvitation).mockResolvedValue(newInvitation)
    const { Wrapper, queryClient } = createWrapper()

    const { useCreateInvitation } = await import('../use-create-invitation.ts')
    const { result } = renderHook(() => useCreateInvitation(), {
      wrapper: Wrapper
    })

    result.current.mutate({
      teamId: 'team-1',
      payload: { maxUses: 10, expiresAt: null }
    })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))

    const cached = queryClient.getQueryData<Invitation[]>([
      'team',
      'team-1',
      'invitations'
    ])
    expect(cached).toEqual([newInvitation])
  })

  it('should set error state on failure', async () => {
    vi.mocked(teamService.createInvitation).mockRejectedValue(
      new Error('Rate limited')
    )
    const { Wrapper } = createWrapper()

    const { useCreateInvitation } = await import('../use-create-invitation.ts')
    const { result } = renderHook(() => useCreateInvitation(), {
      wrapper: Wrapper
    })

    result.current.mutate({
      teamId: 'team-1',
      payload: { maxUses: 1, expiresAt: null }
    })

    await waitFor(() => expect(result.current.isError).toBe(true))
    expect(result.current.error?.message).toBe('Rate limited')
  })
})
