import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { confirmEmail } from '@/service/auth-service.ts'

import { useConfirmEmail } from '../use-confirm-email.ts'

vi.mock('@/service/auth-service.ts', () => ({
  confirmEmail: vi.fn()
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

describe('useConfirmEmail', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call confirmEmail service on mutate', async () => {
    vi.mocked(confirmEmail).mockResolvedValue(true)
    const { result } = renderHook(() => useConfirmEmail(), {
      wrapper: createWrapper()
    })
    result.current.mutate('token123')
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(confirmEmail).toHaveBeenCalledWith('token123', expect.anything())
    expect(result.current.data).toBe(true)
  })

  it('should handle error', async () => {
    vi.mocked(confirmEmail).mockRejectedValue(new Error('fail'))
    const { result } = renderHook(() => useConfirmEmail(), {
      wrapper: createWrapper()
    })
    result.current.mutate('bad-token')
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})
