import { getAxiosClient } from '@/configuration/api/axios-client.ts'
import type { UploadAvatarFile } from '@/domain/types/upload-file.ts'
import type { User } from '@/domain/types/user.ts'

export const getUser = async (id: string) => {
  return getAxiosClient()
    .get<User>(`/users/${id}`)
    .then(res => res.data)
}

export const getUsers = async () => {
  return getAxiosClient()
    .get<User[]>('/users')
    .then(res => res.data)
}

export const createUser = async (payload: User) => {
  return getAxiosClient()
    .post<User>('/users', payload)
    .then(res => res.data)
}

export const updateUser = async ({
  id,
  payload
}: {
  id: string
  payload: User
}) => {
  return getAxiosClient()
    .put<User>(`/users/${id}`, payload)
    .then(res => res.data)
}

export const patchUser = async ({
  id,
  payload
}: {
  id: string
  payload: Partial<User>
}) => {
  return getAxiosClient()
    .patch<User>(`/users/${id}`, payload)
    .then(res => res.data)
}

export const deleteUser = async (id: string) => {
  return getAxiosClient()
    .delete(`/users/${id}`)
    .then(() => {})
}

export const uploadAvatar = async (payload: UploadAvatarFile) => {
  const formData = new FormData()
  formData.append('avatar', payload.avatar)

  return getAxiosClient()
    .post<string>('/users/me/avatar', formData, {
      headers: {
        'Content-Type': 'multipart/form-data'
      }
    })
    .then(res => res.data)
}

export const getAvatar = async (filename: string): Promise<string> => {
  return getAxiosClient()
    .get(`/users/avatars/${filename}`, {
      responseType: 'blob'
    })
    .then(res => {
      return URL.createObjectURL(res.data)
    })
}
