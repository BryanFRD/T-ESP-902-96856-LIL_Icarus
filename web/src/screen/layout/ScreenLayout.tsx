import { Outlet } from 'react-router-dom'

import Sidebar from '@/components/layout/sidebar/Sidebar.tsx'
import { MENU_ITEMS } from '@/constant.ts'
import useToggle from '@/hooks/common/useToggle.ts'
import '@/i18n'

const MainContent = () => (
  <main className='from-blue flex-1 overflow-auto bg-gradient-to-br from-blue-50 via-white to-blue-50 text-gray-900 transition-all duration-300 dark:from-gray-900 dark:via-gray-950 dark:to-gray-900 dark:text-white'>
    <Outlet />
  </main>
)

const ScreenLayout = () => {
  const {
    toggle,
    value: sidebarValue,
    close: closeSidebar,
    open: openSidebar
  } = useToggle(false)

  return (
    <div className='from-blue flex h-screen flex-col overflow-hidden text-gray-900 transition-all duration-300 md:flex md:flex-row dark:text-white'>
      <Sidebar
        isOpen={sidebarValue}
        onClose={closeSidebar}
        onToggle={toggle}
        menuItems={MENU_ITEMS}
        openSidebar={openSidebar}
      />

      <div className='flex flex-1 flex-col overflow-hidden'>
        <MainContent />
      </div>
    </div>
  )
}

export default ScreenLayout
