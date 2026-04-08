import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { register } from '@/service/auth-service.ts'

import { useRegister } from '../use-register.ts'

vi.mock('@/service/auth-service.ts', () => ({
  register: vi.fn()
}))

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: { mutations: { retry: false } }
  })
  function Wrapper({ children }: { children: ReactNode }) {
    return (
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    )
  }
  return Wrapper
}

describe('useRegister', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call register service on mutate', async () => {
    vi.mocked(register).mockResolvedValue(undefined)
    const { result } = renderHook(() => useRegister(), {
      wrapper: createWrapper()
    })
    const payload = {
      username: 'john',
      firstName: 'John',
      lastName: 'Doe',
      email: 'john@test.com',
      password: 'password123'
    }
    result.current.mutate(payload)
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(register).toHaveBeenCalledWith(payload, expect.anything())
  })

  it('should handle register error', async () => {
    vi.mocked(register).mockRejectedValue(new Error('fail'))
    const { result } = renderHook(() => useRegister(), {
      wrapper: createWrapper()
    })
    result.current.mutate({
      username: 'john',
      firstName: 'John',
      lastName: 'Doe',
      email: 'john@test.com',
      password: 'pass'
    })
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})
