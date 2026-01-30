import type { LucideIcon } from 'lucide-react'
import type {
  HTMLInputAutoCompleteAttribute,
  ReactNode,
  RefObject
} from 'react'
import type { FieldValues, Path, UseFormRegister } from 'react-hook-form'

export type Language = 'en' | 'fr'

export interface MenuItem {
  icon: LucideIcon
  label: string
  href: string
  isAdminOnly?: boolean
}

export interface SidebarProps {
  isOpen: boolean
  onClose: () => void
  onToggle: () => void
  menuItems: MenuItem[]
  openSidebar?: () => void
}

export interface InputFieldProps<T extends FieldValues> {
  label: string
  type: 'email' | 'password' | 'text' | 'number' | 'datetime-local'
  icon: LucideIcon
  placeholder?: string
  register: UseFormRegister<T>
  error?: string
  name: Path<T>
  autoComplete?: HTMLInputAutoCompleteAttribute
  registerOptions?: Parameters<UseFormRegister<T>>[1]
}

export interface LoadingButtonProps {
  isLoading: boolean
  children: ReactNode
  loadingText?: string
  className?: string
  disabled?: boolean
  type?: 'button' | 'submit' | 'reset'
}

export interface SocialButtonProps {
  icon: ReactNode
  label: string
  onClick?: () => void
}

export interface FeatureItemProps {
  icon: LucideIcon
  title: string
  description: string
}

export interface SidebarToggleProps {
  isOpen: boolean
  onToggle: () => void
  className?: string
}

export interface NavigationLinkProps {
  item: MenuItem
  isActive: boolean
  isCollapsed?: boolean
}

export interface AxiosApiResponse {
  errorKey?: string
  error?: string
}

export interface UseInteractionOutsideOptions {
  enabled?: boolean
  eventType?:
    | 'click'
    | 'mousedown'
    | 'touchstart'
    | 'pointerdown'
    | 'keydown'
    | 'mouseup'
    | 'touchend'
    | 'pointerup'
  ignoreEscapeKey?: boolean
  ignoreElements?: string[]
}

export type UseInteractionOutsideReturn<T extends HTMLElement> = {
  ref: RefObject<T>
}

export type RouteType = {
  name: string
  path: string
  routeKey: string
  component: React.ComponentType<unknown>
  isAuthRequired?: boolean
  authorizedRoles?: string[]
}

export type UseRedirectOptions = {
  when: boolean
  to?: string
  replace?: boolean
  fromQueryParam?: string
  validatePath?: (path: string) => boolean
  state?: Record<string, unknown> | null
}
