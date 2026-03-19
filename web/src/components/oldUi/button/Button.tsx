import { Loader2 } from 'lucide-react'
import React, { createElement } from 'react'
import { Link } from 'react-router-dom'
import { twMerge } from 'tailwind-merge'

import type { ButtonProps } from '@/types/components/button.type.ts'

const sizeClasses = {
  sm: 'px-2 py-1.5 text-sm',
  md: 'px-3 py-2 text-medium',
  lg: 'px-3 py-3 text-lg'
}

const roundedClasses = {
  none: 'rounded-none',
  sm: 'rounded-md',
  md: 'rounded-xl',
  lg: 'rounded-2xl',
  full: 'rounded-full'
}

const variantClasses = {
  primary: `
    text-white
    bg-gradient-to-r from-red-500 to-orange-500 
    hover:from-red-600 hover:to-orange-600
    dark:from-red-600 dark:to-orange-600
    dark;hover:from-red-700 dark:hover:to-orange-700
  `,
  secondary: `
    text-gray-900 bg-gray-100 hover:bg-gray-200
    dark:text-white dark:bg-gray-700 dark:hover:bg-gray-600
  `,
  danger: `
    text-white bg-red-500 hover:bg-red-600
    dark:bg-red-600 dark:hover:bg-red-700
  `,
  ghost: `
    text-gray-700 hover:bg-gray-200
    dark:text-gray-300 dark:hover:bg-gray-700
    border border-transparent
  `,
  outline: `
    text-gray-800 border border-gray-400 hover:bg-gray-200
    dark:text-gray-100 dark:border-gray-600 dark:hover:bg-gray-700
  `,
  navigationActive: `
    text-white border border-red-500/30
    bg-gradient-to-r from-red-500/80 to-orange-500/80
    dark:from-red-600/60 dark:to-orange-600/60
  `,
  text: `
    text-gray-700
    dark:text-gray-300
    border border-transparent
    `
}

const Button: React.FC<ButtonProps> = ({
  children,
  onClick,
  type = 'button',
  variant = 'primary',
  size = 'md',
  rounded = 'md',
  fullWidth = false,
  isLoading = false,
  disabled = false,
  className = '',
  loadingText = '',
  ariaLabel,
  withBorder = true,
  as = 'button',
  to,
  icon
}) => {
  const baseClasses = twMerge(
    `inline-flex items-center justify-center gap-2 font-semibold transition-all duration-300 border border-transparent overflow-hidden hover:cursor-pointer ${fullWidth ? 'w-full' : ''} ${disabled || isLoading ? 'cursor-not-allowed opacity-50' : ''}`,
    sizeClasses[size],
    roundedClasses[rounded],
    variantClasses[variant],
    withBorder ? 'border' : 'border-0',
    className
  )

  const content = isLoading ? (
    <>
      <Loader2 className='h-5 w-5 animate-spin' />
      {loadingText}
    </>
  ) : (
    <>
      {icon?.position === 'left' && (
        <span>{createElement(icon.type, { className: 'h-5 w-5' })}</span>
      )}
      {children}
      {icon?.position === 'right' && (
        <span>{createElement(icon.type, { className: 'h-5 w-5' })}</span>
      )}
    </>
  )

  if (as === 'link' && to) {
    return (
      <Link
        to={to}
        className={baseClasses}
        aria-label={ariaLabel}
        onClick={onClick}
      >
        {content}
      </Link>
    )
  }

  return (
    <button
      type={type}
      onClick={onClick}
      disabled={disabled || isLoading}
      className={baseClasses}
      aria-label={ariaLabel}
    >
      {content}
    </button>
  )
}

export default Button
