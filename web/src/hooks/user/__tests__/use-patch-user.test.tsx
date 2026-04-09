import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { me } from '@/service/auth-service.ts'
import { patchUser } from '@/service/user-service.ts'

import { usePatchUser } from '../use-patch-user.ts'

vi.mock('@/service/user-service.ts', () => ({
  patchUser: vi.fn()
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

describe('usePatchUser', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    vi.mocked(me).mockResolvedValue({
      id: 'me-id',
      username: 'me',
      email: 'me@test.com'
    } as never)
  })

  it('should call patchUser service on mutate', async () => {
    const updatedUser = { id: 'u1', username: 'patched' }
    vi.mocked(patchUser).mockResolvedValue(updatedUser as never)
    const { result } = renderHook(() => usePatchUser(), {
      wrapper: createWrapper()
    })
    result.current.mutate({ id: 'u1', payload: { username: 'patched' } })
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(patchUser).toHaveBeenCalledWith(
      { id: 'u1', payload: { username: 'patched' } },
      expect.anything()
    )
  })

  it('should handle error', async () => {
    vi.mocked(patchUser).mockRejectedValue(new Error('patch failed'))
    const { result } = renderHook(() => usePatchUser(), {
      wrapper: createWrapper()
    })
    result.current.mutate({ id: 'u1', payload: { username: 'bad' } })
    await waitFor(() => expect(result.current.isError).toBe(true))
  })

  it('should invalidate user queries on success', async () => {
    const updatedUser = { id: 'u1', username: 'patched' }
    vi.mocked(patchUser).mockResolvedValue(updatedUser as never)
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
    const { result } = renderHook(() => usePatchUser(), {
      wrapper: Wrapper
    })
    result.current.mutate({ id: 'u1', payload: { username: 'patched' } })
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(spy).toHaveBeenCalledWith({ queryKey: ['user'] })
    expect(spy).toHaveBeenCalledWith({ queryKey: ['user', 'u1'] })
  })

  it('should invalidate me query when patching self', async () => {
    const updatedUser = { id: 'me-id', username: 'updated-me' }
    vi.mocked(patchUser).mockResolvedValue(updatedUser as never)
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
    const { result } = renderHook(() => usePatchUser(), {
      wrapper: Wrapper
    })
    result.current.mutate({
      id: 'me-id',
      payload: { username: 'updated-me' }
    })
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(spy).toHaveBeenCalledWith({ queryKey: ['user', 'me'] })
  })
})
