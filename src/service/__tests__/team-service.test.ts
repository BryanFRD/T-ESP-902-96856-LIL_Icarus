import { beforeEach, describe, expect, it, vi } from 'vitest'

import { getAxiosClient } from '@/configuration/api/axios-client.ts'

import {
  cancelInvitation,
  createCompany,
  createInvitation,
  createTeam,
  deleteCompany,
  deleteTeam,
  getMyTeams,
  getTeamCompanies,
  getTeamInvitations,
  joinTeam,
  patchCompany,
  patchTeam,
  removeMember,
  updateMemberRole
} from '../team-service.ts'

vi.mock('@/configuration/api/axios-client.ts', () => ({
  getAxiosClient: vi.fn()
}))

const mockClient = {
  get: vi.fn(),
  post: vi.fn(),
  patch: vi.fn(),
  delete: vi.fn()
}

describe('team-service', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    vi.mocked(getAxiosClient).mockReturnValue(mockClient as never)
  })

  describe('teams', () => {
    it('getMyTeams should GET /teams/me', async () => {
      const mockTeams = [{ id: '1', name: 'Team 1' }]
      mockClient.get.mockResolvedValue({ data: mockTeams })
      const result = await getMyTeams()
      expect(mockClient.get).toHaveBeenCalledWith('/teams/me')
      expect(result).toEqual(mockTeams)
    })

    it('createTeam should POST /teams/me', async () => {
      const payload = { name: 'New Team', description: 'Desc' }
      const mockTeam = { id: '1', ...payload }
      mockClient.post.mockResolvedValue({ data: mockTeam })
      const result = await createTeam(payload)
      expect(mockClient.post).toHaveBeenCalledWith('/teams/me', payload)
      expect(result).toEqual(mockTeam)
    })

    it('patchTeam should PATCH /teams/:teamId', async () => {
      const payload = { name: 'Updated' }
      const mockTeam = { id: 't1', name: 'Updated' }
      mockClient.patch.mockResolvedValue({ data: mockTeam })
      const result = await patchTeam({ teamId: 't1', payload })
      expect(mockClient.patch).toHaveBeenCalledWith('/teams/t1', payload)
      expect(result).toEqual(mockTeam)
    })

    it('deleteTeam should DELETE /teams/:teamId', async () => {
      mockClient.delete.mockResolvedValue({})
      await deleteTeam('t1')
      expect(mockClient.delete).toHaveBeenCalledWith('/teams/t1')
    })
  })

  describe('companies', () => {
    it('getTeamCompanies should GET /teams/:teamId/company', async () => {
      const mockCompanies = [{ id: 'c1', name: 'Corp' }]
      mockClient.get.mockResolvedValue({ data: mockCompanies })
      const result = await getTeamCompanies('t1')
      expect(mockClient.get).toHaveBeenCalledWith('/teams/t1/company')
      expect(result).toEqual(mockCompanies)
    })

    it('createCompany should POST /teams/:teamId/company', async () => {
      const payload = { name: 'Corp', siret: '12345678901234' }
      const mockCompany = { id: 'c1', ...payload }
      mockClient.post.mockResolvedValue({ data: mockCompany })
      const result = await createCompany({ teamId: 't1', payload })
      expect(mockClient.post).toHaveBeenCalledWith('/teams/t1/company', payload)
      expect(result).toEqual(mockCompany)
    })

    it('patchCompany should PATCH /teams/:teamId/company/:companyId', async () => {
      const payload = { name: 'Updated Corp' }
      const mockCompany = { id: 'c1', name: 'Updated Corp' }
      mockClient.patch.mockResolvedValue({ data: mockCompany })
      const result = await patchCompany({
        teamId: 't1',
        companyId: 'c1',
        payload
      })
      expect(mockClient.patch).toHaveBeenCalledWith(
        '/teams/t1/company/c1',
        payload
      )
      expect(result).toEqual(mockCompany)
    })

    it('deleteCompany should DELETE /teams/:teamId/company/:companyId', async () => {
      mockClient.delete.mockResolvedValue({})
      await deleteCompany({ teamId: 't1', companyId: 'c1' })
      expect(mockClient.delete).toHaveBeenCalledWith('/teams/t1/company/c1')
    })
  })

  describe('invitations', () => {
    it('getTeamInvitations should GET /teams/:teamId/invitations', async () => {
      const mockInvitations = [{ id: 'i1', token: 'abc' }]
      mockClient.get.mockResolvedValue({ data: mockInvitations })
      const result = await getTeamInvitations('t1')
      expect(mockClient.get).toHaveBeenCalledWith('/teams/t1/invitations')
      expect(result).toEqual(mockInvitations)
    })

    it('createInvitation should POST /teams/:teamId/invitations', async () => {
      const payload = { maxUses: 5, expiresAt: null }
      const mockInvitation = { id: 'i1', ...payload }
      mockClient.post.mockResolvedValue({ data: mockInvitation })
      const result = await createInvitation({ teamId: 't1', payload })
      expect(mockClient.post).toHaveBeenCalledWith(
        '/teams/t1/invitations',
        payload
      )
      expect(result).toEqual(mockInvitation)
    })

    it('cancelInvitation should DELETE /teams/:teamId/invitations/:invitationId', async () => {
      mockClient.delete.mockResolvedValue({})
      await cancelInvitation({ teamId: 't1', invitationId: 'i1' })
      expect(mockClient.delete).toHaveBeenCalledWith('/teams/t1/invitations/i1')
    })

    it('joinTeam should POST /teams/invitations/join/:token', async () => {
      mockClient.post.mockResolvedValue({})
      await joinTeam('abc123')
      expect(mockClient.post).toHaveBeenCalledWith(
        '/teams/invitations/join/abc123'
      )
    })
  })

  describe('members', () => {
    it('removeMember should DELETE /teams/:teamId/members/:memberId', async () => {
      mockClient.delete.mockResolvedValue({})
      await removeMember({ teamId: 't1', memberId: 'm1' })
      expect(mockClient.delete).toHaveBeenCalledWith('/teams/t1/members/m1')
    })

    it('updateMemberRole should PATCH /teams/:teamId/members/:memberId', async () => {
      const payload = { role: 'MODERATOR' as const }
      const mockMember = { id: 'm1', teamRole: 'MODERATOR' }
      mockClient.patch.mockResolvedValue({ data: mockMember })
      const result = await updateMemberRole({
        teamId: 't1',
        memberId: 'm1',
        payload
      })
      expect(mockClient.patch).toHaveBeenCalledWith(
        '/teams/t1/members/m1',
        payload
      )
      expect(result).toEqual(mockMember)
    })
  })
})
