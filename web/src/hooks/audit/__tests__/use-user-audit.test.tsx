import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { getUserAudits } from '@/service/audit-service.ts'

import { useUserAudit } from '../use-user-audit.ts'

vi.mock('@/service/audit-service.ts', () => ({
  getUserAudits: vi.fn()
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

describe('useUserAudit', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should fetch user audits', async () => {
    const mockAudits = [{ id: '1', name: 'Audit 1' }]
    vi.mocked(getUserAudits).mockResolvedValue(mockAudits as never)
    const { result } = renderHook(() => useUserAudit(), {
      wrapper: createWrapper()
    })
    await waitFor(() => expect(result.current.isSuccess).toBe(true))
    expect(result.current.data).toEqual(mockAudits)
  })
})
