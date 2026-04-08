import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import {
  resetPassword,
  sendPasswordResetEmail
} from '@/service/auth-service.ts'

import {
  useResetPassword,
  useSendPasswordResetRequest
} from '../use-send-password-reset-request.ts'

vi.mock('@/service/auth-service.ts', () => ({
  sendPasswordResetEmail: vi.fn(),
  resetPassword: vi.fn()
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

describe('useSendPasswordResetRequest', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call sendPasswordResetEmail on mutate', async () => {
    vi.mocked(sendPasswordResetEmail).mockResolvedValue(undefined)
    const { result } = renderHook(() => useSendPasswordResetRequest(), {
      wrapper: createWrapper()
    })
    result.current.mutate({ resetEmail: 'john@test.com' })
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(sendPasswordResetEmail).toHaveBeenCalledWith(
      { resetEmail: 'john@test.com' },
      expect.anything()
    )
  })
})

describe('useResetPassword', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call resetPassword on mutate', async () => {
    vi.mocked(resetPassword).mockResolvedValue(undefined)
    const { result } = renderHook(() => useResetPassword(), {
      wrapper: createWrapper()
    })
    result.current.mutate({ token: 'abc', newPassword: 'newpass' })
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(resetPassword).toHaveBeenCalledWith(
      { token: 'abc', newPassword: 'newpass' },
      expect.anything()
    )
  })
})
