import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { getMyTeams } from '@/service/team-service.ts'

import { useMyTeam } from '../use-my-team.ts'

vi.mock('@/service/team-service.ts', () => ({
  getMyTeams: vi.fn()
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

describe('useMyTeam', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should fetch teams', async () => {
    const mockTeams = [{ id: '1', name: 'Team 1' }]
    vi.mocked(getMyTeams).mockResolvedValue(mockTeams as never)
    const { result } = renderHook(() => useMyTeam(), {
      wrapper: createWrapper()
    })
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(result.current.data).toEqual(mockTeams)
  })

  it('should handle error', async () => {
    vi.mocked(getMyTeams).mockRejectedValue(new Error('fail'))
    const { result } = renderHook(() => useMyTeam(), {
      wrapper: createWrapper()
    })
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})
