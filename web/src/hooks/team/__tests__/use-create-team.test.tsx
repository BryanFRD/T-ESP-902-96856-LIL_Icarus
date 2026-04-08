import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { createTeam } from '@/service/team-service.ts'

import { useCreateTeam } from '../use-create-team.ts'

vi.mock('@/service/team-service.ts', () => ({
  createTeam: vi.fn()
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

describe('useCreateTeam', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call createTeam service on mutate', async () => {
    const newTeam = { id: '1', name: 'New Team', createdAt: '', updatedAt: '' }
    vi.mocked(createTeam).mockResolvedValue(newTeam as never)
    const { result } = renderHook(() => useCreateTeam(), {
      wrapper: createWrapper()
    })
    result.current.mutate({ name: 'New Team' })
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(createTeam).toHaveBeenCalledWith(
      { name: 'New Team' },
      expect.anything()
    )
  })
})
