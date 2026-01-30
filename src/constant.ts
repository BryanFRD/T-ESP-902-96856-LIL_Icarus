import {
  LayoutDashboardIcon,
  Shield,
  User2,
  UserLock,
  Users
} from 'lucide-react'

import routes from '@/router/routes'
import { ROUTES, getRoutePath } from '@/router/routesName.ts'
import type { MenuItem, RouteType } from '@/types/global'

export const MENU_ITEMS: MenuItem[] = [
  { icon: LayoutDashboardIcon, label: 'Dashboard', href: '/dashboard' },
  { icon: Shield, label: 'Audits', href: '/audits' },
  { icon: Users, label: 'Teams', href: getRoutePath(ROUTES.TEAMS) }
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
