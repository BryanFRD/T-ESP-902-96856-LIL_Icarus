import { describe, expect, it } from 'vitest'

import { uploadAvatarFileSchema } from '../upload-file-schema.ts'

describe('uploadAvatarFileSchema', () => {
  describe('valid data', () => {
    it('should validate a File instance', () => {
      const file = new File(['content'], 'avatar.jpg', { type: 'image/jpeg' })
      const validData = {
        avatar: file
      }

      const result = uploadAvatarFileSchema.safeParse(validData)
      expect(result.success).toBe(true)
    })

    it('should validate different file types', () => {
      const files = [
        new File(['content'], 'avatar.jpg', { type: 'image/jpeg' }),
        new File(['content'], 'avatar.png', { type: 'image/png' }),
        new File(['content'], 'avatar.gif', { type: 'image/gif' })
      ]

      files.forEach(file => {
        const result = uploadAvatarFileSchema.safeParse({ avatar: file })
        expect(result.success).toBe(true)
      })
    })
  })

  describe('invalid data', () => {
    it('should reject non-File instance', () => {
      const invalidData = {
        avatar: 'not-a-file'
      }

      const result = uploadAvatarFileSchema.safeParse(invalidData)
      expect(result.success).toBe(false)
    })

    it('should reject object that is not File', () => {
      const invalidData = {
        avatar: { name: 'avatar.jpg', size: 1024 }
      }

      const result = uploadAvatarFileSchema.safeParse(invalidData)
      expect(result.success).toBe(false)
    })

    it('should reject null', () => {
      const invalidData = {
        avatar: null
      }

      const result = uploadAvatarFileSchema.safeParse(invalidData)
      expect(result.success).toBe(false)
    })

    it('should reject undefined', () => {
      const invalidData = {
        avatar: undefined
      }

      const result = uploadAvatarFileSchema.safeParse(invalidData)
      expect(result.success).toBe(false)
    })
  })

  describe('missing fields', () => {
    it('should reject data without avatar field', () => {
      const result = uploadAvatarFileSchema.safeParse({})
      expect(result.success).toBe(false)
    })
  })
})
