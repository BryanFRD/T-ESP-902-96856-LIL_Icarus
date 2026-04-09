import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import {
  createColumn,
  deleteColumn,
  reorderColumns,
  updateColumn
} from '@/service/kanban-service.ts'

import {
  useCreateColumn,
  useDeleteColumn,
  useReorderColumns,
  useUpdateColumn
} from '../use-column-mutation.ts'

vi.mock('@/service/kanban-service.ts', () => ({
  createColumn: vi.fn(),
  updateColumn: vi.fn(),
  deleteColumn: vi.fn(),
  reorderColumns: vi.fn()
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

describe('useCreateColumn', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call createColumn service on mutate', async () => {
    const mockResponse = { id: 'col-1', name: 'To Do' }
    vi.mocked(createColumn).mockResolvedValue(mockResponse as never)

    const { result } = renderHook(() => useCreateColumn('audit-1'), {
      wrapper: createWrapper()
    })

    result.current.mutate({ name: 'To Do' } as never)
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(createColumn).toHaveBeenCalledWith('audit-1', { name: 'To Do' })
  })

  it('should handle error', async () => {
    vi.mocked(createColumn).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(() => useCreateColumn('audit-1'), {
      wrapper: createWrapper()
    })

    result.current.mutate({ name: 'Bad' } as never)
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})

describe('useUpdateColumn', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call updateColumn service on mutate', async () => {
    const mockResponse = { id: 'col-1', name: 'In Progress' }
    vi.mocked(updateColumn).mockResolvedValue(mockResponse as never)

    const { result } = renderHook(() => useUpdateColumn('audit-1', 'col-1'), {
      wrapper: createWrapper()
    })

    result.current.mutate({ name: 'In Progress' } as never)
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(updateColumn).toHaveBeenCalledWith('audit-1', 'col-1', {
      name: 'In Progress'
    })
  })

  it('should handle error', async () => {
    vi.mocked(updateColumn).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(() => useUpdateColumn('audit-1', 'col-1'), {
      wrapper: createWrapper()
    })

    result.current.mutate({ name: 'Bad' } as never)
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})

describe('useDeleteColumn', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call deleteColumn service on mutate', async () => {
    vi.mocked(deleteColumn).mockResolvedValue(undefined)

    const { result } = renderHook(() => useDeleteColumn('audit-1'), {
      wrapper: createWrapper()
    })

    result.current.mutate('col-1')
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(deleteColumn).toHaveBeenCalledWith('audit-1', 'col-1')
  })

  it('should handle error', async () => {
    vi.mocked(deleteColumn).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(() => useDeleteColumn('audit-1'), {
      wrapper: createWrapper()
    })

    result.current.mutate('col-1')
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})

describe('useReorderColumns', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call reorderColumns service on mutate', async () => {
    vi.mocked(reorderColumns).mockResolvedValue(undefined)

    const { result } = renderHook(() => useReorderColumns('audit-1'), {
      wrapper: createWrapper()
    })

    result.current.mutate({ columnIds: ['col-2', 'col-1', 'col-3'] } as never)
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(reorderColumns).toHaveBeenCalledWith('audit-1', {
      columnIds: ['col-2', 'col-1', 'col-3']
    })
  })

  it('should handle error', async () => {
    vi.mocked(reorderColumns).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(() => useReorderColumns('audit-1'), {
      wrapper: createWrapper()
    })

    result.current.mutate({ columnIds: [] } as never)
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})
