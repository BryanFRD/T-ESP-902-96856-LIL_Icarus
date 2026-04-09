import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { useUserContext } from '@/context/user-context.tsx'
import { MemberRole } from '@/domain/enum/member-role.ts'
import { useMyTeam } from '@/hooks/team/use-my-team.ts'

import { useAuditRole } from '../use-audit-role.ts'

vi.mock('@/context/user-context.tsx', () => ({
  useUserContext: vi.fn()
}))

vi.mock('@/hooks/team/use-my-team.ts', () => ({
  useMyTeam: vi.fn()
}))

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } }
  })
  function Wrapper({ children }: { children: ReactNode }) {
    return (
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    )
  }
  return Wrapper
}

const mockTeams = [
  {
    id: 'team-1',
    name: 'Alpha',
    members: [
      { id: 'user-1', teamRole: MemberRole.OWNER },
      { id: 'user-2', teamRole: MemberRole.MODERATOR },
      { id: 'user-3', teamRole: MemberRole.MEMBER },
      { id: 'user-4', teamRole: MemberRole.CLIENT }
    ]
  },
  {
    id: 'team-2',
    name: 'Beta',
    members: [{ id: 'user-1', teamRole: MemberRole.MEMBER }]
  }
]

describe('useAuditRole', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should return OWNER role and canManage true', () => {
    vi.mocked(useUserContext).mockReturnValue({
      user: { id: 'user-1' }
    } as never)
    vi.mocked(useMyTeam).mockReturnValue({
      data: mockTeams,
      isLoading: false
    } as never)

    const { result } = renderHook(() => useAuditRole('team-1'), {
      wrapper: createWrapper()
    })

    expect(result.current.role).toBe(MemberRole.OWNER)
    expect(result.current.isOwner).toBe(true)
    expect(result.current.isModerator).toBe(false)
    expect(result.current.isMember).toBe(false)
    expect(result.current.isClient).toBe(false)
    expect(result.current.canManage).toBe(true)
  })

  it('should return MODERATOR role and canManage true', () => {
    vi.mocked(useUserContext).mockReturnValue({
      user: { id: 'user-2' }
    } as never)
    vi.mocked(useMyTeam).mockReturnValue({
      data: mockTeams,
      isLoading: false
    } as never)

    const { result } = renderHook(() => useAuditRole('team-1'), {
      wrapper: createWrapper()
    })

    expect(result.current.role).toBe(MemberRole.MODERATOR)
    expect(result.current.isModerator).toBe(true)
    expect(result.current.canManage).toBe(true)
  })

  it('should return MEMBER role and canManage false', () => {
    vi.mocked(useUserContext).mockReturnValue({
      user: { id: 'user-3' }
    } as never)
    vi.mocked(useMyTeam).mockReturnValue({
      data: mockTeams,
      isLoading: false
    } as never)

    const { result } = renderHook(() => useAuditRole('team-1'), {
      wrapper: createWrapper()
    })

    expect(result.current.role).toBe(MemberRole.MEMBER)
    expect(result.current.isMember).toBe(true)
    expect(result.current.canManage).toBe(false)
  })

  it('should return CLIENT role and canManage false', () => {
    vi.mocked(useUserContext).mockReturnValue({
      user: { id: 'user-4' }
    } as never)
    vi.mocked(useMyTeam).mockReturnValue({
      data: mockTeams,
      isLoading: false
    } as never)

    const { result } = renderHook(() => useAuditRole('team-1'), {
      wrapper: createWrapper()
    })

    expect(result.current.role).toBe(MemberRole.CLIENT)
    expect(result.current.isClient).toBe(true)
    expect(result.current.canManage).toBe(false)
  })

  it('should return null role when user is not in the team', () => {
    vi.mocked(useUserContext).mockReturnValue({
      user: { id: 'user-99' }
    } as never)
    vi.mocked(useMyTeam).mockReturnValue({
      data: mockTeams,
      isLoading: false
    } as never)

    const { result } = renderHook(() => useAuditRole('team-1'), {
      wrapper: createWrapper()
    })

    expect(result.current.role).toBeNull()
    expect(result.current.canManage).toBe(false)
  })

  it('should return null role when teamId does not match any team', () => {
    vi.mocked(useUserContext).mockReturnValue({
      user: { id: 'user-1' }
    } as never)
    vi.mocked(useMyTeam).mockReturnValue({
      data: mockTeams,
      isLoading: false
    } as never)

    const { result } = renderHook(() => useAuditRole('nonexistent'), {
      wrapper: createWrapper()
    })

    expect(result.current.role).toBeNull()
    expect(result.current.canManage).toBe(false)
  })

  it('should return null role when user is null', () => {
    vi.mocked(useUserContext).mockReturnValue({
      user: null
    } as never)
    vi.mocked(useMyTeam).mockReturnValue({
      data: mockTeams,
      isLoading: false
    } as never)

    const { result } = renderHook(() => useAuditRole('team-1'), {
      wrapper: createWrapper()
    })

    expect(result.current.role).toBeNull()
    expect(result.current.canManage).toBe(false)
  })

  it('should return null role when teams data is undefined', () => {
    vi.mocked(useUserContext).mockReturnValue({
      user: { id: 'user-1' }
    } as never)
    vi.mocked(useMyTeam).mockReturnValue({
      data: undefined,
      isLoading: true
    } as never)

    const { result } = renderHook(() => useAuditRole('team-1'), {
      wrapper: createWrapper()
    })

    expect(result.current.role).toBeNull()
    expect(result.current.isLoading).toBe(true)
  })

  it('should return null role when teamId is empty', () => {
    vi.mocked(useUserContext).mockReturnValue({
      user: { id: 'user-1' }
    } as never)
    vi.mocked(useMyTeam).mockReturnValue({
      data: mockTeams,
      isLoading: false
    } as never)

    const { result } = renderHook(() => useAuditRole(''), {
      wrapper: createWrapper()
    })

    expect(result.current.role).toBeNull()
    expect(result.current.canManage).toBe(false)
  })

  it('should resolve role from a different team', () => {
    vi.mocked(useUserContext).mockReturnValue({
      user: { id: 'user-1' }
    } as never)
    vi.mocked(useMyTeam).mockReturnValue({
      data: mockTeams,
      isLoading: false
    } as never)

    const { result } = renderHook(() => useAuditRole('team-2'), {
      wrapper: createWrapper()
    })

    expect(result.current.role).toBe(MemberRole.MEMBER)
    expect(result.current.isMember).toBe(true)
    expect(result.current.canManage).toBe(false)
  })
})
