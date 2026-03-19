import { describe, expect, it } from 'vitest'

import {
  getNewPasswordSchema,
  getPasswordSchema
} from '../reset-password-schema.ts'

describe('resetPasswordSchema', () => {
  const t = (key: string) => key

  describe('getPasswordSchema', () => {
    const schema = getPasswordSchema(t)

    describe('valid data', () => {
      it('should validate correct reset email', () => {
        const validData = {
          resetEmail: 'test@example.com'
        }

        const result = schema.safeParse(validData)
        expect(result.success).toBe(true)
      })
    })

    describe('invalid email', () => {
      it('should reject invalid email format', () => {
        const invalidData = {
          resetEmail: 'invalid-email'
        }

        const result = schema.safeParse(invalidData)
        expect(result.success).toBe(false)
        if (!result.success) {
          expect(result.error.issues[0].path).toContain('resetEmail')
          expect(result.error.issues[0].message).toBe('email.invalid')
        }
      })

      it('should reject empty email', () => {
        const invalidData = {
          resetEmail: ''
        }

        const result = schema.safeParse(invalidData)
        expect(result.success).toBe(false)
        if (!result.success) {
          const emailRequiredIssue = result.error.issues.find(
            issue => issue.message === 'inputField.emailRequired'
          )
          expect(emailRequiredIssue).toBeDefined()
        }
      })
    })

    describe('missing fields', () => {
      it('should reject data without resetEmail', () => {
        const result = schema.safeParse({})
        expect(result.success).toBe(false)
      })
    })
  })

  describe('getNewPasswordSchema', () => {
    const schema = getNewPasswordSchema(t)

    describe('valid data', () => {
      it('should validate correct new password', () => {
        const validData = {
          newPassword: 'newpassword123'
        }

        const result = schema.safeParse(validData)
        expect(result.success).toBe(true)
      })

      it('should accept password with exactly 8 characters', () => {
        const validData = {
          newPassword: '12345678'
        }

        const result = schema.safeParse(validData)
        expect(result.success).toBe(true)
      })
    })

    describe('invalid password', () => {
      it('should reject password shorter than 8 characters', () => {
        const invalidData = {
          newPassword: 'short'
        }

        const result = schema.safeParse(invalidData)
        expect(result.success).toBe(false)
        if (!result.success) {
          expect(result.error.issues[0].path).toContain('newPassword')
          expect(result.error.issues[0].message).toBe('password.min')
        }
      })

      it('should reject empty password', () => {
        const invalidData = {
          newPassword: ''
        }

        const result = schema.safeParse(invalidData)
        expect(result.success).toBe(false)
      })
    })

    describe('missing fields', () => {
      it('should reject data without newPassword', () => {
        const result = schema.safeParse({})
        expect(result.success).toBe(false)
      })
    })
  })
})
