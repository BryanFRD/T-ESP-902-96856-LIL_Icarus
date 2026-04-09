import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { getUser } from '@/service/user-service.ts'

import { useUser } from '../use-user.ts'

vi.mock('@/service/user-service.ts', () => ({
  getUser: vi.fn()
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

describe('useUser', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should fetch a single user by id', async () => {
    const mockUser = {
      id: 'u1',
      username: 'jane',
      firstName: 'Jane',
      lastName: 'Doe',
      email: 'jane@test.com',
      role: 'USER',
      createdAt: '',
      updatedAt: ''
    }
    vi.mocked(getUser).mockResolvedValue(mockUser as never)
    const { result } = renderHook(() => useUser('u1'), {
      wrapper: createWrapper()
    })
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(result.current.data).toEqual(mockUser)
    expect(getUser).toHaveBeenCalledWith('u1')
  })

  it('should handle error', async () => {
    vi.mocked(getUser).mockRejectedValue(new Error('not found'))
    const { result } = renderHook(() => useUser('bad-id'), {
      wrapper: createWrapper()
    })
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})
