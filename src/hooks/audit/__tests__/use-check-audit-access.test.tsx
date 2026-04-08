import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { checkAuditsAccess } from '@/service/audit-service.ts'

import { useCheckAuditAccess } from '../use-check-audit-access.ts'

vi.mock('@/service/audit-service.ts', () => ({
  checkAuditsAccess: vi.fn()
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

describe('useCheckAuditAccess', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should fetch audit access', async () => {
    const mockAccess = { hasAccess: true, expiresAt: '2026-12-31T00:00:00Z' }
    vi.mocked(checkAuditsAccess).mockResolvedValue(mockAccess)
    const { result } = renderHook(() => useCheckAuditAccess(), {
      wrapper: createWrapper()
    })
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(result.current.data).toEqual(mockAccess)
  })

  it('should handle error', async () => {
    vi.mocked(checkAuditsAccess).mockRejectedValue(new Error('fail'))
    const { result } = renderHook(() => useCheckAuditAccess(), {
      wrapper: createWrapper()
    })
    await waitFor(() => expect(result.current.isError).toBe(true))
  })
})
