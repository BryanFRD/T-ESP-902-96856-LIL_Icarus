import { useEffect } from 'react'
import { useLocation, useNavigate } from 'react-router-dom'

import { ROUTES, getRoutePath } from '@/router/routes-name.ts'
import type { UseRedirectOptions } from '@/types/global.ts'

function useRedirect<
  T extends Record<string, unknown> = Record<string, unknown>
>({
  when,
  to = getRoutePath(ROUTES.DASHBOARD),
  fromQueryParam = 'redirectTo',
  replace = true,
  validatePath = path => path.startsWith('/') && path !== '/',
  state = null
}: UseRedirectOptions & { state?: T | null }) {
  const navigate = useNavigate()
  const location = useLocation()

  useEffect(() => {
    if (!when) return

    const params = new URLSearchParams(location.search)
    const redirectParam = params.get(fromQueryParam)

    if (redirectParam) {
      const decodedPath = decodeURIComponent(redirectParam)

      if (validatePath(decodedPath)) {
        const newUrl = new URL(globalThis.location.href)
        newUrl.searchParams.delete(fromQueryParam)
        globalThis.history.replaceState({}, '', newUrl.pathname + newUrl.search)

        navigate(decodedPath, { replace, state })
        return
      }
    }

    if (location.pathname !== to) {
      navigate(to, { replace, state })
    }
  }, [
    when,
    to,
    fromQueryParam,
    replace,
    validatePath,
    navigate,
    location,
    state
  ])
}

export default useRedirect
