import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import type { User } from '@/domain/types/user.ts'
import { me } from '@/service/auth-service.ts'
import { updateUser } from '@/service/user-service.ts'

import { useUpdateUser } from '../use-update-user.ts'

vi.mock('@/service/user-service.ts', () => ({
  updateUser: vi.fn()
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

const mockPayload: User = {
  id: 'u1',
  username: 'updated',
  firstName: 'Updated',
  lastName: 'User',
  email: 'updated@test.com',
  role: 'USER' as User['role'],
  createdAt: '',
  updatedAt: ''
}

describe('useUpdateUser', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    vi.mocked(me).mockResolvedValue({
      id: 'me-id',
      username: 'me',
      email: 'me@test.com'
    } as never)
  })

  it('should call updateUser service on mutate', async () => {
    vi.mocked(updateUser).mockResolvedValue(mockPayload as never)
    const { result } = renderHook(() => useUpdateUser(), {
      wrapper: createWrapper()
    })
    result.current.mutate({ id: 'u1', payload: mockPayload })
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(updateUser).toHaveBeenCalledWith(
      { id: 'u1', payload: mockPayload },
      expect.anything()
    )
  })

  it('should handle error', async () => {
    vi.mocked(updateUser).mockRejectedValue(new Error('update failed'))
    const { result } = renderHook(() => useUpdateUser(), {
      wrapper: createWrapper()
    })
    result.current.mutate({ id: 'u1', payload: mockPayload })
    await waitFor(() => expect(result.current.isError).toBe(true))
  })

  it('should invalidate user queries on success', async () => {
    vi.mocked(updateUser).mockResolvedValue(mockPayload as never)
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
    const { result } = renderHook(() => useUpdateUser(), {
      wrapper: Wrapper
    })
    result.current.mutate({ id: 'u1', payload: mockPayload })
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(spy).toHaveBeenCalledWith({ queryKey: ['user'] })
    expect(spy).toHaveBeenCalledWith({ queryKey: ['user', 'u1'] })
  })

  it('should invalidate me query when updating self', async () => {
    vi.mocked(updateUser).mockResolvedValue(mockPayload as never)
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
    const { result } = renderHook(() => useUpdateUser(), {
      wrapper: Wrapper
    })
    result.current.mutate({ id: 'me-id', payload: mockPayload })
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(spy).toHaveBeenCalledWith({ queryKey: ['user', 'me'] })
  })
})
