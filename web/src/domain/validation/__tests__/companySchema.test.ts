import { describe, expect, it } from 'vitest'

import { getCompanySchema } from '../company-schema.ts'

const t = (key: string) => key

describe('getCompanySchema', () => {
  const schema = getCompanySchema(t)

  it('should accept valid company data', () => {
    const result = schema.safeParse({
      name: 'Corp Inc',
      siret: '12345678901234',
      description: 'A company'
    })
    expect(result.success).toBe(true)
  })

  it('should reject empty name', () => {
    const result = schema.safeParse({ name: '', siret: '12345678901234' })
    expect(result.success).toBe(false)
  })

  it('should reject invalid siret (non-14 digits)', () => {
    const result = schema.safeParse({ name: 'Corp', siret: '1234' })
    expect(result.success).toBe(false)
  })

  it('should reject siret with non-digit characters', () => {
    const result = schema.safeParse({ name: 'Corp', siret: '1234567890123a' })
    expect(result.success).toBe(false)
  })

  it('should accept without description', () => {
    const result = schema.safeParse({ name: 'Corp', siret: '12345678901234' })
    expect(result.success).toBe(true)
  })
})
