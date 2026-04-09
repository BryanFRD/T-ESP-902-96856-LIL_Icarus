import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import type { User } from '@/domain/types/user.ts'
import { createUser } from '@/service/user-service.ts'

import { useCreateUser } from '../use-create-user.ts'

vi.mock('@/service/user-service.ts', () => ({
  createUser: vi.fn()
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

const mockPayload: User = {
  id: 'u1',
  username: 'newuser',
  firstName: 'New',
  lastName: 'User',
  email: 'new@test.com',
  role: 'USER' as User['role'],
  createdAt: '',
  updatedAt: ''
}

describe('useCreateUser', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call createUser service on mutate', async () => {
    vi.mocked(createUser).mockResolvedValue(mockPayload as never)
    const { result } = renderHook(() => useCreateUser(), {
      wrapper: createWrapper()
    })
    result.current.mutate(mockPayload)
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(createUser).toHaveBeenCalledWith(mockPayload, expect.anything())
  })

  it('should handle error', async () => {
    vi.mocked(createUser).mockRejectedValue(new Error('create failed'))
    const { result } = renderHook(() => useCreateUser(), {
      wrapper: createWrapper()
    })
    result.current.mutate(mockPayload)
    await waitFor(() => expect(result.current.isError).toBe(true))
  })

  it('should invalidate user queries on success', async () => {
    vi.mocked(createUser).mockResolvedValue(mockPayload as never)
    const queryClient = new QueryClient({
      defaultOptions: { mutations: { retry: false } }
    })
    const spy = vi.spyOn(queryClient, 'invalidateQueries')
    function Wrapper({ children }: { children: ReactNode }) {
      return (
        <QueryClientProvider client={queryClient}>
          {children}
        </QueryClientProvider>
      )
    }
    const { result } = renderHook(() => useCreateUser(), {
      wrapper: Wrapper
    })
    result.current.mutate(mockPayload)
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(spy).toHaveBeenCalledWith({ queryKey: ['user'] })
    expect(spy).toHaveBeenCalledWith({ queryKey: ['user', 'u1'] })
  })
})
