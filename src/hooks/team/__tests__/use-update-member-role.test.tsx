import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { MemberRole } from '@/domain/enum/member-role.ts'
import { UserRole } from '@/domain/enum/role-enum.ts'
import type { Team, TeamMember } from '@/domain/types/team.ts'
import * as teamService from '@/service/team-service.ts'

vi.mock('@/service/team-service.ts', () => ({
  updateMemberRole: vi.fn()
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

const member: TeamMember = {
  id: 'user-1',
  username: 'alice',
  firstName: 'Alice',
  lastName: 'Smith',
  email: 'alice@test.com',
  userRole: UserRole.PENTESTER,
  teamRole: MemberRole.MEMBER,
  createdAt: '2024-01-01',
  updatedAt: '2024-01-01'
}

const updatedMember: TeamMember = {
  ...member,
  teamRole: MemberRole.MODERATOR,
  updatedAt: '2024-06-01'
}

const mockTeam: Team = {
  id: 'team-1',
  name: 'Test Team',
  createdAt: '2024-01-01',
  updatedAt: '2024-01-01',
  members: [member]
}

describe('useUpdateMemberRole', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should update member role and update cache', async () => {
    vi.mocked(teamService.updateMemberRole).mockResolvedValue(updatedMember)
    const { Wrapper, queryClient } = createWrapper()
    queryClient.setQueryData(['team', 'me'], [mockTeam])

    const invalidateSpy = vi.spyOn(queryClient, 'invalidateQueries')

    const { useUpdateMemberRole } = await import('../use-update-member-role.ts')
    const { result } = renderHook(() => useUpdateMemberRole(), {
      wrapper: Wrapper
    })

    result.current.mutate({
      teamId: 'team-1',
      memberId: 'user-1',
      payload: { role: MemberRole.MODERATOR }
    })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))

    expect(invalidateSpy).toHaveBeenCalledWith({ queryKey: ['team', 'me'] })
    expect(invalidateSpy).toHaveBeenCalledWith({
      queryKey: ['team', 'team-1']
    })

    const cached = queryClient.getQueryData<Team[]>(['team', 'me'])
    expect(cached?.[0].members?.[0].teamRole).toBe(MemberRole.MODERATOR)
  })

  it('should handle team without members array', async () => {
    vi.mocked(teamService.updateMemberRole).mockResolvedValue(updatedMember)
    const { Wrapper, queryClient } = createWrapper()
    const teamNoMembers: Team = {
      id: 'team-1',
      name: 'Test',
      createdAt: '2024-01-01',
      updatedAt: '2024-01-01'
    }
    queryClient.setQueryData(['team', 'me'], [teamNoMembers])

    const { useUpdateMemberRole } = await import('../use-update-member-role.ts')
    const { result } = renderHook(() => useUpdateMemberRole(), {
      wrapper: Wrapper
    })

    result.current.mutate({
      teamId: 'team-1',
      memberId: 'user-1',
      payload: { role: MemberRole.MODERATOR }
    })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))

    const cached = queryClient.getQueryData<Team[]>(['team', 'me'])
    expect(cached?.[0].members).toBeUndefined()
  })

  it('should set error state on failure', async () => {
    vi.mocked(teamService.updateMemberRole).mockRejectedValue(
      new Error('Forbidden')
    )
    const { Wrapper } = createWrapper()

    const { useUpdateMemberRole } = await import('../use-update-member-role.ts')
    const { result } = renderHook(() => useUpdateMemberRole(), {
      wrapper: Wrapper
    })

    result.current.mutate({
      teamId: 'team-1',
      memberId: 'user-1',
      payload: { role: MemberRole.OWNER }
    })

    await waitFor(() => expect(result.current.isError).toBe(true))
    expect(result.current.error?.message).toBe('Forbidden')
  })
})
