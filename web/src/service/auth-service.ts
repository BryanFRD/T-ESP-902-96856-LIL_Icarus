import { HttpStatusCode } from 'axios'

import { getAxiosClient } from '@/configuration/api/axios-client.ts'
import type {
  LoginPayload,
  RegisterPayload,
  ResetPasswordPayload,
  SendResetPasswordEmailPayload
} from '@/domain/types/authentication.ts'
import type { User } from '@/domain/types/user.ts'

export interface ChangePasswordPayload {
  password: string
  newPassword: string
  confirmPassword: string
}

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
    .patch('/users/me', payload)
    .then(() => {})
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

export const changePassword = async (payload: ChangePasswordPayload) => {
  return getAxiosClient()
    .patch('/users/me', payload)
    .then(() => {})
}

export const confirmEmail = async (token: string) => {
  const response = await getAxiosClient().post('/auth/confirm-email', { token })
  return response.status === HttpStatusCode.Created
}
