import { lazy } from 'react'

import { ROUTES, getRoutePath } from '@/router/routes-name.ts'
import type { RouteType } from '@/types/global'

const Login = lazy(() => import('@/screen/auth/login-screen.tsx'))
const Register = lazy(() => import('@/screen/auth/register-screen.tsx'))
const Dashboard = lazy(() => import('@/screen/dashboard-screen.tsx'))
const Audits = lazy(() => import('@/screen/audit/audit-screen.tsx'))
const Profile = lazy(() => import('@/screen/auth/profile-screen.tsx'))
const ResetPassword = lazy(() => import('@/screen/reset-password-screen.tsx'))
const ConfirmEmail = lazy(() => import('@/screen/confirm-email-screen.tsx'))
const AuditFormCreation = lazy(
  () => import('@/screen/audit/form/audit-create-form-screen.tsx')
)
const AuditDetailsScreen = lazy(
  () => import('@/screen/audit/audit-detail-screen.tsx')
)
const Teams = lazy(() => import('@/screen/team/team-screen'))
const TeamDetail = lazy(() => import('@/screen/team/team-detail-screen.tsx'))
const JoinTeam = lazy(() => import('@/screen/team/join-team-screen.tsx'))

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
    component: AuditDetailsScreen
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
