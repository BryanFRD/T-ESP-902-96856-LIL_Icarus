import { PUBLIC_ROUTES } from '@/constant'

class UnauthorizedHandler {
  private static isRedirecting: boolean = false
  private static redirectTimeout: NodeJS.Timeout | null = null
  private static readonly REDIRECT_DELAY = 2000

  static handle(): void {
    if (this.isRedirecting) return

    this.isRedirecting = true
    this.clearQueryCache()
    this.clearPreviousTimeout()
    this.performRedirect()
    this.scheduleReset()
  }

  private static clearQueryCache(): void {
    if (globalThis.queryClient?.clear) {
      globalThis.queryClient.clear()
    }
  }

  private static clearPreviousTimeout(): void {
    if (this.redirectTimeout) {
      clearTimeout(this.redirectTimeout)
      this.redirectTimeout = null
    }
  }

  private static performRedirect(): void {
    const currentPath = globalThis.location.pathname
    if (!this.isPublicRoute(currentPath)) {
      const redirectUrl = '/?redirectTo=' + encodeURIComponent(currentPath)
      globalThis.location.href = redirectUrl
    }
  }

  private static scheduleReset(): void {
    this.redirectTimeout = setTimeout(() => {
      this.isRedirecting = false
      this.redirectTimeout = null
    }, this.REDIRECT_DELAY)
  }

  static isCurrentlyRedirecting(): boolean {
    return this.isRedirecting
  }

  private static isPublicRoute(path: string): boolean {
    return PUBLIC_ROUTES.some(
      route => route === path || path.startsWith(route + '/')
    )
  }
}

export default UnauthorizedHandler
