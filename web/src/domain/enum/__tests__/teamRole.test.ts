import { describe, expect, it } from 'vitest'

import { TeamRole } from '../team-role.ts'

describe('TeamRole', () => {
  it('should have Owner role', () => {
    expect(TeamRole.Owner).toBe('Owner')
  })

  it('should have Admin role', () => {
    expect(TeamRole.Admin).toBe('Admin')
  })

  it('should have Member role', () => {
    expect(TeamRole.Member).toBe('Member')
  })

  it('should have Moderator role', () => {
    expect(TeamRole.Moderator).toBe('Moderator')
  })

  it('should have exactly 4 roles', () => {
    expect(Object.keys(TeamRole)).toHaveLength(4)
  })
})
