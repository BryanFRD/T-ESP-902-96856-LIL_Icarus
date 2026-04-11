import { Outlet } from 'react-router-dom'

import { SidebarInset, SidebarProvider } from '../ui/sidebar'
import { AppSidebar } from './sidebar/app-sidebar'

export function DefaultLayout() {
  return (
    <SidebarProvider>
      <AppSidebar />
      <SidebarInset>
        <main className='min-w-0 overflow-x-hidden'>
          <Outlet />
        </main>
      </SidebarInset>
    </SidebarProvider>
  )
}
