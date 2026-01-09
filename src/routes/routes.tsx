import { lazy } from 'react'

import { ROUTES, getRoutePath } from '@/routes/routesName.ts'
import type { RouteType } from '@/types/global'

const Login = lazy(() => import('@/screens/LoginScreen.tsx'))
const Register = lazy(() => import('@/screens/RegisterScreen.tsx'))
const Dashboard = lazy(() => import('@/screens/DashboardScreen.tsx'))
const Audits = lazy(() => import('@/screens/AuditsScreen.tsx'))
const Profile = lazy(() => import('@/screens/ProfileScreen.tsx'))
const ResetPassword = lazy(() => import('@/screens/ResetPasswordScreen.tsx'))
const ConfirmEmail = lazy(() => import('@/screens/ConfirmEmailScreen.tsx'))

const routes: RouteType[] = [
  {
    name: ROUTES.LOGIN,
    routeKey: 'route.login',
    path: getRoutePath(ROUTES.LOGIN),
    component: Login
  },
  {
    name: ROUTES.REGISTER,
    routeKey: 'route.register',
    path: getRoutePath(ROUTES.REGISTER),
    component: Register
  },
  {
    name: ROUTES.DASHBOARD,
    routeKey: 'route.dashboard',
    path: getRoutePath(ROUTES.DASHBOARD),
    isAuthRequired: true,
    component: Dashboard
  },
  {
    name: ROUTES.AUDITS,
    routeKey: 'route.audits',
    path: getRoutePath(ROUTES.AUDITS),
    isAuthRequired: true,
    component: Audits
  },
  {
    name: ROUTES.PROFILE,
    routeKey: 'route.profile',
    path: getRoutePath(ROUTES.PROFILE),
    isAuthRequired: true,
    component: Profile
  },
  {
    name: ROUTES.RESET_PASSWORD,
    routeKey: 'route.resetPassword',
    path: getRoutePath(ROUTES.RESET_PASSWORD),
    component: ResetPassword
  },
  {
    name: ROUTES.CONFIRM_EMAIL,
    routeKey: 'route.confirmEmail',
    path: getRoutePath(ROUTES.CONFIRM_EMAIL),
    component: ConfirmEmail
  }
]

export default routes
