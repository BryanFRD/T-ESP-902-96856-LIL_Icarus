import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { MemberRole } from '@/domain/enum/member-role.ts'
import { UserRole } from '@/domain/enum/role-enum.ts'
import type { Team, TeamMember } from '@/domain/types/team.ts'
import * as teamService from '@/service/team-service.ts'

vi.mock('@/service/team-service.ts', () => ({
  removeMember: vi.fn()
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

const member1: TeamMember = {
  id: 'user-1',
  username: 'alice',
  firstName: 'Alice',
  lastName: 'Smith',
  email: 'alice@test.com',
  userRole: UserRole.PENTESTER,
  teamRole: MemberRole.OWNER,
  createdAt: '2024-01-01',
  updatedAt: '2024-01-01'
}

const member2: TeamMember = {
  id: 'user-2',
  username: 'bob',
  firstName: 'Bob',
  lastName: 'Jones',
  email: 'bob@test.com',
  userRole: UserRole.PENTESTER,
  teamRole: MemberRole.MEMBER,
  createdAt: '2024-01-01',
  updatedAt: '2024-01-01'
}

const mockTeam: Team = {
  id: 'team-1',
  name: 'Test Team',
  createdAt: '2024-01-01',
  updatedAt: '2024-01-01',
  members: [member1, member2]
}

describe('useRemoveMember', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should remove member and update cache', async () => {
    vi.mocked(teamService.removeMember).mockResolvedValue(undefined)
    const { Wrapper, queryClient } = createWrapper()
    queryClient.setQueryData(['team', 'me'], [mockTeam])

    const invalidateSpy = vi.spyOn(queryClient, 'invalidateQueries')

    const { useRemoveMember } = await import('../use-remove-member.ts')
    const { result } = renderHook(() => useRemoveMember(), { wrapper: Wrapper })

    result.current.mutate({ teamId: 'team-1', memberId: 'user-2' })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))

    expect(invalidateSpy).toHaveBeenCalledWith({ queryKey: ['team', 'me'] })
    expect(invalidateSpy).toHaveBeenCalledWith({
      queryKey: ['team', 'team-1']
    })

    const cached = queryClient.getQueryData<Team[]>(['team', 'me'])
    expect(cached?.[0].members).toHaveLength(1)
    expect(cached?.[0].members?.[0].id).toBe('user-1')
  })

  it('should handle team without members array', async () => {
    vi.mocked(teamService.removeMember).mockResolvedValue(undefined)
    const { Wrapper, queryClient } = createWrapper()
    const teamNoMembers: Team = {
      id: 'team-1',
      name: 'Test',
      createdAt: '2024-01-01',
      updatedAt: '2024-01-01'
    }
    queryClient.setQueryData(['team', 'me'], [teamNoMembers])

    const { useRemoveMember } = await import('../use-remove-member.ts')
    const { result } = renderHook(() => useRemoveMember(), { wrapper: Wrapper })

    result.current.mutate({ teamId: 'team-1', memberId: 'user-2' })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))

    const cached = queryClient.getQueryData<Team[]>(['team', 'me'])
    expect(cached?.[0].members).toBeUndefined()
  })

  it('should set error state on failure', async () => {
    vi.mocked(teamService.removeMember).mockRejectedValue(
      new Error('Cannot remove owner')
    )
    const { Wrapper } = createWrapper()

    const { useRemoveMember } = await import('../use-remove-member.ts')
    const { result } = renderHook(() => useRemoveMember(), { wrapper: Wrapper })

    result.current.mutate({ teamId: 'team-1', memberId: 'user-1' })

    await waitFor(() => expect(result.current.isError).toBe(true))
    expect(result.current.error?.message).toBe('Cannot remove owner')
  })
})
