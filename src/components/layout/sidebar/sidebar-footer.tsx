import {
  ChevronsUpDownIcon,
  Laptop,
  Moon,
  PanelLeftIcon,
  Sun
} from 'lucide-react'
import { CircleFlagLanguage } from 'react-circle-flags'
import { useTranslation } from 'react-i18next'
import { Link } from 'react-router-dom'

import { useTheme } from '@/components/theme-provider'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuPortal,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger
} from '@/components/ui/dropdown-menu'
import {
  SidebarFooter,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar
} from '@/components/ui/sidebar'
import { useUserContext } from '@/context/user-context.tsx'

export function AppSidebarFooter() {
  const { t } = useTranslation('app')
  const { user, logout } = useUserContext()

  const { toggleSidebar } = useSidebar()
  const { setTheme } = useTheme()
  const { i18n } = useTranslation()

  return (
    <SidebarFooter>
      <SidebarMenu>
        <SidebarMenuItem>
          <SidebarMenuButton onClick={toggleSidebar}>
            <PanelLeftIcon />
            <span>{t('layout.sidebar.footer.closeSidebar')}</span>
          </SidebarMenuButton>
        </SidebarMenuItem>

        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <SidebarMenuItem>
              <SidebarMenuButton
                size='lg'
                className='group-data-[collapsible=icon]:rounded-full'
              >
                <Avatar>
                  <AvatarImage src={user?.avatarUrl} alt={user?.firstName} />
                  <AvatarFallback>{user?.firstName.charAt(0)}</AvatarFallback>
                </Avatar>
                <div className='flex flex-1 flex-col items-start truncate'>
                  <span className='font-medium'>
                    {user?.firstName} {user?.lastName}
                  </span>
                  <span className='text-muted-foreground text-xs'>
                    {user?.email}
                  </span>
                </div>
                <ChevronsUpDownIcon />
              </SidebarMenuButton>
            </SidebarMenuItem>
          </DropdownMenuTrigger>

          <DropdownMenuContent>
            <DropdownMenuItem asChild>
              <Link to='/profile'>
                {t('layout.sidebar.footer.userMenu.profile')}
              </Link>
            </DropdownMenuItem>

            <DropdownMenuSub>
              <DropdownMenuSubTrigger>
                {t('layout.sidebar.footer.userMenu.language.label')}
              </DropdownMenuSubTrigger>
              <DropdownMenuPortal>
                <DropdownMenuSubContent>
                  <DropdownMenuGroup>
                    <DropdownMenuItem onClick={() => i18n.changeLanguage('en')}>
                      <CircleFlagLanguage
                        languageCode='en'
                        className='size-4'
                      />
                      <span>
                        {t('layout.sidebar.footer.userMenu.language.en')}
                      </span>
                    </DropdownMenuItem>
                    <DropdownMenuItem onClick={() => i18n.changeLanguage('fr')}>
                      <CircleFlagLanguage
                        languageCode='fr'
                        className='size-4'
                      />
                      <span>
                        {t('layout.sidebar.footer.userMenu.language.fr')}
                      </span>
                    </DropdownMenuItem>
                  </DropdownMenuGroup>
                </DropdownMenuSubContent>
              </DropdownMenuPortal>
            </DropdownMenuSub>

            <DropdownMenuSub>
              <DropdownMenuSubTrigger>
                {t('layout.sidebar.footer.userMenu.theme.label')}
              </DropdownMenuSubTrigger>
              <DropdownMenuPortal>
                <DropdownMenuSubContent>
                  <DropdownMenuGroup>
                    <DropdownMenuItem onClick={() => setTheme('light')}>
                      <Sun className='size-4' />
                      <span>
                        {t('layout.sidebar.footer.userMenu.theme.light')}
                      </span>
                    </DropdownMenuItem>
                    <DropdownMenuItem onClick={() => setTheme('dark')}>
                      <Moon className='size-4' />
                      <span>
                        {t('layout.sidebar.footer.userMenu.theme.dark')}
                      </span>
                    </DropdownMenuItem>
                    <DropdownMenuItem onClick={() => setTheme('system')}>
                      <Laptop className='size-4' />
                      <span>
                        {t('layout.sidebar.footer.userMenu.theme.system')}
                      </span>
                    </DropdownMenuItem>
                  </DropdownMenuGroup>
                </DropdownMenuSubContent>
              </DropdownMenuPortal>
            </DropdownMenuSub>

            <DropdownMenuSeparator />

            <DropdownMenuItem variant='destructive' onClick={logout}>
              {t('layout.sidebar.footer.userMenu.logout')}
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </SidebarMenu>
    </SidebarFooter>
  )
}
