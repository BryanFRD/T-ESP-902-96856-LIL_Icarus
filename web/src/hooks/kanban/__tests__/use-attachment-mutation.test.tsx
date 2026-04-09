import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import {
  deleteAttachment,
  downloadAttachment,
  getAttachmentPreview,
  getAttachments,
  uploadAttachment
} from '@/service/kanban-service.ts'

import {
  useAttachmentPreview,
  useAttachments,
  useDeleteAttachment,
  useDownloadAttachment,
  useUploadAttachment
} from '../use-attachment-mutation.ts'

vi.mock('@/service/kanban-service.ts', () => ({
  getAttachments: vi.fn(),
  uploadAttachment: vi.fn(),
  downloadAttachment: vi.fn(),
  getAttachmentPreview: vi.fn(),
  deleteAttachment: vi.fn()
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

describe('useAttachments', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should fetch attachments for a ticket', async () => {
    const mockAttachments = [{ id: 'att-1', filename: 'report.pdf' }]
    vi.mocked(getAttachments).mockResolvedValue(mockAttachments as never)

    const { result } = renderHook(() => useAttachments('audit-1', 'ticket-1'), {
      wrapper: createWrapper()
    })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(result.current.data).toEqual(mockAttachments)
    expect(getAttachments).toHaveBeenCalledWith('audit-1', 'ticket-1')
  })

  it('should handle error', async () => {
    vi.mocked(getAttachments).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(() => useAttachments('audit-1', 'ticket-1'), {
      wrapper: createWrapper()
    })

    await waitFor(() => expect(result.current.isError).toBe(true))
  })

  it('should not fetch when auditId is empty', async () => {
    const { result } = renderHook(() => useAttachments('', 'ticket-1'), {
      wrapper: createWrapper()
    })

    expect(result.current.fetchStatus).toBe('idle')
    expect(getAttachments).not.toHaveBeenCalled()
  })

  it('should not fetch when ticketId is empty', async () => {
    const { result } = renderHook(() => useAttachments('audit-1', ''), {
      wrapper: createWrapper()
    })

    expect(result.current.fetchStatus).toBe('idle')
    expect(getAttachments).not.toHaveBeenCalled()
  })
})

describe('useUploadAttachment', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call uploadAttachment service on mutate', async () => {
    const mockResponse = { id: 'att-1', filename: 'doc.pdf' }
    vi.mocked(uploadAttachment).mockResolvedValue(mockResponse as never)

    const { result } = renderHook(
      () => useUploadAttachment('audit-1', 'ticket-1'),
      { wrapper: createWrapper() }
    )

    const file = new File(['content'], 'doc.pdf', { type: 'application/pdf' })
    result.current.mutate(file)
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(uploadAttachment).toHaveBeenCalledWith('audit-1', 'ticket-1', file)
  })

  it('should handle error', async () => {
    vi.mocked(uploadAttachment).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(
      () => useUploadAttachment('audit-1', 'ticket-1'),
      { wrapper: createWrapper() }
    )

    const file = new File([''], 'bad.pdf')
    result.current.mutate(file)
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})

describe('useDownloadAttachment', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call downloadAttachment service on mutate', async () => {
    const mockBlob = new Blob(['data'])
    vi.mocked(downloadAttachment).mockResolvedValue(mockBlob)

    const { result } = renderHook(
      () => useDownloadAttachment('audit-1', 'ticket-1'),
      { wrapper: createWrapper() }
    )

    result.current.mutate('att-1')
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(downloadAttachment).toHaveBeenCalledWith(
      'audit-1',
      'ticket-1',
      'att-1'
    )
  })

  it('should handle error', async () => {
    vi.mocked(downloadAttachment).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(
      () => useDownloadAttachment('audit-1', 'ticket-1'),
      { wrapper: createWrapper() }
    )

    result.current.mutate('att-1')
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})

describe('useAttachmentPreview', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    globalThis.URL.createObjectURL = vi.fn(() => 'blob:preview-url')
  })

  it('should fetch attachment preview and create object URL', async () => {
    const mockBlob = new Blob(['image-data'])
    vi.mocked(getAttachmentPreview).mockResolvedValue(mockBlob)

    const { result } = renderHook(
      () => useAttachmentPreview('audit-1', 'ticket-1', 'att-1'),
      { wrapper: createWrapper() }
    )

    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(result.current.data).toBe('blob:preview-url')
    expect(getAttachmentPreview).toHaveBeenCalledWith(
      'audit-1',
      'ticket-1',
      'att-1'
    )
  })

  it('should handle error', async () => {
    vi.mocked(getAttachmentPreview).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(
      () => useAttachmentPreview('audit-1', 'ticket-1', 'att-1'),
      { wrapper: createWrapper() }
    )

    await waitFor(() => expect(result.current.isError).toBe(true))
  })

  it('should not fetch when auditId is empty', async () => {
    const { result } = renderHook(
      () => useAttachmentPreview('', 'ticket-1', 'att-1'),
      { wrapper: createWrapper() }
    )

    expect(result.current.fetchStatus).toBe('idle')
    expect(getAttachmentPreview).not.toHaveBeenCalled()
  })

  it('should not fetch when ticketId is empty', async () => {
    const { result } = renderHook(
      () => useAttachmentPreview('audit-1', '', 'att-1'),
      { wrapper: createWrapper() }
    )

    expect(result.current.fetchStatus).toBe('idle')
    expect(getAttachmentPreview).not.toHaveBeenCalled()
  })

  it('should not fetch when attachmentId is empty', async () => {
    const { result } = renderHook(
      () => useAttachmentPreview('audit-1', 'ticket-1', ''),
      { wrapper: createWrapper() }
    )

    expect(result.current.fetchStatus).toBe('idle')
    expect(getAttachmentPreview).not.toHaveBeenCalled()
  })
})

describe('useDeleteAttachment', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call deleteAttachment service on mutate', async () => {
    vi.mocked(deleteAttachment).mockResolvedValue(undefined)

    const { result } = renderHook(
      () => useDeleteAttachment('audit-1', 'ticket-1'),
      { wrapper: createWrapper() }
    )

    result.current.mutate('att-1')
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(deleteAttachment).toHaveBeenCalledWith(
      'audit-1',
      'ticket-1',
      'att-1'
    )
  })

  it('should handle error', async () => {
    vi.mocked(deleteAttachment).mockRejectedValue(new Error('fail'))

    const { result } = renderHook(
      () => useDeleteAttachment('audit-1', 'ticket-1'),
      { wrapper: createWrapper() }
    )

    result.current.mutate('att-1')
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})
