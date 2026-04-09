import { act, renderHook } from '@testing-library/react'
import { describe, expect, it } from 'vitest'

import { useImageCrop } from '../use-image-crop.ts'

describe('useImageCrop', () => {
  it('should initialize with default values', () => {
    const { result } = renderHook(() => useImageCrop())

    expect(result.current.crop).toBeUndefined()
    expect(result.current.completedCrop).toBeUndefined()
    expect(result.current.scale).toBe(1)
    expect(result.current.rotate).toBe(0)
  })

  it('should allow setting scale and rotate', () => {
    const { result } = renderHook(() => useImageCrop())

    act(() => {
      result.current.setScale(1.5)
    })
    expect(result.current.scale).toBe(1.5)

    act(() => {
      result.current.setRotate(90)
    })
    expect(result.current.rotate).toBe(90)
  })

  it('should allow setting crop manually', () => {
    const { result } = renderHook(() => useImageCrop())

    const newCrop = { unit: '%' as const, x: 10, y: 10, width: 80, height: 80 }
    act(() => {
      result.current.setCrop(newCrop)
    })
    expect(result.current.crop).toEqual(newCrop)
  })

  it('should allow setting completed crop', () => {
    const { result } = renderHook(() => useImageCrop())

    const completed = {
      unit: 'px' as const,
      x: 50,
      y: 50,
      width: 200,
      height: 200
    }
    act(() => {
      result.current.setCompletedCrop(completed)
    })
    expect(result.current.completedCrop).toEqual(completed)
  })

  it('should compute crop on image load via onImageLoad', () => {
    const { result } = renderHook(() => useImageCrop(1))

    const fakeEvent = {
      currentTarget: { width: 400, height: 400 }
    } as unknown as React.SyntheticEvent<HTMLImageElement>

    act(() => {
      result.current.onImageLoad(fakeEvent)
    })

    expect(result.current.crop).toBeDefined()
    expect(result.current.crop?.unit).toBe('%')
    expect(result.current.crop?.width).toBe(90)
  })

  it('should accept a custom aspect ratio', () => {
    const { result } = renderHook(() => useImageCrop(16 / 9))

    const fakeEvent = {
      currentTarget: { width: 1920, height: 1080 }
    } as unknown as React.SyntheticEvent<HTMLImageElement>

    act(() => {
      result.current.onImageLoad(fakeEvent)
    })

    expect(result.current.crop).toBeDefined()
  })

  it('should expose getCroppedImage as a function', () => {
    const { result } = renderHook(() => useImageCrop())
    expect(typeof result.current.getCroppedImage).toBe('function')
  })
})
