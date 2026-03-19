import { getAxiosClient } from '@/configuration/api/axios-client.ts'
import type {
  Company,
  CreateCompanyPayload,
  CreateInvitationPayload,
  CreateTeamPayload,
  Invitation,
  PatchCompanyPayload,
  PatchTeamPayload,
  Team,
  TeamMember,
  TeamWithDetails,
  UpdateMemberRolePayload
} from '@/domain/types/team.ts'

// Teams
export const getMyTeams = async () => {
  return getAxiosClient()
    .get<TeamWithDetails[]>('/teams/me')
    .then(res => res.data)
}

export const createTeam = async (payload: CreateTeamPayload) => {
  return getAxiosClient()
    .post<Team>('/teams/me', payload)
    .then(res => res.data)
}

export const patchTeam = async ({
  teamId,
  payload
}: {
  teamId: string
  payload: PatchTeamPayload
}) => {
  return getAxiosClient()
    .patch<Team>(`/teams/${teamId}`, payload)
    .then(res => res.data)
}

export const deleteTeam = async (teamId: string) => {
  return getAxiosClient()
    .delete(`/teams/${teamId}`)
    .then(() => {})
}

// Companies
export const getTeamCompanies = async (teamId: string) => {
  return getAxiosClient()
    .get<Company[]>(`/teams/${teamId}/company`)
    .then(res => res.data)
}

export const createCompany = async ({
  teamId,
  payload
}: {
  teamId: string
  payload: CreateCompanyPayload
}) => {
  return getAxiosClient()
    .post<Company>(`/teams/${teamId}/company`, payload)
    .then(res => res.data)
}

export const patchCompany = async ({
  teamId,
  companyId,
  payload
}: {
  teamId: string
  companyId: string
  payload: PatchCompanyPayload
}) => {
  return getAxiosClient()
    .patch<Company>(`/teams/${teamId}/company/${companyId}`, payload)
    .then(res => res.data)
}

export const deleteCompany = async ({
  teamId,
  companyId
}: {
  teamId: string
  companyId: string
}) => {
  return getAxiosClient()
    .delete(`/teams/${teamId}/company/${companyId}`)
    .then(() => {})
}

// Invitations
export const getTeamInvitations = async (teamId: string) => {
  return getAxiosClient()
    .get<Invitation[]>(`/teams/${teamId}/invitations`)
    .then(res => res.data)
}

export const createInvitation = async ({
  teamId,
  payload
}: {
  teamId: string
  payload: CreateInvitationPayload
}) => {
  return getAxiosClient()
    .post<Invitation>(`/teams/${teamId}/invitations`, payload)
    .then(res => res.data)
}

export const cancelInvitation = async ({
  teamId,
  invitationId
}: {
  teamId: string
  invitationId: string
}) => {
  return getAxiosClient()
    .delete(`/teams/${teamId}/invitations/${invitationId}`)
    .then(() => {})
}

export const joinTeam = async (token: string) => {
  return getAxiosClient()
    .post(`/teams/invitations/join/${token}`)
    .then(() => {})
}

// Members
export const removeMember = async ({
  teamId,
  memberId
}: {
  teamId: string
  memberId: string
}) => {
  return getAxiosClient()
    .delete(`/teams/${teamId}/members/${memberId}`)
    .then(() => {})
}

export const updateMemberRole = async ({
  teamId,
  memberId,
  payload
}: {
  teamId: string
  memberId: string
  payload: UpdateMemberRolePayload
}) => {
  return getAxiosClient()
    .patch<TeamMember>(`/teams/${teamId}/members/${memberId}`, payload)
    .then(res => res.data)
}
