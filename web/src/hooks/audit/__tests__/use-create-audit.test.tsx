import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { createAudit } from '@/service/audit-service.ts'

import { useCreateAudit } from '../use-create-audit.ts'

vi.mock('@/service/audit-service.ts', () => ({
  createAudit: vi.fn()
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

describe('useCreateAudit', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call createAudit service on mutate', async () => {
    vi.mocked(createAudit).mockResolvedValue({ id: '1' } as never)
    const { result } = renderHook(() => useCreateAudit(), {
      wrapper: createWrapper()
    })
    const payload = {
      name: 'Audit 1',
      description: 'Test',
      endDate: '2026-12-31',
      companyId: '01HQXYZ',
      teamId: '01HQABC'
    }
    result.current.mutate(payload)
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(createAudit).toHaveBeenCalledWith(payload, expect.anything())
  })

  it('should handle error', async () => {
    vi.mocked(createAudit).mockRejectedValue(new Error('fail'))
    const { result } = renderHook(() => useCreateAudit(), {
      wrapper: createWrapper()
    })
    result.current.mutate({
      name: 'Audit',
      description: '',
      endDate: '2026-12-31',
      companyId: 'x',
      teamId: 'y'
    })
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})
