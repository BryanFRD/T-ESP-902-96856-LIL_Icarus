import { describe, expect, it } from 'vitest'

import { UserRole } from './roles'

describe('UserRole', () => {
  describe('enum values', () => {
    it('should have CLIENT role with correct value', () => {
      expect(UserRole.CLIENT).toBe('Client')
    })

    it('should have PENTESTER role with correct value', () => {
      expect(UserRole.PENTESTER).toBe('Pentester')
    })

    it('should have ADMIN role with correct value', () => {
      expect(UserRole.ADMIN).toBe('Admin')
    })

    it('should have SUPER_ADMIN role with correct value', () => {
      expect(UserRole.SUPER_ADMIN).toBe('SuperAdmin')
    })
  })

  describe('enum structure', () => {
    it('should have exactly 4 roles', () => {
      const roles = Object.keys(UserRole)
      expect(roles).toHaveLength(4)
    })

    it('should contain all expected role keys', () => {
      const roles = Object.keys(UserRole)
      expect(roles).toContain('CLIENT')
      expect(roles).toContain('PENTESTER')
      expect(roles).toContain('ADMIN')
      expect(roles).toContain('SUPER_ADMIN')
    })

    it('should contain all expected role values', () => {
      const roleValues = Object.values(UserRole)
      expect(roleValues).toContain('Client')
      expect(roleValues).toContain('Pentester')
      expect(roleValues).toContain('Admin')
      expect(roleValues).toContain('SuperAdmin')
    })
  })

  describe('type safety', () => {
    it('should accept valid UserRole values', () => {
      const role: UserRole = UserRole.CLIENT
      expect(role).toBe('Client')
    })

    it('should work with all role values in array', () => {
      const roles: UserRole[] = [
        UserRole.CLIENT,
        UserRole.PENTESTER,
        UserRole.ADMIN,
        UserRole.SUPER_ADMIN
      ]
      expect(roles).toHaveLength(4)
    })
  })
})
