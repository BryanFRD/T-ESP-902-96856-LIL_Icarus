import { useTranslation } from 'react-i18next'

import Logo from '@/components/icon/logo.tsx'
import {
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem
} from '@/components/ui/sidebar'

export function AppSidebarHeader() {
  const { t } = useTranslation('app')

  return (
    <SidebarHeader>
      <SidebarMenu>
        <SidebarMenuItem>
          <SidebarMenuButton
            tabIndex={-1}
            className='pointer-events-none h-8 p-0 group-data-[collapsible=icon]:p-0! [&_svg]:size-8'
          >
            <Logo fill='#ff6900' />
            <span className='text-lg font-bold'>{t('name')}</span>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarHeader>
  )
}
