import { HttpStatusCode } from 'axios'

import { getAxiosClient } from '@/api/axiosClient'
import type {
  LoginPayload,
  RegisterPayload,
  ResetPasswordPayload,
  SendResetPasswordEmailPayload
} from '@/domain/commonTypes/authType'
import type { User } from '@/domain/commonTypes/userTypes.ts'

export const register = async (payload: RegisterPayload) => {
  return getAxiosClient()
    .post('/auth/register', payload)
    .then(() => {})
}

export const login = async (payload: LoginPayload) => {
  return getAxiosClient()
    .post('/auth/login', payload)
    .then(() => {})
}

export const logout = async () => {
  const res = await getAxiosClient().post('/auth/logout')
  return res.status === HttpStatusCode.Ok
}

export const patchMe = async (payload: Partial<User>) => {
  return await getAxiosClient()
    .patch<User>('/users/me', payload)
    .then(res => res.data)
}

export const me = async () => {
  const res = await getAxiosClient().get<User>('/users/me')
  return res.data
}

export const sendPasswordResetEmail = async (
  payload: SendResetPasswordEmailPayload
) => {
  return getAxiosClient()
    .post('/auth/request-password-reset', { resetEmail: payload.resetEmail })
    .then(() => {})
}

export const resetPassword = async (payload: ResetPasswordPayload) => {
  return getAxiosClient()
    .post('/auth/reset-password', {
      token: payload.token,
      newPassword: payload.newPassword
    })
    .then(() => {})
}

export const confirmEmail = async (token: string) => {
  const response = await getAxiosClient().post('/auth/confirm-email', { token })
  return response.status === HttpStatusCode.Created
}
