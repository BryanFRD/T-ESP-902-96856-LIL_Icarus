import { LayoutDashboardIcon, Shield, User2, UserLock } from 'lucide-react'

import { ROUTES, getRoutePath } from '@/routes/routesName.ts'

import routes from './routes/routes'
import type { MenuItem, RouteType } from './types/global'

export const MENU_ITEMS: MenuItem[] = [
  { icon: LayoutDashboardIcon, label: 'Dashboard', href: '/dashboard' },
  { icon: Shield, label: 'Audits', href: '/audits' }
]

export const PROFILE_MENU_ITEMS: MenuItem[] = [
  { icon: User2, label: 'Profile', href: getRoutePath(ROUTES.PROFILE) },
  {
    icon: UserLock,
    label: 'Admin',
    href: getRoutePath(ROUTES.ADMIN),
    isAdminOnly: true
  }
]

export const PUBLIC_ROUTES: string[] = routes
  .filter((route: RouteType) => !route.isAuthRequired)
  .map((route: RouteType) => route.path)
