import { Suspense } from 'react'
import { BrowserRouter, Route, Routes } from 'react-router-dom'

import RefreshLoader from '@/components/loader/RefreshLoader.tsx'
import { useUserContext } from '@/context/UserContext'
import ScreenLayout from '@/screen/layout/ScreenLayout.tsx'

import routes from './routes'

const Router = () => {
  const { user } = useUserContext()

  const isUserAuthenticated = !!user

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
            <Route element={<ScreenLayout />}>
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
