import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { verifyAuditAccess } from '@/service/audit-service.ts'

import { useVerifyAuditAccess } from '../use-verify-audit-access.ts'

vi.mock('@/service/audit-service.ts', () => ({
  verifyAuditAccess: vi.fn()
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

describe('useVerifyAuditAccess', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should call verifyAuditAccess on mutate', async () => {
    vi.mocked(verifyAuditAccess).mockResolvedValue(true)
    const { result } = renderHook(() => useVerifyAuditAccess(), {
      wrapper: createWrapper()
    })
    result.current.mutate('password123')
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(verifyAuditAccess).toHaveBeenCalledWith('password123')
  })
})
