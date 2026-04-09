import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import {
  createComment,
  deleteComment,
  getComments,
  updateComment
} from '@/service/kanban-service.ts'

import {
  useComments,
  useCreateComment,
  useDeleteComment,
  useUpdateComment
} from '../use-comment-mutation.ts'

vi.mock('@/service/kanban-service.ts', () => ({
  getComments: vi.fn(),
  createComment: vi.fn(),
  updateComment: vi.fn(),
  deleteComment: vi.fn()
}))

vi.mock('react-i18next', () => ({
  useTranslation: () => ({ t: (key: string) => key })
}))

vi.mock('react-hot-toast', () => ({
  default: { success: vi.fn(), error: vi.fn() }
}))

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: { retry: false },
      mutations: { retry: false }
    }
  })
  function Wrapper({ children }: { children: ReactNode }) {
    return (
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    )
  }
  return Wrapper
}

describe('useComments', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should fetch comments for a ticket', async () => {
    const mockComments = [{ id: 'c-1', content: 'Looks good' }]
    vi.mocked(getComments).mockResolvedValue(mockComments as never)

    const { result } = renderHook(() => useComments('audit-1', 'ticket-1'), {
      wrapper: createWrapper()
    })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(result.current.data).toEqual(mockComments)
    expect(getComments).toHaveBeenCalledWith('audit-1', 'ticket-1')
  })

  it('should handle error', async () => {
    vi.mocked(getComments).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(() => useComments('audit-1', 'ticket-1'), {
      wrapper: createWrapper()
    })

    await waitFor(() => expect(result.current.isError).toBe(true))
  })

  it('should not fetch when auditId is empty', async () => {
    const { result } = renderHook(() => useComments('', 'ticket-1'), {
      wrapper: createWrapper()
    })

    expect(result.current.fetchStatus).toBe('idle')
    expect(getComments).not.toHaveBeenCalled()
  })

  it('should not fetch when ticketId is empty', async () => {
    const { result } = renderHook(() => useComments('audit-1', ''), {
      wrapper: createWrapper()
    })

    expect(result.current.fetchStatus).toBe('idle')
    expect(getComments).not.toHaveBeenCalled()
  })
})

describe('useCreateComment', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call createComment service on mutate', async () => {
    const mockResponse = { id: 'c-1', content: 'New comment' }
    vi.mocked(createComment).mockResolvedValue(mockResponse as never)

    const { result } = renderHook(
      () => useCreateComment('audit-1', 'ticket-1'),
      { wrapper: createWrapper() }
    )

    result.current.mutate({ content: 'New comment' } as never)
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(createComment).toHaveBeenCalledWith('audit-1', 'ticket-1', {
      content: 'New comment'
    })
  })

  it('should handle error', async () => {
    vi.mocked(createComment).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(
      () => useCreateComment('audit-1', 'ticket-1'),
      { wrapper: createWrapper() }
    )

    result.current.mutate({ content: 'Bad' } as never)
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})

describe('useUpdateComment', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call updateComment service on mutate', async () => {
    const mockResponse = { id: 'c-1', content: 'Updated' }
    vi.mocked(updateComment).mockResolvedValue(mockResponse as never)

    const { result } = renderHook(
      () => useUpdateComment('audit-1', 'ticket-1', 'c-1'),
      { wrapper: createWrapper() }
    )

    result.current.mutate({ content: 'Updated' } as never)
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(updateComment).toHaveBeenCalledWith('audit-1', 'ticket-1', 'c-1', {
      content: 'Updated'
    })
  })

  it('should handle error', async () => {
    vi.mocked(updateComment).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(
      () => useUpdateComment('audit-1', 'ticket-1', 'c-1'),
      { wrapper: createWrapper() }
    )

    result.current.mutate({ content: 'Bad' } as never)
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})

describe('useDeleteComment', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call deleteComment service on mutate', async () => {
    vi.mocked(deleteComment).mockResolvedValue(undefined)

    const { result } = renderHook(
      () => useDeleteComment('audit-1', 'ticket-1'),
      { wrapper: createWrapper() }
    )

    result.current.mutate('c-1')
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(deleteComment).toHaveBeenCalledWith('audit-1', 'ticket-1', 'c-1')
  })

  it('should handle error', async () => {
    vi.mocked(deleteComment).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(
      () => useDeleteComment('audit-1', 'ticket-1'),
      { wrapper: createWrapper() }
    )

    result.current.mutate('c-1')
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})
