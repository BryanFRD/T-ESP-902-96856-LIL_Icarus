import { describe, expect, it } from 'vitest'

import { getModifiedFields } from '../modifiedFields.ts'

describe('getModifiedFields', () => {
  it('returns modified fields', () => {
    const all = { a: 1, b: 2 }
    const dirty = { a: true }
    expect(getModifiedFields(all, dirty)).toEqual({ a: 1 })
  })

  it('returns empty object if no dirty fields', () => {
    expect(getModifiedFields({ a: 1 }, {})).toEqual({})
  })
})
