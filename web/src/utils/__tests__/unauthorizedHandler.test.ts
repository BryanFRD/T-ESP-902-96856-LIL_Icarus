import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

import UnauthorizedHandler from '../unauthorized-handler.ts'

vi.mock('@/constant', () => ({
  PUBLIC_ROUTES: ['/', '/login', '/register']
}))

describe('UnauthorizedHandler', () => {
  const originalLocation = globalThis.location

  beforeEach(() => {
    vi.useFakeTimers()
    Object.defineProperty(globalThis, 'location', {
      value: { pathname: '/dashboard', href: 'http://localhost/dashboard' },
      writable: true,
      configurable: true
    })
    Object.defineProperty(globalThis, 'queryClient', {
      value: { clear: vi.fn() },
      writable: true,
      configurable: true
    })
  })

  afterEach(() => {
    // Always reset redirecting state before restoring real timers
    vi.advanceTimersByTime(3000)
    vi.useRealTimers()
    Object.defineProperty(globalThis, 'location', {
      value: originalLocation,
      writable: true,
      configurable: true
    })
  })

  it('should report not redirecting initially', () => {
    expect(UnauthorizedHandler.isCurrentlyRedirecting()).toBe(false)
  })

  it('should set isRedirecting to true after handle and reset after timeout', () => {
    UnauthorizedHandler.handle()
    expect(UnauthorizedHandler.isCurrentlyRedirecting()).toBe(true)
    vi.advanceTimersByTime(2000)
    expect(UnauthorizedHandler.isCurrentlyRedirecting()).toBe(false)
  })

  it('should not redirect when already redirecting', () => {
    UnauthorizedHandler.handle()
    const href = globalThis.location.href
    UnauthorizedHandler.handle()
    expect(globalThis.location.href).toBe(href)
  })

  it('should not redirect on public routes', () => {
    Object.defineProperty(globalThis, 'location', {
      value: { pathname: '/login', href: 'http://localhost/login' },
      writable: true,
      configurable: true
    })
    UnauthorizedHandler.handle()
    expect(globalThis.location.href).toBe('http://localhost/login')
  })

  it('should redirect with redirectTo param on private routes', () => {
    UnauthorizedHandler.handle()
    expect(globalThis.location.href).toBe('/?redirectTo=%2Fdashboard')
  })

  it('should clear query cache on handle', () => {
    UnauthorizedHandler.handle()
    expect(globalThis.queryClient!.clear).toHaveBeenCalled()
  })
})
