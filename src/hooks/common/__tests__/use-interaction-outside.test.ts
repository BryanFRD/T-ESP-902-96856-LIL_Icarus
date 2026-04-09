import { act, renderHook } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import useInteractionOutside from '../use-interaction-outside.ts'

describe('useInteractionOutside', () => {
  beforeEach(() => {
    vi.useFakeTimers()
  })

  afterEach(() => {
    vi.useRealTimers()
  })

  it('should call callback when clicking outside the referenced element', async () => {
    const callback = vi.fn()
    const { result } = renderHook(() =>
      useInteractionOutside<HTMLDivElement>(callback)
    )

    const div = document.createElement('div')
    const outsideEl = document.createElement('div')
    document.body.appendChild(div)
    document.body.appendChild(outsideEl)
    Object.defineProperty(result.current.ref, 'current', {
      value: div,
      writable: true
    })

    // The hook uses setTimeout(0) before attaching listeners
    act(() => {
      vi.runAllTimers()
    })

    act(() => {
      outsideEl.dispatchEvent(new MouseEvent('click', { bubbles: true }))
    })

    expect(callback).toHaveBeenCalledTimes(1)

    document.body.removeChild(div)
    document.body.removeChild(outsideEl)
  })

  it('should not call callback when clicking inside the referenced element', () => {
    const callback = vi.fn()
    const { result } = renderHook(() =>
      useInteractionOutside<HTMLDivElement>(callback)
    )

    const div = document.createElement('div')
    const child = document.createElement('span')
    div.appendChild(child)
    document.body.appendChild(div)
    Object.defineProperty(result.current.ref, 'current', {
      value: div,
      writable: true
    })

    act(() => {
      vi.runAllTimers()
    })

    act(() => {
      div.dispatchEvent(new MouseEvent('click', { bubbles: true }))
    })

    expect(callback).not.toHaveBeenCalled()

    document.body.removeChild(div)
  })

  it('should call callback on Escape key when ignoreEscapeKey is false', () => {
    const callback = vi.fn()
    const { result } = renderHook(() =>
      useInteractionOutside<HTMLDivElement>(callback, {
        ignoreEscapeKey: false
      })
    )

    const div = document.createElement('div')
    document.body.appendChild(div)
    Object.defineProperty(result.current.ref, 'current', {
      value: div,
      writable: true
    })

    act(() => {
      vi.runAllTimers()
    })

    act(() => {
      document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape' }))
    })

    expect(callback).toHaveBeenCalledTimes(1)
    document.body.removeChild(div)
  })

  it('should not call callback on Escape key when ignoreEscapeKey is true', () => {
    const callback = vi.fn()
    const { result } = renderHook(() =>
      useInteractionOutside<HTMLDivElement>(callback, { ignoreEscapeKey: true })
    )

    const div = document.createElement('div')
    document.body.appendChild(div)
    Object.defineProperty(result.current.ref, 'current', {
      value: div,
      writable: true
    })

    act(() => {
      vi.runAllTimers()
    })

    act(() => {
      document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape' }))
    })

    expect(callback).not.toHaveBeenCalled()
    document.body.removeChild(div)
  })

  it('should not attach listeners when enabled is false', () => {
    const callback = vi.fn()
    const { result } = renderHook(() =>
      useInteractionOutside<HTMLDivElement>(callback, { enabled: false })
    )

    const div = document.createElement('div')
    document.body.appendChild(div)
    Object.defineProperty(result.current.ref, 'current', {
      value: div,
      writable: true
    })

    act(() => {
      vi.runAllTimers()
    })

    act(() => {
      document.dispatchEvent(new MouseEvent('click', { bubbles: true }))
    })

    expect(callback).not.toHaveBeenCalled()
    document.body.removeChild(div)
  })

  it('should listen to custom event types', () => {
    const callback = vi.fn()
    const { result } = renderHook(() =>
      useInteractionOutside<HTMLDivElement>(callback, {
        eventType: 'mousedown'
      })
    )

    const div = document.createElement('div')
    const outsideEl = document.createElement('div')
    document.body.appendChild(div)
    document.body.appendChild(outsideEl)
    Object.defineProperty(result.current.ref, 'current', {
      value: div,
      writable: true
    })

    act(() => {
      vi.runAllTimers()
    })

    // 'click' should not trigger since we listen on 'mousedown'
    act(() => {
      outsideEl.dispatchEvent(new MouseEvent('click', { bubbles: true }))
    })
    expect(callback).not.toHaveBeenCalled()

    act(() => {
      outsideEl.dispatchEvent(new MouseEvent('mousedown', { bubbles: true }))
    })
    expect(callback).toHaveBeenCalledTimes(1)

    document.body.removeChild(div)
    document.body.removeChild(outsideEl)
  })
})
