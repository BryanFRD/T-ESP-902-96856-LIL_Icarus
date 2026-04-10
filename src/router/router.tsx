import { Suspense } from 'react'
import { BrowserRouter, Route, Routes } from 'react-router-dom'

import { DefaultLayout } from '@/components/layout/default-layout'
import RefreshLoader from '@/components/loader/refresh-loader.tsx'
import { useUserContext } from '@/context/user-context.tsx'

import routes from './routes'

const Router = () => {
  const { user, loading } = useUserContext()

  const isUserAuthenticated = !!user

  if (loading) {
    return <RefreshLoader />
  }

  const publicRoutes = routes.filter(route => {
    if (route.isAuthRequired) return false
    return !route.authorizedRoles
  })

  const privateRoutes = routes.filter(route => {
    if (!route.isAuthRequired) return false
    if (!isUserAuthenticated) return false
    return !(
      route.authorizedRoles && !route.authorizedRoles.includes(user.role)
    )
  })

  return (
    <BrowserRouter>
      <Suspense fallback={<RefreshLoader />}>
        <Routes>
          {publicRoutes.map(route => (
            <Route
              key={route.name}
              path={route.path}
              element={<route.component />}
            />
          ))}
          {privateRoutes.length > 0 && (
            <Route element={<DefaultLayout />}>
              {privateRoutes.map(route => (
                <Route
                  key={route.name}
                  path={route.path}
                  element={<route.component />}
                />
              ))}
            </Route>
          )}
        </Routes>
      </Suspense>
    </BrowserRouter>
  )
}

export default Router
