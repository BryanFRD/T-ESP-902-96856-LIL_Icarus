import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { getKanbanBoard } from '@/service/kanban-service.ts'

import { useKanbanBoard } from '../use-kanban-board.ts'

vi.mock('@/service/kanban-service.ts', () => ({
  getKanbanBoard: vi.fn()
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

describe('useKanbanBoard', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should fetch kanban board for a given audit', async () => {
    const mockBoard = { id: 'board-1', columns: [] }
    vi.mocked(getKanbanBoard).mockResolvedValue(mockBoard as never)

    const { result } = renderHook(() => useKanbanBoard('audit-1'), {
      wrapper: createWrapper()
    })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(result.current.data).toEqual(mockBoard)
    expect(getKanbanBoard).toHaveBeenCalledWith('audit-1')
  })

  it('should handle error', async () => {
    vi.mocked(getKanbanBoard).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(() => useKanbanBoard('audit-1'), {
      wrapper: createWrapper()
    })

    await waitFor(() => expect(result.current.isError).toBe(true))
    expect(result.current.error).toBeInstanceOf(Error)
  })

  it('should not fetch when auditId is empty', async () => {
    const { result } = renderHook(() => useKanbanBoard(''), {
      wrapper: createWrapper()
    })

    expect(result.current.fetchStatus).toBe('idle')
    expect(getKanbanBoard).not.toHaveBeenCalled()
  })
})
