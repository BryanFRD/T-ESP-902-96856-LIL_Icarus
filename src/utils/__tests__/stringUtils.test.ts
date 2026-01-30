import { describe, expect, it } from 'vitest'

import { isEmptyString, isNotNull, isNull, validToken } from '../stringUtils.ts'

describe('stringUtils', () => {
  describe('isNull', () => {
    it('should return true for null values', () => {
      expect(isNull(null)).toBe(true)
    })

    it('should return true for undefined values', () => {
      expect(isNull(undefined)).toBe(true)
    })

    it('should return false for non-null values', () => {
      expect(isNull('hello')).toBe(false)
      expect(isNull(0)).toBe(false)
      expect(isNull(false)).toBe(false)
    })
  })

  describe('isNotNull', () => {
    it('should return false for null values', () => {
      expect(isNotNull(null)).toBe(false)
    })

    it('should return false for undefined values', () => {
      expect(isNotNull(undefined)).toBe(false)
    })

    it('should return true for non-null values', () => {
      expect(isNotNull('hello')).toBe(true)
      expect(isNotNull(0)).toBe(true)
      expect(isNotNull(false)).toBe(true)
    })
  })

  describe('isEmptyString', () => {
    it('should return true for null or undefined', () => {
      expect(isEmptyString(null)).toBe(true)
      expect(isEmptyString(undefined)).toBe(true)
    })

    it('should return true for empty string', () => {
      expect(isEmptyString('')).toBe(true)
    })

    it('should return true for whitespace-only string', () => {
      expect(isEmptyString('   ')).toBe(true)
      expect(isEmptyString('\t')).toBe(true)
    })

    it('should return false for non-empty string', () => {
      expect(isEmptyString('hello')).toBe(false)
      expect(isEmptyString('  hello  ')).toBe(false)
    })
  })

  describe('validToken', () => {
    it('should return true for valid tokens', () => {
      expect(validToken('abc123')).toBe(true)
      expect(validToken('token-with-dash')).toBe(true)
      expect(validToken('token_with_underscore')).toBe(true)
      expect(validToken('jwt.token.here')).toBe(true)
    })

    it('should return false for null or undefined', () => {
      expect(validToken(null)).toBe(false)
      expect(validToken(undefined)).toBe(false)
    })

    it('should return false for empty string', () => {
      expect(validToken('')).toBe(false)
      expect(validToken('   ')).toBe(false)
    })

    it('should return false for invalid token format', () => {
      expect(validToken('token with spaces')).toBe(false)
      expect(validToken('token.with.too.many.parts')).toBe(false)
    })
  })
})
