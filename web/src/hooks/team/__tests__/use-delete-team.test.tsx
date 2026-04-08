import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { deleteTeam } from '@/service/team-service.ts'

import { useDeleteTeam } from '../use-delete-team.ts'

vi.mock('@/service/team-service.ts', () => ({
  deleteTeam: vi.fn()
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

describe('useDeleteTeam', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call deleteTeam service on mutate', async () => {
    vi.mocked(deleteTeam).mockResolvedValue(undefined)
    const { result } = renderHook(() => useDeleteTeam(), {
      wrapper: createWrapper()
    })
    result.current.mutate('team-1')
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(deleteTeam).toHaveBeenCalledWith('team-1', expect.anything())
  })
})
