import type { LucideIcon } from 'lucide-react'
import type { ReactNode } from 'react'

export const buttonVariants = [
  'primary',
  'secondary',
  'danger',
  'ghost',
  'outline',
  'navigationActive',
  'text'
] as const

export type ButtonVariant = (typeof buttonVariants)[number]

export const buttonSizes = ['sm', 'md', 'lg'] as const
export const buttonRounded = ['none', 'sm', 'md', 'lg', 'full'] as const

export type ButtonSize = (typeof buttonSizes)[number]
export type ButtonRounded = (typeof buttonRounded)[number]

export type ButtonIcon = {
  type: LucideIcon
  position: 'left' | 'right'
}

export type ButtonProps = {
  children: ReactNode
  onClick?: () => void
  type?: 'button' | 'submit' | 'reset'
  variant?: ButtonVariant
  size?: ButtonSize
  rounded?: ButtonRounded
  fullWidth?: boolean
  isLoading?: boolean
  disabled?: boolean
  className?: string
  loadingText?: string
  ariaLabel?: string
  withBorder?: boolean
  withHover?: boolean
  as?: 'button' | 'link'
  to?: string
  icon?: ButtonIcon
}
