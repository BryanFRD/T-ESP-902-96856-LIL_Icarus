import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import {
  addLabelToTicket,
  createLabel,
  deleteLabel,
  removeLabelFromTicket,
  updateLabel
} from '@/service/kanban-service.ts'

import {
  useAddLabelToTicket,
  useCreateLabel,
  useDeleteLabel,
  useRemoveLabelFromTicket,
  useUpdateLabel
} from '../use-label-mutation.ts'

vi.mock('@/service/kanban-service.ts', () => ({
  createLabel: vi.fn(),
  updateLabel: vi.fn(),
  deleteLabel: vi.fn(),
  addLabelToTicket: vi.fn(),
  removeLabelFromTicket: vi.fn()
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

describe('useCreateLabel', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call createLabel service on mutate', async () => {
    const mockResponse = { id: 'label-1', name: 'Bug', color: '#ff0000' }
    vi.mocked(createLabel).mockResolvedValue(mockResponse as never)

    const { result } = renderHook(() => useCreateLabel('audit-1'), {
      wrapper: createWrapper()
    })

    result.current.mutate({ name: 'Bug', color: '#ff0000' } as never)
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(createLabel).toHaveBeenCalledWith('audit-1', {
      name: 'Bug',
      color: '#ff0000'
    })
  })

  it('should handle error', async () => {
    vi.mocked(createLabel).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(() => useCreateLabel('audit-1'), {
      wrapper: createWrapper()
    })

    result.current.mutate({ name: 'Bad', color: '#000' } as never)
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})

describe('useUpdateLabel', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call updateLabel service on mutate', async () => {
    const mockResponse = { id: 'label-1', name: 'Feature', color: '#00ff00' }
    vi.mocked(updateLabel).mockResolvedValue(mockResponse as never)

    const { result } = renderHook(() => useUpdateLabel('audit-1', 'label-1'), {
      wrapper: createWrapper()
    })

    result.current.mutate({ name: 'Feature', color: '#00ff00' } as never)
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(updateLabel).toHaveBeenCalledWith('audit-1', 'label-1', {
      name: 'Feature',
      color: '#00ff00'
    })
  })

  it('should handle error', async () => {
    vi.mocked(updateLabel).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(() => useUpdateLabel('audit-1', 'label-1'), {
      wrapper: createWrapper()
    })

    result.current.mutate({ name: 'Bad' } as never)
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})

describe('useDeleteLabel', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call deleteLabel service on mutate', async () => {
    vi.mocked(deleteLabel).mockResolvedValue(undefined)

    const { result } = renderHook(() => useDeleteLabel('audit-1'), {
      wrapper: createWrapper()
    })

    result.current.mutate('label-1')
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(deleteLabel).toHaveBeenCalledWith('audit-1', 'label-1')
  })

  it('should handle error', async () => {
    vi.mocked(deleteLabel).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(() => useDeleteLabel('audit-1'), {
      wrapper: createWrapper()
    })

    result.current.mutate('label-1')
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})

describe('useAddLabelToTicket', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call addLabelToTicket service on mutate', async () => {
    vi.mocked(addLabelToTicket).mockResolvedValue(undefined)

    const { result } = renderHook(
      () => useAddLabelToTicket('audit-1', 'ticket-1'),
      { wrapper: createWrapper() }
    )

    result.current.mutate('label-1')
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(addLabelToTicket).toHaveBeenCalledWith(
      'audit-1',
      'ticket-1',
      'label-1'
    )
  })

  it('should handle error', async () => {
    vi.mocked(addLabelToTicket).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(
      () => useAddLabelToTicket('audit-1', 'ticket-1'),
      { wrapper: createWrapper() }
    )

    result.current.mutate('label-1')
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})

describe('useRemoveLabelFromTicket', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call removeLabelFromTicket service on mutate', async () => {
    vi.mocked(removeLabelFromTicket).mockResolvedValue(undefined)

    const { result } = renderHook(
      () => useRemoveLabelFromTicket('audit-1', 'ticket-1'),
      { wrapper: createWrapper() }
    )

    result.current.mutate('label-1')
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(removeLabelFromTicket).toHaveBeenCalledWith(
      'audit-1',
      'ticket-1',
      'label-1'
    )
  })

  it('should handle error', async () => {
    vi.mocked(removeLabelFromTicket).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(
      () => useRemoveLabelFromTicket('audit-1', 'ticket-1'),
      { wrapper: createWrapper() }
    )

    result.current.mutate('label-1')
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})
