import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { uploadAvatar } from '@/service/user-service.ts'

import { useAvatarUpload } from '../use-avatar-upload.ts'

vi.mock('@/service/user-service.ts', () => ({
  uploadAvatar: vi.fn()
}))

vi.mock('@/utils/api-utils.ts', () => ({
  getErrorMessage: vi.fn()
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

describe('useAvatarUpload', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call uploadAvatar service on mutate', async () => {
    vi.mocked(uploadAvatar).mockResolvedValue('avatar-url.png')
    const file = new File(['img'], 'avatar.png', { type: 'image/png' })
    const { result } = renderHook(() => useAvatarUpload(), {
      wrapper: createWrapper()
    })
    result.current.mutate({ avatar: file })
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(uploadAvatar).toHaveBeenCalledWith(
      { avatar: file },
      expect.anything()
    )
    expect(result.current.data).toBe('avatar-url.png')
  })

  it('should handle error', async () => {
    vi.mocked(uploadAvatar).mockRejectedValue(new Error('upload failed'))
    const file = new File(['img'], 'avatar.png', { type: 'image/png' })
    const { result } = renderHook(() => useAvatarUpload(), {
      wrapper: createWrapper()
    })
    result.current.mutate({ avatar: file })
    await waitFor(() => expect(result.current.isError).toBe(true))
  })

  it('should invalidate me query on success', async () => {
    vi.mocked(uploadAvatar).mockResolvedValue('avatar-url.png')
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
    const file = new File(['img'], 'avatar.png', { type: 'image/png' })
    const { result } = renderHook(() => useAvatarUpload(), {
      wrapper: Wrapper
    })
    result.current.mutate({ avatar: file })
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(spy).toHaveBeenCalledWith({ queryKey: ['user', 'me'] })
  })
})
