import { describe, expect, it } from 'vitest'

import { ServerStatus, type ServerStatusEnum } from '../ServerStatus.ts'

describe('ServerStatus', () => {
  describe('enum values', () => {
    it('should have Up status with correct value', () => {
      expect(ServerStatus.Up).toBe('Up')
    })

    it('should have Maintenance status with correct value', () => {
      expect(ServerStatus.Maintenance).toBe('Maintenance')
    })

    it('should have DOWN status with correct value', () => {
      expect(ServerStatus.DOWN).toBe('Down')
    })
  })

  describe('enum structure', () => {
    it('should have exactly 3 statuses', () => {
      const statuses = Object.keys(ServerStatus)
      expect(statuses).toHaveLength(3)
    })

    it('should contain all expected status keys', () => {
      const statuses = Object.keys(ServerStatus)
      expect(statuses).toContain('Up')
      expect(statuses).toContain('Maintenance')
      expect(statuses).toContain('DOWN')
    })

    it('should contain all expected status values', () => {
      const statusValues = Object.values(ServerStatus)
      expect(statusValues).toContain('Up')
      expect(statusValues).toContain('Maintenance')
      expect(statusValues).toContain('Down')
    })
  })

  describe('types safety', () => {
    it('should accept valid ServerStatus values', () => {
      const status: ServerStatusEnum = ServerStatus.Up
      expect(status).toBe('Up')
    })

    it('should work with all status values in array', () => {
      const statuses: ServerStatusEnum[] = [
        ServerStatus.Up,
        ServerStatus.Maintenance,
        ServerStatus.DOWN
      ]
      expect(statuses).toHaveLength(3)
    })
  })
})
