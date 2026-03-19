import { describe, expect, it } from 'vitest'

import { getLoginSchema } from '../login-schema.ts'

describe('loginSchema', () => {
  const t = (key: string) => key
  const schema = getLoginSchema(t)

  describe('valid data', () => {
    it('should validate correct login data', () => {
      const validData = {
        email: 'test@example.com',
        password: 'password123'
      }

      const result = schema.safeParse(validData)
      expect(result.success).toBe(true)
    })

    it('should validate email with various formats', () => {
      const emails = [
        'user@example.com',
        'user.name@example.com',
        'user+tag@example.co.uk',
        'user_123@test-domain.com'
      ]

      emails.forEach(email => {
        const result = schema.safeParse({
          email,
          password: 'password123'
        })
        expect(result.success).toBe(true)
      })
    })
  })

  describe('invalid email', () => {
    it('should reject invalid email format', () => {
      const invalidData = {
        email: 'invalid-email',
        password: 'password123'
      }

      const result = schema.safeParse(invalidData)
      expect(result.success).toBe(false)
      if (!result.success) {
        expect(result.error.issues[0].path).toContain('email')
        expect(result.error.issues[0].message).toBe('email.invalid')
      }
    })

    it('should reject empty email', () => {
      const invalidData = {
        email: '',
        password: 'password123'
      }

      const result = schema.safeParse(invalidData)
      expect(result.success).toBe(false)
    })

    it('should reject email without @', () => {
      const invalidData = {
        email: 'testexample.com',
        password: 'password123'
      }

      const result = schema.safeParse(invalidData)
      expect(result.success).toBe(false)
    })
  })

  describe('invalid password', () => {
    it('should reject password shorter than 8 characters', () => {
      const invalidData = {
        email: 'test@example.com',
        password: 'short'
      }

      const result = schema.safeParse(invalidData)
      expect(result.success).toBe(false)
      if (!result.success) {
        expect(result.error.issues[0].path).toContain('password')
        expect(result.error.issues[0].message).toBe('password.min')
      }
    })

    it('should reject empty password', () => {
      const invalidData = {
        email: 'test@example.com',
        password: ''
      }

      const result = schema.safeParse(invalidData)
      expect(result.success).toBe(false)
    })

    it('should accept password with exactly 8 characters', () => {
      const validData = {
        email: 'test@example.com',
        password: '12345678'
      }

      const result = schema.safeParse(validData)
      expect(result.success).toBe(true)
    })
  })

  describe('missing fields', () => {
    it('should reject data without email', () => {
      const invalidData = {
        password: 'password123'
      }

      const result = schema.safeParse(invalidData)
      expect(result.success).toBe(false)
    })

    it('should reject data without password', () => {
      const invalidData = {
        email: 'test@example.com'
      }

      const result = schema.safeParse(invalidData)
      expect(result.success).toBe(false)
    })

    it('should reject empty object', () => {
      const result = schema.safeParse({})
      expect(result.success).toBe(false)
    })
  })
})
