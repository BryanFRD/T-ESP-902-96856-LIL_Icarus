import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { getTicket } from '@/service/kanban-service.ts'

import { useTicket } from '../use-ticket.ts'

vi.mock('@/service/kanban-service.ts', () => ({
  getTicket: vi.fn()
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

describe('useTicket', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should fetch a ticket by auditId and ticketId', async () => {
    const mockTicket = { id: 'ticket-1', title: 'Fix bug' }
    vi.mocked(getTicket).mockResolvedValue(mockTicket as never)

    const { result } = renderHook(() => useTicket('audit-1', 'ticket-1'), {
      wrapper: createWrapper()
    })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(result.current.data).toEqual(mockTicket)
    expect(getTicket).toHaveBeenCalledWith('audit-1', 'ticket-1')
  })

  it('should handle error', async () => {
    vi.mocked(getTicket).mockRejectedValue(new Error('not found'))

    const { result } = renderHook(() => useTicket('audit-1', 'ticket-1'), {
      wrapper: createWrapper()
    })

    await waitFor(() => expect(result.current.isError).toBe(true))
    expect(result.current.error).toBeInstanceOf(Error)
  })

  it('should not fetch when auditId is empty', async () => {
    const { result } = renderHook(() => useTicket('', 'ticket-1'), {
      wrapper: createWrapper()
    })

    expect(result.current.fetchStatus).toBe('idle')
    expect(getTicket).not.toHaveBeenCalled()
  })

  it('should not fetch when ticketId is empty', async () => {
    const { result } = renderHook(() => useTicket('audit-1', ''), {
      wrapper: createWrapper()
    })

    expect(result.current.fetchStatus).toBe('idle')
    expect(getTicket).not.toHaveBeenCalled()
  })
})
