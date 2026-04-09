import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import type { Invitation } from '@/domain/types/team.ts'
import * as teamService from '@/service/team-service.ts'

vi.mock('@/service/team-service.ts', () => ({
  getTeamInvitations: vi.fn()
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

const mockInvitations: Invitation[] = [
  {
    id: 'inv-1',
    teamId: 'team-1',
    token: 'abc123',
    maxUses: 5,
    usedCount: 1,
    isCanceled: false,
    createdAt: '2024-01-01',
    updatedAt: '2024-01-01'
  }
]

describe('useTeamInvitation', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should fetch invitations for a given team', async () => {
    vi.mocked(teamService.getTeamInvitations).mockResolvedValue(mockInvitations)
    const { Wrapper } = createWrapper()

    const { useTeamInvitation } = await import('../use-team-invitation.ts')
    const { result } = renderHook(() => useTeamInvitation('team-1'), {
      wrapper: Wrapper
    })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(result.current.data).toEqual(mockInvitations)
    expect(teamService.getTeamInvitations).toHaveBeenCalledWith('team-1')
  })

  it('should not fetch when teamId is empty', async () => {
    const { Wrapper } = createWrapper()

    const { useTeamInvitation } = await import('../use-team-invitation.ts')
    const { result } = renderHook(() => useTeamInvitation(''), {
      wrapper: Wrapper
    })

    await waitFor(() => expect(result.current.fetchStatus).toBe('idle'))
    expect(teamService.getTeamInvitations).not.toHaveBeenCalled()
  })

  it('should set error state when the fetch fails', async () => {
    vi.mocked(teamService.getTeamInvitations).mockRejectedValue(
      new Error('Unauthorized')
    )
    const { Wrapper } = createWrapper()

    const { useTeamInvitation } = await import('../use-team-invitation.ts')
    const { result } = renderHook(() => useTeamInvitation('team-1'), {
      wrapper: Wrapper
    })

    await waitFor(() => expect(result.current.isError).toBe(true))
    expect(result.current.error?.message).toBe('Unauthorized')
  })
})
