export const ROUTES = {
  ADMIN: 'Admin',
  LOGIN: 'Login',
  REGISTER: 'Register',
  DASHBOARD: 'Dashboard',
  AUDITS: 'Audits',
  PROFILE: 'Profile',
  RESET_PASSWORD: 'ResetPassword',
  CONFIRM_EMAIL: 'ConfirmEmail',
  TEAMS: 'Teams',
  TEAM_DETAIL: 'TeamDetail',
  JOIN_TEAM: 'JoinTeam',
  CREATE_AUDIT: 'Create Audit',
  AUDIT_DETAILS: 'Audit Detail'
} as const

export type RouteName = (typeof ROUTES)[keyof typeof ROUTES]

export const ROUTE_PATHS: Record<RouteName, string> = {
  [ROUTES.ADMIN]: '/admin',
  [ROUTES.LOGIN]: '/',
  [ROUTES.REGISTER]: '/register',
  [ROUTES.DASHBOARD]: '/dashboard',
  [ROUTES.PROFILE]: '/profile',
  [ROUTES.RESET_PASSWORD]: '/reset-password',
  [ROUTES.CONFIRM_EMAIL]: '/confirm-email',
  [ROUTES.TEAMS]: '/teams',
  [ROUTES.TEAM_DETAIL]: '/teams/:teamId',
  [ROUTES.JOIN_TEAM]: '/teams/join/:token',
  [ROUTES.AUDITS]: '/audits',
  [ROUTES.CREATE_AUDIT]: '/audits/create',
  [ROUTES.AUDIT_DETAILS]: '/audits/:auditName'
}

export const getRoutePath = (name: RouteName) => ROUTE_PATHS[name]

export default ROUTES
