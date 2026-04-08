import { beforeEach, describe, expect, it, vi } from 'vitest'

import { getAxiosClient } from '@/configuration/api/axios-client.ts'

import { status } from '../server-service.ts'

vi.mock('@/configuration/api/axios-client.ts', () => ({
  getAxiosClient: vi.fn()
}))

const mockClient = {
  get: vi.fn()
}

describe('server-service', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    vi.mocked(getAxiosClient).mockReturnValue(mockClient as never)
  })

  it('should GET /status and return server data', async () => {
    const mockStatus = { status: 'RUNNING', message: 'OK' }
    mockClient.get.mockResolvedValue({ data: mockStatus })
    const result = await status()
    expect(mockClient.get).toHaveBeenCalledWith('/status')
    expect(result).toEqual(mockStatus)
  })
})
