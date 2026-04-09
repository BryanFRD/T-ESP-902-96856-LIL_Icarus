import { renderHook } from '@testing-library/react'
import { describe, expect, it } from 'vitest'

import { useGetAvatar } from '../use-get-avatar.ts'

describe('useGetAvatar', () => {
  it('should return avatar url when path is provided', () => {
    const { result } = renderHook(() =>
      useGetAvatar('/v1/users/avatars/avatar.webp')
    )
    expect(result.current.data).toContain('/v1/users/avatars/avatar.webp')
  })

  it('should return undefined when path is null', () => {
    const { result } = renderHook(() => useGetAvatar(null))
    expect(result.current.data).toBeUndefined()
  })

  it('should return undefined when path is undefined', () => {
    const { result } = renderHook(() => useGetAvatar(undefined))
    expect(result.current.data).toBeUndefined()
  })
})
