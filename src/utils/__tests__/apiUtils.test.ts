import { describe, expect, it } from 'vitest'

import { ApiError, getErrorMessage } from '../api-utils.ts'

describe('ApiError', () => {
  it('should set errorKey and error', () => {
    const err = new ApiError('key', 'message')
    expect(err.errorKey).toBe('key')
    expect(err.error).toBe('message')
    expect(err.name).toBe('ApiError')
  })

  it('should serialize to JSON', () => {
    const err = new ApiError('key', 'message')
    const json = err.toJSON()
    expect(json.errorKey).toBe('key')
    expect(json.error).toBe('message')
    expect(json.name).toBe('ApiError')
    expect(json.message).toBe('message')
    expect(json.stack).toBeTypeOf('string')
  })
})

describe('getErrorMessage', () => {
  it('returns string if error is string', () => {
    expect(getErrorMessage('fail')).toBe('fail')
  })
  it('returns errorKey if error is ApiError', () => {
    const err = new ApiError('custom.key', 'Custom error')
    expect(getErrorMessage(err)).toBe('custom.key')
  })
  it('returns fallback for unknown error', () => {
    expect(getErrorMessage({})).toBe('error.fallback')
    expect(getErrorMessage(undefined, 'default')).toBe('default')
  })
})
