import { describe, expect, it } from 'vitest'

import { getTeamSchema } from '../team-schema.ts'

const t = (key: string) => key

describe('getTeamSchema', () => {
  const schema = getTeamSchema(t)

  it('should accept valid team data', () => {
    const result = schema.safeParse({ name: 'Team Alpha', description: 'Desc' })
    expect(result.success).toBe(true)
  })

  it('should accept team without description', () => {
    const result = schema.safeParse({ name: 'Team Alpha' })
    expect(result.success).toBe(true)
  })

  it('should reject empty name', () => {
    const result = schema.safeParse({ name: '' })
    expect(result.success).toBe(false)
  })

  it('should reject name longer than 100 chars', () => {
    const result = schema.safeParse({ name: 'a'.repeat(101) })
    expect(result.success).toBe(false)
  })

  it('should reject description longer than 500 chars', () => {
    const result = schema.safeParse({
      name: 'Team',
      description: 'a'.repeat(501)
    })
    expect(result.success).toBe(false)
  })
})
