import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { getUsers } from '@/service/user-service.ts'

import { useUsers } from '../use-users.ts'

vi.mock('@/service/user-service.ts', () => ({
  getUsers: vi.fn()
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

describe('useUsers', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should fetch all users', async () => {
    const mockUsers = [
      { id: '1', username: 'alice', email: 'alice@test.com' },
      { id: '2', username: 'bob', email: 'bob@test.com' }
    ]
    vi.mocked(getUsers).mockResolvedValue(mockUsers as never)
    const { result } = renderHook(() => useUsers(), {
      wrapper: createWrapper()
    })
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(result.current.data).toEqual(mockUsers)
  })

  it('should handle error', async () => {
    vi.mocked(getUsers).mockRejectedValue(new Error('fail'))
    const { result } = renderHook(() => useUsers(), {
      wrapper: createWrapper()
    })
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})
