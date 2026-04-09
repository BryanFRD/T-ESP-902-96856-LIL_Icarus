import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import {
  createTicket,
  deleteTicket,
  moveTicket,
  updateTicket
} from '@/service/kanban-service.ts'

import {
  useCreateTicket,
  useDeleteTicket,
  useMoveTicket,
  useUpdateTicket
} from '../use-ticket-mutation.ts'

vi.mock('@/service/kanban-service.ts', () => ({
  createTicket: vi.fn(),
  updateTicket: vi.fn(),
  deleteTicket: vi.fn(),
  moveTicket: vi.fn()
}))

vi.mock('react-i18next', () => ({
  useTranslation: () => ({ t: (key: string) => key })
}))

vi.mock('react-hot-toast', () => ({
  default: { success: vi.fn(), error: vi.fn() }
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

describe('useCreateTicket', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call createTicket service on mutate', async () => {
    const mockResponse = { id: 'ticket-1', title: 'New ticket' }
    vi.mocked(createTicket).mockResolvedValue(mockResponse as never)

    const { result } = renderHook(() => useCreateTicket('audit-1'), {
      wrapper: createWrapper()
    })

    result.current.mutate({ title: 'New ticket', columnId: 'col-1' } as never)
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(createTicket).toHaveBeenCalledWith('audit-1', {
      title: 'New ticket',
      columnId: 'col-1'
    })
  })

  it('should handle error', async () => {
    vi.mocked(createTicket).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(() => useCreateTicket('audit-1'), {
      wrapper: createWrapper()
    })

    result.current.mutate({ title: 'Bad', columnId: 'col-1' } as never)
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})

describe('useUpdateTicket', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call updateTicket service on mutate', async () => {
    const mockResponse = { id: 'ticket-1', title: 'Updated' }
    vi.mocked(updateTicket).mockResolvedValue(mockResponse as never)

    const { result } = renderHook(
      () => useUpdateTicket('audit-1', 'ticket-1'),
      { wrapper: createWrapper() }
    )

    result.current.mutate({ title: 'Updated' } as never)
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(updateTicket).toHaveBeenCalledWith('audit-1', 'ticket-1', {
      title: 'Updated'
    })
  })

  it('should handle error', async () => {
    vi.mocked(updateTicket).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(
      () => useUpdateTicket('audit-1', 'ticket-1'),
      { wrapper: createWrapper() }
    )

    result.current.mutate({ title: 'Bad' } as never)
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})

describe('useDeleteTicket', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call deleteTicket service on mutate', async () => {
    vi.mocked(deleteTicket).mockResolvedValue(undefined)

    const { result } = renderHook(() => useDeleteTicket('audit-1'), {
      wrapper: createWrapper()
    })

    result.current.mutate('ticket-1')
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(deleteTicket).toHaveBeenCalledWith('audit-1', 'ticket-1')
  })

  it('should handle error', async () => {
    vi.mocked(deleteTicket).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(() => useDeleteTicket('audit-1'), {
      wrapper: createWrapper()
    })

    result.current.mutate('ticket-1')
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})

describe('useMoveTicket', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call moveTicket service on mutate', async () => {
    const mockResponse = { id: 'ticket-1' }
    vi.mocked(moveTicket).mockResolvedValue(mockResponse as never)

    const { result } = renderHook(() => useMoveTicket('audit-1'), {
      wrapper: createWrapper()
    })

    result.current.mutate({
      ticketId: 'ticket-1',
      payload: { columnId: 'col-2', position: 0 }
    } as never)
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(moveTicket).toHaveBeenCalledWith('audit-1', 'ticket-1', {
      columnId: 'col-2',
      position: 0
    })
  })

  it('should handle error', async () => {
    vi.mocked(moveTicket).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(() => useMoveTicket('audit-1'), {
      wrapper: createWrapper()
    })

    result.current.mutate({
      ticketId: 'ticket-1',
      payload: { columnId: 'col-2', position: 0 }
    } as never)
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})
