import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { changePassword, me } from '@/service/auth-service.ts'

import { useChangePassword } from '../use-change-password.ts'

vi.mock('@/service/auth-service.ts', () => ({
  changePassword: vi.fn(),
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

const mockPayload = {
  password: 'oldpass',
  newPassword: 'newpass',
  confirmPassword: 'newpass'
}

describe('useChangePassword', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    vi.mocked(me).mockResolvedValue({
      id: 'me-id',
      username: 'me',
      email: 'me@test.com'
    } as never)
  })

  it('should call changePassword service on mutate', async () => {
    vi.mocked(changePassword).mockResolvedValue(undefined as never)
    const { result } = renderHook(() => useChangePassword(), {
      wrapper: createWrapper()
    })
    result.current.mutate(mockPayload)
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(changePassword).toHaveBeenCalledWith(mockPayload)
  })

  it('should handle error', async () => {
    vi.mocked(changePassword).mockRejectedValue(new Error('wrong password'))
    const { result } = renderHook(() => useChangePassword(), {
      wrapper: createWrapper()
    })
    result.current.mutate(mockPayload)
    await waitFor(() => expect(result.current.isError).toBe(true))
  })

  it('should invalidate queries on success', async () => {
    vi.mocked(changePassword).mockResolvedValue(undefined as never)
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
    const { result } = renderHook(() => useChangePassword(), {
      wrapper: Wrapper
    })
    result.current.mutate(mockPayload)
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(spy).toHaveBeenCalledWith({ queryKey: ['users'] })
    expect(spy).toHaveBeenCalledWith({ queryKey: ['user', 'me'] })
  })
})
