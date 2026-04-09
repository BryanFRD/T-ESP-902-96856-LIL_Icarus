import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { me } from '@/service/auth-service.ts'
import { deleteUser } from '@/service/user-service.ts'

import { useDeleteUser } from '../use-delete-user.ts'

vi.mock('@/service/user-service.ts', () => ({
  deleteUser: vi.fn()
}))

vi.mock('@/service/auth-service.ts', () => ({
  me: vi.fn()
}))

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: { retry: false },
      mutations: { retry: false }
    }
  })
  function Wrapper({ children }: { children: ReactNode }) {
    return (
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    )
  }
  return Wrapper
}

describe('useDeleteUser', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    vi.mocked(me).mockResolvedValue({
      id: 'me-id',
      username: 'me',
      email: 'me@test.com'
    } as never)
  })

  it('should call deleteUser service on mutate', async () => {
    vi.mocked(deleteUser).mockResolvedValue(undefined)
    const { result } = renderHook(() => useDeleteUser(), {
      wrapper: createWrapper()
    })
    result.current.mutate('user-1')
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(deleteUser).toHaveBeenCalledWith('user-1', expect.anything())
  })

  it('should handle error', async () => {
    vi.mocked(deleteUser).mockRejectedValue(new Error('delete failed'))
    const { result } = renderHook(() => useDeleteUser(), {
      wrapper: createWrapper()
    })
    result.current.mutate('user-1')
    await waitFor(() => expect(result.current.isError).toBe(true))
  })

  it('should invalidate user queries on success', async () => {
    vi.mocked(deleteUser).mockResolvedValue(undefined)
    const queryClient = new QueryClient({
      defaultOptions: {
        queries: { retry: false },
        mutations: { retry: false }
      }
    })
    const spy = vi.spyOn(queryClient, 'invalidateQueries')
    function Wrapper({ children }: { children: ReactNode }) {
      return (
        <QueryClientProvider client={queryClient}>
          {children}
        </QueryClientProvider>
      )
    }
    const { result } = renderHook(() => useDeleteUser(), {
      wrapper: Wrapper
    })
    result.current.mutate('user-1')
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(spy).toHaveBeenCalledWith({ queryKey: ['user'] })
    expect(spy).toHaveBeenCalledWith({ queryKey: ['user', 'user-1'] })
  })

  it('should invalidate me query when deleting self', async () => {
    vi.mocked(deleteUser).mockResolvedValue(undefined)
    const queryClient = new QueryClient({
      defaultOptions: {
        queries: { retry: false },
        mutations: { retry: false }
      }
    })
    queryClient.setQueryData(['user', 'me'], {
      id: 'me-id',
      username: 'me'
    })
    const spy = vi.spyOn(queryClient, 'invalidateQueries')
    function Wrapper({ children }: { children: ReactNode }) {
      return (
        <QueryClientProvider client={queryClient}>
          {children}
        </QueryClientProvider>
      )
    }
    const { result } = renderHook(() => useDeleteUser(), {
      wrapper: Wrapper
    })
    result.current.mutate('me-id')
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(spy).toHaveBeenCalledWith({ queryKey: ['user', 'me'] })
  })
})
