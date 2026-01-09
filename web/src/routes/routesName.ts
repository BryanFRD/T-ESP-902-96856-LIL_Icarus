export const ROUTES = {
  ADMIN: 'Admin',
  LOGIN: 'Login',
  REGISTER: 'Register',
  DASHBOARD: 'Dashboard',
  AUDITS: 'Audits',
  PROFILE: 'Profile',
  RESET_PASSWORD: 'ResetPassword',
  CONFIRM_EMAIL: 'ConfirmEmail'
} as const

export type RouteName = (typeof ROUTES)[keyof typeof ROUTES]

export const ROUTE_PATHS: Record<RouteName, string> = {
  [ROUTES.ADMIN]: '/admin',
  [ROUTES.LOGIN]: '/',
  [ROUTES.REGISTER]: '/register',
  [ROUTES.DASHBOARD]: '/dashboard',
  [ROUTES.AUDITS]: '/audits',
  [ROUTES.PROFILE]: '/profile',
  [ROUTES.RESET_PASSWORD]: '/reset-password',
  [ROUTES.CONFIRM_EMAIL]: '/confirm-email'
}

export const getRoutePath = (name: RouteName) => ROUTE_PATHS[name]

export default ROUTES
