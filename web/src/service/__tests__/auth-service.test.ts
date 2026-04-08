import { HttpStatusCode } from 'axios'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { getAxiosClient } from '@/configuration/api/axios-client.ts'

import {
  changePassword,
  confirmEmail,
  login,
  logout,
  me,
  patchMe,
  register,
  resetPassword,
  sendPasswordResetEmail
} from '../auth-service.ts'

vi.mock('@/configuration/api/axios-client.ts', () => ({
  getAxiosClient: vi.fn()
}))

const mockClient = {
  get: vi.fn(),
  post: vi.fn(),
  patch: vi.fn()
}

describe('auth-service', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    vi.mocked(getAxiosClient).mockReturnValue(mockClient as never)
  })

  describe('register', () => {
    it('should POST to /auth/register with payload', async () => {
      mockClient.post.mockResolvedValue({ data: {} })
      const payload = {
        username: 'john',
        firstName: 'John',
        lastName: 'Doe',
        email: 'john@test.com',
        password: 'password123'
      }
      await register(payload)
      expect(mockClient.post).toHaveBeenCalledWith('/auth/register', payload)
    })
  })

  describe('login', () => {
    it('should POST to /auth/login with payload', async () => {
      mockClient.post.mockResolvedValue({ data: {} })
      const payload = { email: 'john@test.com', password: 'password123' }
      await login(payload)
      expect(mockClient.post).toHaveBeenCalledWith('/auth/login', payload)
    })
  })

  describe('logout', () => {
    it('should POST to /auth/logout and return true on 200', async () => {
      mockClient.post.mockResolvedValue({ status: HttpStatusCode.Ok })
      const result = await logout()
      expect(mockClient.post).toHaveBeenCalledWith('/auth/logout')
      expect(result).toBe(true)
    })

    it('should return false when status is not 200', async () => {
      mockClient.post.mockResolvedValue({
        status: HttpStatusCode.InternalServerError
      })
      const result = await logout()
      expect(result).toBe(false)
    })
  })

  describe('patchMe', () => {
    it('should PATCH /users/me with payload', async () => {
      mockClient.patch.mockResolvedValue({ data: {} })
      const payload = { firstName: 'Jane' }
      await patchMe(payload)
      expect(mockClient.patch).toHaveBeenCalledWith('/users/me', payload)
    })
  })

  describe('me', () => {
    it('should GET /users/me and return user data', async () => {
      const mockUser = { id: '1', username: 'john', email: 'john@test.com' }
      mockClient.get.mockResolvedValue({ data: mockUser })
      const result = await me()
      expect(mockClient.get).toHaveBeenCalledWith('/users/me')
      expect(result).toEqual(mockUser)
    })
  })

  describe('sendPasswordResetEmail', () => {
    it('should POST to /auth/request-password-reset', async () => {
      mockClient.post.mockResolvedValue({ data: {} })
      await sendPasswordResetEmail({ resetEmail: 'john@test.com' })
      expect(mockClient.post).toHaveBeenCalledWith(
        '/auth/request-password-reset',
        {
          resetEmail: 'john@test.com'
        }
      )
    })
  })

  describe('resetPassword', () => {
    it('should POST to /auth/reset-password with token and new password', async () => {
      mockClient.post.mockResolvedValue({ data: {} })
      await resetPassword({ token: 'abc123', newPassword: 'newpass' })
      expect(mockClient.post).toHaveBeenCalledWith('/auth/reset-password', {
        token: 'abc123',
        newPassword: 'newpass'
      })
    })
  })

  describe('changePassword', () => {
    it('should PATCH /users/me with password payload', async () => {
      mockClient.patch.mockResolvedValue({ data: {} })
      const payload = {
        password: 'old',
        newPassword: 'new',
        confirmPassword: 'new'
      }
      await changePassword(payload)
      expect(mockClient.patch).toHaveBeenCalledWith('/users/me', payload)
    })
  })

  describe('confirmEmail', () => {
    it('should POST to /auth/confirm-email and return true on 201', async () => {
      mockClient.post.mockResolvedValue({ status: HttpStatusCode.Created })
      const result = await confirmEmail('token123')
      expect(mockClient.post).toHaveBeenCalledWith('/auth/confirm-email', {
        token: 'token123'
      })
      expect(result).toBe(true)
    })

    it('should return false when status is not 201', async () => {
      mockClient.post.mockResolvedValue({ status: HttpStatusCode.Ok })
      const result = await confirmEmail('token123')
      expect(result).toBe(false)
    })
  })
})
