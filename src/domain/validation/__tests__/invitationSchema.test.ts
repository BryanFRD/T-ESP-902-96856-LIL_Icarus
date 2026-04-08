import { describe, expect, it } from 'vitest'

import { getInvitationSchema } from '../invitation-schema.ts'

const t = (key: string) => key

describe('getInvitationSchema', () => {
  const schema = getInvitationSchema(t)

  it('should accept valid invitation data', () => {
    const result = schema.safeParse({ maxUses: 5 })
    expect(result.success).toBe(true)
  })

  it('should accept with expiresAt', () => {
    const result = schema.safeParse({ maxUses: 10, expiresAt: '2026-12-31' })
    expect(result.success).toBe(true)
  })

  it('should reject maxUses less than 1', () => {
    const result = schema.safeParse({ maxUses: 0 })
    expect(result.success).toBe(false)
  })

  it('should reject maxUses greater than 100', () => {
    const result = schema.safeParse({ maxUses: 101 })
    expect(result.success).toBe(false)
  })
})
