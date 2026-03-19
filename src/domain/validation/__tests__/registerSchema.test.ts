import { describe, expect, it } from 'vitest'

import { getRegisterSchema } from '../register-schema.ts'

describe('registerSchema', () => {
  const t = (key: string) => key
  const schema = getRegisterSchema(t)

  describe('valid data', () => {
    it('should validate correct registration data', () => {
      const validData = {
        username: 'johndoe',
        email: 'john@example.com',
        firstName: 'John',
        lastName: 'Doe',
        password: 'password123',
        confirmPassword: 'password123'
      }

      const result = schema.safeParse(validData)
      expect(result.success).toBe(true)
    })
  })

  describe('invalid username', () => {
    it('should reject empty username', () => {
      const invalidData = {
        username: '',
        email: 'john@example.com',
        firstName: 'John',
        lastName: 'Doe',
        password: 'password123',
        confirmPassword: 'password123'
      }

      const result = schema.safeParse(invalidData)
      expect(result.success).toBe(false)
      if (!result.success) {
        expect(result.error.issues[0].path).toContain('username')
        expect(result.error.issues[0].message).toBe(
          'inputField.usernameRequired'
        )
      }
    })
  })

  describe('invalid email', () => {
    it('should reject invalid email format', () => {
      const invalidData = {
        username: 'johndoe',
        email: 'invalid-email',
        firstName: 'John',
        lastName: 'Doe',
        password: 'password123',
        confirmPassword: 'password123'
      }

      const result = schema.safeParse(invalidData)
      expect(result.success).toBe(false)
      if (!result.success) {
        expect(result.error.issues[0].path).toContain('email')
        expect(result.error.issues[0].message).toBe('email.invalid')
      }
    })
  })

  describe('invalid firstName', () => {
    it('should reject empty firstName', () => {
      const invalidData = {
        username: 'johndoe',
        email: 'john@example.com',
        firstName: '',
        lastName: 'Doe',
        password: 'password123',
        confirmPassword: 'password123'
      }

      const result = schema.safeParse(invalidData)
      expect(result.success).toBe(false)
      if (!result.success) {
        expect(result.error.issues[0].path).toContain('firstName')
        expect(result.error.issues[0].message).toBe(
          'inputField.firstNameRequired'
        )
      }
    })
  })

  describe('invalid lastName', () => {
    it('should reject empty lastName', () => {
      const invalidData = {
        username: 'johndoe',
        email: 'john@example.com',
        firstName: 'John',
        lastName: '',
        password: 'password123',
        confirmPassword: 'password123'
      }

      const result = schema.safeParse(invalidData)
      expect(result.success).toBe(false)
      if (!result.success) {
        expect(result.error.issues[0].path).toContain('lastName')
        expect(result.error.issues[0].message).toBe(
          'inputField.lastNameRequired'
        )
      }
    })
  })

  describe('invalid password', () => {
    it('should reject password shorter than 8 characters', () => {
      const invalidData = {
        username: 'johndoe',
        email: 'john@example.com',
        firstName: 'John',
        lastName: 'Doe',
        password: 'short',
        confirmPassword: 'short'
      }

      const result = schema.safeParse(invalidData)
      expect(result.success).toBe(false)
      if (!result.success) {
        expect(result.error.issues[0].path).toContain('password')
        expect(result.error.issues[0].message).toBe('password.min')
      }
    })
  })

  describe('password mismatch', () => {
    it('should reject when passwords do not match', () => {
      const invalidData = {
        username: 'johndoe',
        email: 'john@example.com',
        firstName: 'John',
        lastName: 'Doe',
        password: 'password123',
        confirmPassword: 'differentpassword'
      }

      const result = schema.safeParse(invalidData)
      expect(result.success).toBe(false)
      if (!result.success) {
        const confirmPasswordIssue = result.error.issues.find(issue =>
          issue.path.includes('confirmPassword')
        )
        expect(confirmPasswordIssue).toBeDefined()
        expect(confirmPasswordIssue?.message).toBe('password.mismatch')
      }
    })

    it('should accept when passwords match', () => {
      const validData = {
        username: 'johndoe',
        email: 'john@example.com',
        firstName: 'John',
        lastName: 'Doe',
        password: 'password123',
        confirmPassword: 'password123'
      }

      const result = schema.safeParse(validData)
      expect(result.success).toBe(true)
    })
  })

  describe('missing fields', () => {
    it('should reject data with missing required fields', () => {
      const invalidData = {
        username: 'johndoe',
        email: 'john@example.com'
      }

      const result = schema.safeParse(invalidData)
      expect(result.success).toBe(false)
    })
  })
})
