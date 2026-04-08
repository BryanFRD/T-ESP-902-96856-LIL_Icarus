import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { me } from '@/service/auth-service.ts'

import { useMe } from '../use-me.ts'

vi.mock('@/service/auth-service.ts', () => ({
  me: vi.fn()
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

describe('useMe', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should fetch user data', async () => {
    const mockUser = { id: '1', username: 'john', email: 'john@test.com' }
    vi.mocked(me).mockResolvedValue(mockUser as never)
    const { result } = renderHook(() => useMe(), { wrapper: createWrapper() })
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(result.current.data).toEqual(mockUser)
  })

  it('should not fetch when ready is false', () => {
    const { result } = renderHook(() => useMe(false), {
      wrapper: createWrapper()
    })
    expect(result.current.isFetching).toBe(false)
  })

  it('should handle error', async () => {
    vi.mocked(me).mockRejectedValue(new Error('fail'))
    const { result } = renderHook(() => useMe(), { wrapper: createWrapper() })
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})
