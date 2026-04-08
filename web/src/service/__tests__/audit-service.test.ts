import { HttpStatusCode } from 'axios'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { getAxiosClient } from '@/configuration/api/axios-client.ts'

import {
  checkAuditsAccess,
  createAudit,
  getUserAudits,
  verifyAuditAccess
} from '../audit-service.ts'

vi.mock('@/configuration/api/axios-client.ts', () => ({
  getAxiosClient: vi.fn()
}))

const mockClient = {
  get: vi.fn(),
  post: vi.fn()
}

describe('audit-service', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    vi.mocked(getAxiosClient).mockReturnValue(mockClient as never)
  })

  describe('checkAuditsAccess', () => {
    it('should GET /audits/check-access and return data', async () => {
      const mockData = { hasAccess: true, expiresAt: '2026-12-31T00:00:00Z' }
      mockClient.get.mockResolvedValue({ data: mockData })
      const result = await checkAuditsAccess()
      expect(mockClient.get).toHaveBeenCalledWith('/audits/check-access')
      expect(result).toEqual(mockData)
    })
  })

  describe('verifyAuditAccess', () => {
    it('should POST /audits/verify-access and return true on 200', async () => {
      mockClient.post.mockResolvedValue({ status: HttpStatusCode.Ok })
      const result = await verifyAuditAccess('password123')
      expect(mockClient.post).toHaveBeenCalledWith('/audits/verify-access', {
        password: 'password123'
      })
      expect(result).toBe(true)
    })

    it('should return false when status is not 200', async () => {
      mockClient.post.mockResolvedValue({ status: HttpStatusCode.Forbidden })
      const result = await verifyAuditAccess('wrong')
      expect(result).toBe(false)
    })
  })

  describe('createAudit', () => {
    it('should POST /audits/me with data and return response', async () => {
      const payload = {
        name: 'Audit 1',
        description: 'Test',
        endDate: '2026-12-31',
        companyId: '01HQXYZ',
        teamId: '01HQABC'
      }
      const mockResponse = { id: '1', ...payload }
      mockClient.post.mockResolvedValue({ data: mockResponse })
      const result = await createAudit(payload)
      expect(mockClient.post).toHaveBeenCalledWith('/audits/me', payload)
      expect(result).toEqual(mockResponse)
    })
  })

  describe('getUserAudits', () => {
    it('should GET /audits/me and return audits array', async () => {
      const mockAudits = [{ id: '1', name: 'Audit 1' }]
      mockClient.get.mockResolvedValue({ data: mockAudits })
      const result = await getUserAudits()
      expect(mockClient.get).toHaveBeenCalledWith('/audits/me')
      expect(result).toEqual(mockAudits)
    })
  })
})
