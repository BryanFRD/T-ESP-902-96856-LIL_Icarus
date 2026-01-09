import { describe, expect, it } from 'vitest'

import { ServerStatusEnum } from './serverStatusEnum'

describe('ServerStatusEnum', () => {
  describe('enum values', () => {
    it('should have Up status with correct value', () => {
      expect(ServerStatusEnum.Up).toBe('Up')
    })

    it('should have Maintenance status with correct value', () => {
      expect(ServerStatusEnum.Maintenance).toBe('Maintenance')
    })

    it('should have DOWN status with correct value', () => {
      expect(ServerStatusEnum.DOWN).toBe('Down')
    })
  })

  describe('enum structure', () => {
    it('should have exactly 3 statuses', () => {
      const statuses = Object.keys(ServerStatusEnum)
      expect(statuses).toHaveLength(3)
    })

    it('should contain all expected status keys', () => {
      const statuses = Object.keys(ServerStatusEnum)
      expect(statuses).toContain('Up')
      expect(statuses).toContain('Maintenance')
      expect(statuses).toContain('DOWN')
    })

    it('should contain all expected status values', () => {
      const statusValues = Object.values(ServerStatusEnum)
      expect(statusValues).toContain('Up')
      expect(statusValues).toContain('Maintenance')
      expect(statusValues).toContain('Down')
    })
  })

  describe('type safety', () => {
    it('should accept valid ServerStatusEnum values', () => {
      const status: ServerStatusEnum = ServerStatusEnum.Up
      expect(status).toBe('Up')
    })

    it('should work with all status values in array', () => {
      const statuses: ServerStatusEnum[] = [
        ServerStatusEnum.Up,
        ServerStatusEnum.Maintenance,
        ServerStatusEnum.DOWN
      ]
      expect(statuses).toHaveLength(3)
    })
  })
})
