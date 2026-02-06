import { useEffect } from 'react'
import { useTranslation } from 'react-i18next'
import { useLocation } from 'react-router-dom'

import routes from './routes'

const RouteHead = () => {
  const location = useLocation()
  const { t } = useTranslation()

  useEffect(() => {
    const currentRoute = routes.find(route => route.path === location.pathname)
    document.title = t(currentRoute?.routeKey ?? 'app:name')
  }, [location.pathname, t])

  return <></>
}

export default RouteHead
