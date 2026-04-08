import { describe, expect, it } from 'vitest'

import { MemberRole } from '../member-role.ts'

describe('MemberRole', () => {
  it('should have OWNER role', () => {
    expect(MemberRole.OWNER).toBe('OWNER')
  })

  it('should have MODERATOR role', () => {
    expect(MemberRole.MODERATOR).toBe('MODERATOR')
  })

  it('should have MEMBER role', () => {
    expect(MemberRole.MEMBER).toBe('MEMBER')
  })

  it('should have CLIENT role', () => {
    expect(MemberRole.CLIENT).toBe('CLIENT')
  })

  it('should have exactly 4 roles', () => {
    expect(Object.keys(MemberRole)).toHaveLength(4)
  })
})
