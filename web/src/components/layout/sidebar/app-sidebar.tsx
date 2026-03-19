import { LayoutDashboard, ShieldIcon, UsersIcon } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Link } from 'react-router-dom'

import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem
} from '@/components/ui/sidebar'

import { AppSidebarFooter } from './sidebar-footer'
import { AppSidebarHeader } from './sidebar-header'

export function AppSidebar() {
  const { t } = useTranslation('app')

  const items = [
    {
      label: t('layout.sidebar.items.dashboard'),
      icon: <LayoutDashboard />,
      href: '/dashboard'
    },
    {
      label: t('layout.sidebar.items.audits'),
      icon: <ShieldIcon />,
      href: '/audits'
    },
    {
      label: t('layout.sidebar.items.teams'),
      icon: <UsersIcon />,
      href: '/teams'
    }
  ]

  return (
    <Sidebar collapsible='icon' variant='floating'>
      <AppSidebarHeader />

      <SidebarContent>
        <SidebarGroup>
          <SidebarMenu>
            {items.map((item, index) => (
              <SidebarMenuItem key={index}>
                <SidebarMenuButton asChild>
                  <Link to={item.href}>
                    {item.icon}
                    <span>{item.label}</span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
            ))}
          </SidebarMenu>
        </SidebarGroup>
      </SidebarContent>

      <AppSidebarFooter />
    </Sidebar>
  )
}
