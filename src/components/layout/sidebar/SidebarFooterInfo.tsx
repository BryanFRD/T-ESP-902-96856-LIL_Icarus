import { AnimatePresence, motion } from 'framer-motion'
import { ChevronDown } from 'lucide-react'

import LogoutButton from '@/components/layout/sidebar/LogoutButton.tsx'
import PreferencesToggle from '@/components/ui/PreferencesToggle.tsx'
import Button from '@/components/ui/button/Button.tsx'
import { PROFILE_MENU_ITEMS } from '@/constant.ts'
import { UserRole } from '@/domain/enum/RoleEnum.ts'
import type { User } from '@/domain/types/User.ts'
import useInteractionOutside from '@/hooks/common/useInteractionOutside.ts'
import useToggle from '@/hooks/common/useToggle.ts'
import { useAvatar } from '@/hooks/user/useAvatar.ts'

const SidebarFooterInfo = ({
  user,
  isDesktopCollapsed,
  openSidebar
}: {
  user: User
  isDesktopCollapsed: boolean
  openSidebar?: () => void
}) => {
  const { value: open, toggle, close } = useToggle(false)
  const { currentAvatarUrl } = useAvatar()
  const { ref } = useInteractionOutside<HTMLDivElement>(close, {
    enabled: open
  })

  function isAccessibleByUser(item: (typeof PROFILE_MENU_ITEMS)[number]) {
    return !item.isAdminOnly || user?.role === UserRole.ADMIN
  }

  function openBoth() {
    toggle()
    if (openSidebar) {
      openSidebar()
    }
  }

  return (
    <div className='flex flex-col gap-2' ref={ref}>
      <AnimatePresence initial={false}>
        {open && (
          <motion.div
            key='profile-menu'
            initial={{ height: 0, opacity: 0 }}
            animate={{ height: 'auto', opacity: 1 }}
            exit={{ height: 0, opacity: 0 }}
            transition={{ duration: 0.22, ease: [0.4, 0, 0.2, 1] }}
            style={{ overflow: 'hidden' }}
          >
            <div className='overflow-hidden rounded-lg border border-b border-gray-300/50 bg-white bg-gradient-to-r from-blue-50 via-white to-blue-50 text-gray-900 shadow-lg transition-all duration-300 dark:border-gray-800 dark:bg-gray-900 dark:bg-gradient-to-r dark:from-gray-900 dark:via-gray-800 dark:to-gray-900 dark:text-gray-100'>
              <ul className='w-full py-2'>
                {PROFILE_MENU_ITEMS.filter(isAccessibleByUser).map(item => (
                  <li key={item.label}>
                    <Button
                      variant={
                        item.href === globalThis.location.pathname
                          ? 'navigationActive'
                          : 'ghost'
                      }
                      size='md'
                      rounded='none'
                      className='w-full justify-start gap-4'
                      to={item.href}
                      as='link'
                      onClick={close}
                    >
                      <>
                        {item.icon && (
                          <item.icon size={16} aria-hidden='true' />
                        )}
                        <span className='text-sm font-medium text-shadow-white dark:text-gray-200'>
                          {item.label}
                        </span>
                      </>
                    </Button>
                  </li>
                ))}
                <li>
                  <LogoutButton />
                </li>
                <li className='mt-2 flex justify-evenly border-t border-gray-300/50 pt-2 dark:border-gray-800'>
                  <PreferencesToggle />
                </li>
              </ul>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
      <fieldset className='rounded-xltransition-all relative flex w-auto items-center duration-300'>
        <button
          className={`flex w-full cursor-pointer items-center rounded-lg bg-gray-200/50 p-2 transition-all duration-300 hover:bg-gray-300/50 dark:bg-gray-800/50 hover:dark:bg-gray-800 ${isDesktopCollapsed ? 'gap-0' : 'gap-2'}`}
          onClick={openBoth}
        >
          {currentAvatarUrl ? (
            <img
              src={currentAvatarUrl}
              alt={user.username}
              className='h-10 w-10 flex-shrink-0 rounded-full object-cover'
            />
          ) : (
            <div
              className='flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-full bg-gradient-to-r from-red-500 to-orange-500'
              aria-label={user?.username}
            >
              <span className='text-lg font-bold text-white'>
                {user?.username?.charAt(0)?.toUpperCase() ?? '?'}
              </span>
            </div>
          )}
          <div
            className={`min-w-0 flex-1 overflow-hidden transition-all duration-300 ${
              isDesktopCollapsed
                ? 'md:invisible md:opacity-0'
                : 'md:visible md:opacity-100'
            }`}
            style={{
              width: isDesktopCollapsed ? '0px' : 'auto'
            }}
          >
            <p className='truncate text-start text-sm font-medium text-gray-900 dark:text-white'>
              {user?.username}
            </p>
            <p className='truncate text-start text-xs text-gray-400'>
              {user?.role}
            </p>
          </div>
          <div
            className={`flex w-6 justify-center transition-all duration-300 ${
              isDesktopCollapsed
                ? 'md:invisible md:w-0 md:opacity-0'
                : 'md:visible md:w-6 md:opacity-100'
            }`}
          >
            <ChevronDown
              size={18}
              className={`text-gray-300 transition-transform ${
                open ? '-rotate-180' : ''
              }`}
            />
          </div>
        </button>
      </fieldset>
    </div>
  )
}

export default SidebarFooterInfo
