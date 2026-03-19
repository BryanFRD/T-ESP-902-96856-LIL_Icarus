import { describe, expect, it } from 'vitest'

import { formatter } from '../date-formatter.ts'

describe('formatter', () => {
  const lang = 'fr'
  const f = formatter(lang)
  const date = new Date('2023-01-01T12:00:00')

  it('formats date (short)', () => {
    expect(f.date(date)).toBe('01/01/2023')
  })

  it('formats date (long)', () => {
    expect(f.dateLong(date)).toMatch(/janvier|January|01/) // locale dependent
  })

  it('formats time (short)', () => {
    expect(f.time(date)).toMatch(/12:00|12\s*:\s*00/)
  })

  it('formats dateTime (short)', () => {
    expect(f.dateTime(date)).toMatch(/01\/01\/2023 à 12:00/)
  })

  it('formats dateTimeNative', () => {
    expect(f.dateTimeNative(date)).toBeTypeOf('string')
  })

  it('formats dateLongTimeNative', () => {
    expect(f.dateLongTimeNative(date)).toMatch(/janvier|January|à|at/)
  })
})
