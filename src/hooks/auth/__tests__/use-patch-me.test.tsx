import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { UserRole } from '@/domain/enum/role-enum.ts'
import type { User } from '@/domain/types/user.ts'
import * as authService from '@/service/auth-service.ts'

vi.mock('@/service/auth-service.ts', () => ({
  patchMe: vi.fn(),
  me: vi.fn()
}))

const mockUser: User = {
  id: 'user-1',
  username: 'testuser',
  firstName: 'Test',
  lastName: 'User',
  email: 'test@test.com',
  role: UserRole.PENTESTER,
  createdAt: '2024-01-01',
  updatedAt: '2024-01-01'
}

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false }, mutations: { retry: false } }
  })
  // Pre-seed the me query so useMe().data is available
  queryClient.setQueryData(['user', 'me'], mockUser)

  function Wrapper({ children }: { children: ReactNode }) {
    return (
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    )
  }
  return { Wrapper, queryClient }
}

describe('usePatchMe', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should patch user and update all related caches', async () => {
    vi.mocked(authService.patchMe).mockResolvedValue(undefined)
    vi.mocked(authService.me).mockResolvedValue(mockUser)

    const { Wrapper, queryClient } = createWrapper()
    queryClient.setQueryData(['user', mockUser.id], mockUser)
    queryClient.setQueryData(['users'], [mockUser])

    const invalidateSpy = vi.spyOn(queryClient, 'invalidateQueries')

    const { usePatchMe } = await import('../use-patch-me.ts')
    const { result } = renderHook(() => usePatchMe(), { wrapper: Wrapper })

    result.current.mutate({ firstName: 'Updated' })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))

    expect(vi.mocked(authService.patchMe).mock.calls[0][0]).toEqual({
      firstName: 'Updated'
    })

    // Verify invalidation was called
    expect(invalidateSpy).toHaveBeenCalledWith({ queryKey: ['users'] })
    expect(invalidateSpy).toHaveBeenCalledWith({
      queryKey: ['user', mockUser.id]
    })
    expect(invalidateSpy).toHaveBeenCalledWith({
      queryKey: ['user', 'me']
    })
  })

  it('should call setQueryData for user me cache on success', async () => {
    vi.mocked(authService.patchMe).mockResolvedValue(undefined)
    vi.mocked(authService.me).mockResolvedValue(mockUser)

    const { Wrapper, queryClient } = createWrapper()
    const setQueryDataSpy = vi.spyOn(queryClient, 'setQueryData')

    const { usePatchMe } = await import('../use-patch-me.ts')
    const { result } = renderHook(() => usePatchMe(), { wrapper: Wrapper })

    result.current.mutate({ lastName: 'NewLast' })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))

    expect(setQueryDataSpy).toHaveBeenCalledWith(
      ['user', 'me'],
      expect.any(Function)
    )
  })

  it('should not update users list for mismatched user id', async () => {
    vi.mocked(authService.patchMe).mockResolvedValue(undefined)
    vi.mocked(authService.me).mockResolvedValue(mockUser)

    const otherUser: User = {
      ...mockUser,
      id: 'other-user',
      firstName: 'Other'
    }
    const { Wrapper, queryClient } = createWrapper()
    queryClient.setQueryData(['users'], [otherUser])

    const { usePatchMe } = await import('../use-patch-me.ts')
    const { result } = renderHook(() => usePatchMe(), { wrapper: Wrapper })

    result.current.mutate({ firstName: 'Changed' })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))

    const cachedUsers = queryClient.getQueryData<User[]>(['users'])
    expect(cachedUsers?.[0].firstName).toBe('Other')
  })

  it('should set error state on failure', async () => {
    vi.mocked(authService.patchMe).mockRejectedValue(
      new Error('Validation failed')
    )
    vi.mocked(authService.me).mockResolvedValue(mockUser)

    const { Wrapper } = createWrapper()

    const { usePatchMe } = await import('../use-patch-me.ts')
    const { result } = renderHook(() => usePatchMe(), { wrapper: Wrapper })

    result.current.mutate({ email: 'invalid' })

    await waitFor(() => expect(result.current.isError).toBe(true))
    expect(result.current.error?.message).toBe('Validation failed')
  })
})
