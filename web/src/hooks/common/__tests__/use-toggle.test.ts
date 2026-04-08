import { act, renderHook } from '@testing-library/react'
import { describe, expect, it } from 'vitest'

import useToggle from '../use-toggle.ts'

describe('useToggle', () => {
  it('should initialize with false by default', () => {
    const { result } = renderHook(() => useToggle())
    expect(result.current.value).toBe(false)
  })

  it('should initialize with provided value', () => {
    const { result } = renderHook(() => useToggle(true))
    expect(result.current.value).toBe(true)
  })

  it('should toggle value', () => {
    const { result } = renderHook(() => useToggle())
    act(() => result.current.toggle())
    expect(result.current.value).toBe(true)
    act(() => result.current.toggle())
    expect(result.current.value).toBe(false)
  })

  it('should open (set to true)', () => {
    const { result } = renderHook(() => useToggle())
    act(() => result.current.open())
    expect(result.current.value).toBe(true)
  })

  it('should close (set to false)', () => {
    const { result } = renderHook(() => useToggle(true))
    act(() => result.current.close())
    expect(result.current.value).toBe(false)
  })

  it('should set to specific value', () => {
    const { result } = renderHook(() => useToggle())
    act(() => result.current.set(true))
    expect(result.current.value).toBe(true)
    act(() => result.current.set(false))
    expect(result.current.value).toBe(false)
  })
})
