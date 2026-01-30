import { lazy } from 'react'

import { ROUTES, getRoutePath } from '@/router/routesName.ts'
import type { RouteType } from '@/types/global'

const Login = lazy(() => import('@/screen/auth/LoginScreen.tsx'))
const Register = lazy(() => import('@/screen/auth/RegisterScreen.tsx'))
const Dashboard = lazy(() => import('@/screen/DashboardScreen.tsx'))
const Audits = lazy(() => import('@/screen/audit/AuditScreen.tsx'))
const Profile = lazy(() => import('@/screen/auth/ProfileScreen.tsx'))
const ResetPassword = lazy(() => import('@/screen/ResetPasswordScreen.tsx'))
const ConfirmEmail = lazy(() => import('@/screen/ConfirmEmailScreen.tsx'))
const AuditFormCreation = lazy(
  () => import('@/screen/audit/form/AuditCreateFormScreen.tsx')
)
const AuditDetails = lazy(() => import('@/components/audit/AuditDetails.tsx'))
const Teams = lazy(() => import('@/screen/team/TeamsScreen.tsx'))
const TeamDetail = lazy(() => import('@/screen/team/TeamDetailScreen.tsx'))
const JoinTeam = lazy(() => import('@/screen/team/JoinTeamScreen.tsx'))

const routes: RouteType[] = [
  {
    name: ROUTES.LOGIN,
    routeKey: 'router.login',
    path: getRoutePath(ROUTES.LOGIN),
    component: Login
  },
  {
    name: ROUTES.REGISTER,
    routeKey: 'router.register',
    path: getRoutePath(ROUTES.REGISTER),
    component: Register
  },
  {
    name: ROUTES.DASHBOARD,
    routeKey: 'router.dashboard',
    path: getRoutePath(ROUTES.DASHBOARD),
    isAuthRequired: true,
    component: Dashboard
  },
  {
    name: ROUTES.AUDITS,
    routeKey: 'router.audit',
    path: getRoutePath(ROUTES.AUDITS),
    isAuthRequired: true,
    component: Audits
  },
  {
    name: ROUTES.CREATE_AUDIT,
    routeKey: 'router.createAudit',
    path: getRoutePath(ROUTES.CREATE_AUDIT),
    isAuthRequired: true,
    component: AuditFormCreation
  },
  {
    name: ROUTES.PROFILE,
    routeKey: 'router.profile',
    path: getRoutePath(ROUTES.PROFILE),
    isAuthRequired: true,
    component: Profile
  },
  {
    name: ROUTES.RESET_PASSWORD,
    routeKey: 'router.resetPassword',
    path: getRoutePath(ROUTES.RESET_PASSWORD),
    component: ResetPassword
  },
  {
    name: ROUTES.CONFIRM_EMAIL,
    routeKey: 'router.confirmEmail',
    path: getRoutePath(ROUTES.CONFIRM_EMAIL),
    component: ConfirmEmail
  },
  {
    name: ROUTES.AUDIT_DETAILS,
    routeKey: 'router.auditDetail',
    path: getRoutePath(ROUTES.AUDIT_DETAILS),
    isAuthRequired: true,
    component: AuditDetails
  },
  {
    name: ROUTES.TEAMS,
    routeKey: 'router.team',
    path: getRoutePath(ROUTES.TEAMS),
    isAuthRequired: true,
    component: Teams
  },
  {
    name: ROUTES.TEAM_DETAIL,
    routeKey: 'router.teamDetail',
    path: getRoutePath(ROUTES.TEAM_DETAIL),
    isAuthRequired: true,
    component: TeamDetail
  },
  {
    name: ROUTES.JOIN_TEAM,
    routeKey: 'router.joinTeam',
    path: getRoutePath(ROUTES.JOIN_TEAM),
    isAuthRequired: true,
    component: JoinTeam
  }
]

export default routes
