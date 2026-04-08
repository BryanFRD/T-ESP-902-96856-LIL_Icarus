import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { logout } from '@/service/auth-service.ts'

import { useLogout } from '../use-logout.ts'

vi.mock('@/service/auth-service.ts', () => ({
  logout: vi.fn()
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

describe('useLogout', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call logout service on mutate', async () => {
    vi.mocked(logout).mockResolvedValue(true)
    const { result } = renderHook(() => useLogout(), {
      wrapper: createWrapper()
    })
    result.current.mutate()
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(logout).toHaveBeenCalled()
    expect(result.current.data).toBe(true)
  })

  it('should handle logout error', async () => {
    vi.mocked(logout).mockRejectedValue(new Error('fail'))
    const { result } = renderHook(() => useLogout(), {
      wrapper: createWrapper()
    })
    result.current.mutate()
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})
