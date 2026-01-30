import { describe, expect, it } from 'vitest'

import {
  addMissingKeys,
  buildNestedObject,
  countEmptyValues,
  countNullValues,
  extractKeysFromCode,
  flattenObject,
  removeExtraKeys,
  srcDir
} from '../check-i18n'

describe('i18n check utilities', () => {
  it('should extract keys from code', () => {
    const keys = extractKeysFromCode(srcDir)
    expect(keys).toBeInstanceOf(Set)
    expect(keys.size).toBeGreaterThan(0)
  })

  it('should flatten and build nested objects correctly', () => {
    const nested = {
      a: { b: { c: 'value' } },
      d: 'another value'
    }
    const flat = flattenObject(nested)
    expect(flat).toEqual({ 'a.b.c': 'value', d: 'another value' })

    const rebuilt = buildNestedObject(['a.b.c', 'd'], flat)
    expect(rebuilt).toEqual(nested)
  })

  it('should count null and empty values correctly', () => {
    const data = {
      a: null,
      b: '',
      c: 'value',
      d: undefined,
      e: 0
    }
    expect(countNullValues(data)).toBe(1)
    expect(countEmptyValues(data)).toBe(1)
  })

  it('should add missing keys to an object', () => {
    const obj = { 'a.b.c': 'value' }
    const requiredKeys = new Set(['a.b.c', 'a.b.d'])
    addMissingKeys(obj, requiredKeys)
    expect(obj).toEqual({ 'a.b.c': 'value', 'a.b.d': null })
  })

  it('should remove extra keys not in the required set', () => {
    const obj = { 'a.b.c': 'value', 'x.y.z': 'extra' }
    const requiredKeys = new Set(['a.b.c'])
    const cleaned = removeExtraKeys(obj, requiredKeys)
    expect(cleaned).toEqual({ 'a.b.c': 'value' })
  })
})
