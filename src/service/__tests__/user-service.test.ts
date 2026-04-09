import { beforeEach, describe, expect, it, vi } from 'vitest'

import { getAxiosClient } from '@/configuration/api/axios-client.ts'

import {
  createUser,
  deleteUser,
  getUser,
  getUsers,
  patchUser,
  updateUser,
  uploadAvatar
} from '../user-service.ts'

vi.mock('@/configuration/api/axios-client.ts', () => ({
  getAxiosClient: vi.fn()
}))

const mockClient = {
  get: vi.fn(),
  post: vi.fn(),
  put: vi.fn(),
  patch: vi.fn(),
  delete: vi.fn()
}

describe('user-service', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    vi.mocked(getAxiosClient).mockReturnValue(mockClient as never)
  })

  it('getUser should GET /users/:id', async () => {
    const mockUser = { id: 'u1', username: 'john' }
    mockClient.get.mockResolvedValue({ data: mockUser })
    const result = await getUser('u1')
    expect(mockClient.get).toHaveBeenCalledWith('/users/u1')
    expect(result).toEqual(mockUser)
  })

  it('getUsers should GET /users', async () => {
    const mockUsers = [{ id: 'u1' }, { id: 'u2' }]
    mockClient.get.mockResolvedValue({ data: mockUsers })
    const result = await getUsers()
    expect(mockClient.get).toHaveBeenCalledWith('/users')
    expect(result).toEqual(mockUsers)
  })

  it('createUser should POST /users', async () => {
    const payload = {
      id: '',
      username: 'john',
      firstName: 'John',
      lastName: 'Doe',
      email: 'j@t.com',
      role: 'USER' as const,
      createdAt: '',
      updatedAt: ''
    }
    mockClient.post.mockResolvedValue({ data: payload })
    const result = await createUser(payload as never)
    expect(mockClient.post).toHaveBeenCalledWith('/users', payload)
    expect(result).toEqual(payload)
  })

  it('updateUser should PUT /users/:id', async () => {
    const payload = {
      id: 'u1',
      username: 'john',
      firstName: 'John',
      lastName: 'Doe',
      email: 'j@t.com',
      role: 'USER' as const,
      createdAt: '',
      updatedAt: ''
    }
    mockClient.put.mockResolvedValue({ data: payload })
    const result = await updateUser({ id: 'u1', payload: payload as never })
    expect(mockClient.put).toHaveBeenCalledWith('/users/u1', payload)
    expect(result).toEqual(payload)
  })

  it('patchUser should PATCH /users/:id', async () => {
    const payload = { firstName: 'Jane' }
    const mockUser = { id: 'u1', firstName: 'Jane' }
    mockClient.patch.mockResolvedValue({ data: mockUser })
    const result = await patchUser({ id: 'u1', payload })
    expect(mockClient.patch).toHaveBeenCalledWith('/users/u1', payload)
    expect(result).toEqual(mockUser)
  })

  it('deleteUser should DELETE /users/:id', async () => {
    mockClient.delete.mockResolvedValue({})
    await deleteUser('u1')
    expect(mockClient.delete).toHaveBeenCalledWith('/users/u1')
  })

  it('uploadAvatar should POST /users/me/avatar with FormData', async () => {
    const mockFile = new File(['content'], 'avatar.png', { type: 'image/png' })
    mockClient.post.mockResolvedValue({ data: 'avatar-url.png' })
    const result = await uploadAvatar({ avatar: mockFile })
    expect(mockClient.post).toHaveBeenCalledWith(
      '/users/me/avatar',
      expect.any(FormData),
      { headers: { 'Content-Type': 'multipart/form-data' } }
    )
    expect(result).toBe('avatar-url.png')
  })
})
