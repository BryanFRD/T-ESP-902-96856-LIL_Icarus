import { describe, expect, it } from 'vitest'

import { createAuditSchema, verifyAuditAccessSchema } from '../audit-schema.ts'

const t = (key: string) => key

describe('createAuditSchema', () => {
  const schema = createAuditSchema(t)

  it('should accept valid data', () => {
    const result = schema.safeParse({
      name: 'Audit Name',
      description: 'Description',
      endDate: '2026-12-31',
      companyId: '01ARZ3NDEKTSV4RRFFQ69G5FAV',
      teamId: '01ARZ3NDEKTSV4RRFFQ69G5FAV'
    })
    expect(result.success).toBe(true)
  })

  it('should reject name shorter than 3 chars', () => {
    const result = schema.safeParse({
      name: 'ab',
      description: '',
      endDate: '2026-12-31',
      companyId: '01ARZ3NDEKTSV4RRFFQ69G5FAV',
      teamId: '01ARZ3NDEKTSV4RRFFQ69G5FAV'
    })
    expect(result.success).toBe(false)
  })

  it('should reject invalid date', () => {
    const result = schema.safeParse({
      name: 'Audit',
      description: '',
      endDate: 'not-a-date',
      companyId: '01ARZ3NDEKTSV4RRFFQ69G5FAV',
      teamId: '01ARZ3NDEKTSV4RRFFQ69G5FAV'
    })
    expect(result.success).toBe(false)
  })

  it('should reject invalid ULID for companyId', () => {
    const result = schema.safeParse({
      name: 'Audit',
      description: '',
      endDate: '2026-12-31',
      companyId: 'invalid',
      teamId: '01ARZ3NDEKTSV4RRFFQ69G5FAV'
    })
    expect(result.success).toBe(false)
  })
})

describe('verifyAuditAccessSchema', () => {
  const schema = verifyAuditAccessSchema(t)

  it('should accept valid password', () => {
    const result = schema.safeParse({ password: 'secret' })
    expect(result.success).toBe(true)
  })

  it('should reject empty password', () => {
    const result = schema.safeParse({ password: '' })
    expect(result.success).toBe(false)
  })
})
