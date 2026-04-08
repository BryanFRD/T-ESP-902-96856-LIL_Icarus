import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { login } from '@/service/auth-service.ts'

import { useLogin } from '../use-login.ts'

vi.mock('@/service/auth-service.ts', () => ({
  login: vi.fn()
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
  return Wrapper
}

describe('useLogin', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call login service on mutate', async () => {
    vi.mocked(login).mockResolvedValue(undefined)
    const { result } = renderHook(() => useLogin(), {
      wrapper: createWrapper()
    })
    result.current.mutate({ email: 'test@test.com', password: 'pass' })
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(login).toHaveBeenCalledWith(
      { email: 'test@test.com', password: 'pass' },
      expect.anything()
    )
  })

  it('should handle login error', async () => {
    vi.mocked(login).mockRejectedValue(new Error('fail'))
    const { result } = renderHook(() => useLogin(), {
      wrapper: createWrapper()
    })
    result.current.mutate({ email: 'test@test.com', password: 'wrong' })
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})
