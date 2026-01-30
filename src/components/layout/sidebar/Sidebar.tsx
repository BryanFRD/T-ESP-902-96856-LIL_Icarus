import { Lock, LockOpen } from 'lucide-react'
import { useLocation } from 'react-router-dom'

import LogoSidebar from '@/components/layout/sidebar/LogoSidebar'
import LogoSidebarMobile from '@/components/layout/sidebar/LogoSidebarMobile'
import SidebarToggle from '@/components/layout/sidebar/SideBarToggle'
import SidebarFooterInfo from '@/components/layout/sidebar/SidebarFooterInfo'
import Button from '@/components/ui/button/Button.tsx'
import { useUserContext } from '@/context/UserContext'
import { useCheckAuditsAccess } from '@/hooks/audit/useCheckAuditsAccess.ts'
import type { SidebarProps } from '@/types/global'

const Sidebar = ({
  isOpen,
  onClose,
  menuItems,
  onToggle,
  openSidebar
}: SidebarProps) => {
  const location = useLocation()
  const { user } = useUserContext()
  const { data: auditsAccessData } = useCheckAuditsAccess()

  const isDesktopCollapsed = !isOpen

  return (
    <>
      <div className='relative z-40 h-16 overflow-hidden border-b border-gray-300/50 bg-gradient-to-r from-blue-50 via-white to-blue-50 transition-all duration-300 md:hidden dark:border-gray-800/50 dark:from-gray-900 dark:via-gray-950 dark:to-gray-900'>
        <LogoSidebarMobile onToggle={onToggle} isOpen={isOpen} />
      </div>
      <div
        className={`inset-y-0 left-0 z-50 w-full transform border-r border-gray-300/50 bg-gradient-to-b from-blue-50 via-white to-blue-50 transition-all duration-300 ease-in-out md:static md:inset-0 dark:border-gray-800/50 dark:from-gray-900 dark:via-gray-950 dark:to-gray-900 ${
          isOpen
            ? 'translate-x-0 md:w-72'
            : 'w-0 -translate-x-full md:w-20 md:translate-x-0'
        }`}
      >
        <div className='flex h-full flex-col'>
          <LogoSidebar
            onClose={onClose}
            isDesktopCollapsed={isDesktopCollapsed}
          />

          <nav className='flex-1 space-y-1 px-4 py-6'>
            {menuItems.map(item => {
              const isAuditItem =
                item.href === '/audits' || item.label === 'Audits'

              return (
                <Button
                  key={item.label}
                  variant={
                    item.href === location.pathname
                      ? 'navigationActive'
                      : 'ghost'
                  }
                  as='link'
                  to={item.href}
                  size='lg'
                  fullWidth
                  className='justify-start'
                >
                  {item.icon && <item.icon size={20} className='shrink-0' />}

                  {!isDesktopCollapsed && (
                    <>
                      <span className='text-sm'>{item.label}</span>
                      {isAuditItem && (
                        <div className='ml-auto'>
                          {auditsAccessData?.hasAccess ? (
                            <LockOpen size={16} className='text-shadow-white' />
                          ) : (
                            <Lock size={16} className='text-shadow-white' />
                          )}
                        </div>
                      )}
                    </>
                  )}
                </Button>
              )
            })}
          </nav>
          <div className='flex px-4 py-2'>
            <SidebarToggle isOpen={isOpen} onToggle={onToggle} />
          </div>
          <div className='flex-shrink-0 border-t border-gray-300/50 px-3 py-4 dark:border-gray-800'>
            <SidebarFooterInfo
              user={user!}
              openSidebar={openSidebar}
              isDesktopCollapsed={isDesktopCollapsed}
            />
          </div>
        </div>
      </div>
    </>
  )
}

export default Sidebar
