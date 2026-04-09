import { renderHook } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

const mockNavigate = vi.fn()
const mockLocation = {
  pathname: '/login',
  search: '',
  hash: '',
  state: null,
  key: 'default'
}

vi.mock('react-router-dom', () => ({
  useNavigate: () => mockNavigate,
  useLocation: () => mockLocation
}))

vi.mock('@/router/routes-name.ts', () => ({
  ROUTES: { DASHBOARD: 'Dashboard' },
  getRoutePath: (name: string) =>
    name === 'Dashboard' ? '/dashboard' : `/${name.toLowerCase()}`
}))

describe('useRedirect', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    mockLocation.pathname = '/login'
    mockLocation.search = ''
  })

  it('should navigate to the default route when condition is true', async () => {
    const { default: useRedirect } = await import('../use-redirect.ts')

    renderHook(() => useRedirect({ when: true }))

    expect(mockNavigate).toHaveBeenCalledWith('/dashboard', {
      replace: true,
      state: null
    })
  })

  it('should not navigate when condition is false', async () => {
    const { default: useRedirect } = await import('../use-redirect.ts')

    renderHook(() => useRedirect({ when: false }))

    expect(mockNavigate).not.toHaveBeenCalled()
  })

  it('should navigate to a custom destination', async () => {
    const { default: useRedirect } = await import('../use-redirect.ts')

    renderHook(() => useRedirect({ when: true, to: '/profile' }))

    expect(mockNavigate).toHaveBeenCalledWith('/profile', {
      replace: true,
      state: null
    })
  })

  it('should use redirectTo query param when present and valid', async () => {
    mockLocation.search = '?redirectTo=%2Faudits'

    // Mock globalThis.location for URL construction
    Object.defineProperty(globalThis, 'location', {
      value: { href: 'http://localhost/login?redirectTo=%2Faudits' },
      writable: true,
      configurable: true
    })
    const replaceStateSpy = vi.spyOn(globalThis.history, 'replaceState')

    const { default: useRedirect } = await import('../use-redirect.ts')

    renderHook(() => useRedirect({ when: true }))

    expect(mockNavigate).toHaveBeenCalledWith('/audits', {
      replace: true,
      state: null
    })
    replaceStateSpy.mockRestore()
  })

  it('should ignore invalid redirectTo paths', async () => {
    mockLocation.search = '?redirectTo=%2F'

    Object.defineProperty(globalThis, 'location', {
      value: { href: 'http://localhost/login?redirectTo=%2F' },
      writable: true,
      configurable: true
    })

    const { default: useRedirect } = await import('../use-redirect.ts')

    renderHook(() => useRedirect({ when: true }))

    // '/' fails the default validatePath (path !== '/'), so it falls through to default
    expect(mockNavigate).toHaveBeenCalledWith('/dashboard', {
      replace: true,
      state: null
    })
  })

  it('should not navigate when already on the target path', async () => {
    mockLocation.pathname = '/dashboard'

    const { default: useRedirect } = await import('../use-redirect.ts')

    renderHook(() => useRedirect({ when: true }))

    expect(mockNavigate).not.toHaveBeenCalled()
  })
})
