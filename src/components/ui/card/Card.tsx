import { motion } from 'framer-motion'
import { type ReactNode } from 'react'

export interface CardProps {
  children: ReactNode
  className?: string
  hover?: boolean
  padding?: 'none' | 'sm' | 'md' | 'lg'
  variant?: 'default' | 'outlined' | 'elevated'
  onClick?: () => void
  animate?: boolean
  animationDelay?: number
}

const paddingClasses = {
  none: '',
  sm: 'p-4',
  md: 'p-5',
  lg: 'p-6'
}

const variantClasses = {
  default:
    'bg-white border border-gray-200 dark:bg-gray-900 dark:border-gray-800',
  outlined: 'bg-transparent border border-gray-200 dark:border-gray-700',
  elevated:
    'bg-white shadow-md border border-gray-100 dark:bg-gray-900 dark:border-gray-800'
}

const Card = ({
  children,
  className = '',
  hover = false,
  padding = 'md',
  variant = 'default',
  onClick,
  animate = false,
  animationDelay = 0
}: CardProps) => {
  const baseClasses = `
    rounded-xl
    transition-all
    duration-300
    ${variantClasses[variant]}
    ${paddingClasses[padding]}
    ${hover ? 'hover:border-orange-200 hover:shadow-lg dark:hover:border-orange-900/50 cursor-pointer' : ''}
    ${onClick ? 'cursor-pointer' : ''}
    ${className}
  `
    .trim()
    .replace(/\s+/g, ' ')

  if (animate) {
    return (
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: animationDelay }}
        className={baseClasses}
        onClick={onClick}
      >
        {children}
      </motion.div>
    )
  }

  return (
    <div className={baseClasses} onClick={onClick}>
      {children}
    </div>
  )
}

export interface CardHeaderProps {
  children: ReactNode
  className?: string
}

const CardHeader = ({ children, className = '' }: CardHeaderProps) => {
  return <div className={`mb-4 ${className}`}>{children}</div>
}

export interface CardTitleProps {
  children: ReactNode
  className?: string
}

const CardTitle = ({ children, className = '' }: CardTitleProps) => {
  return (
    <h3
      className={`text-lg font-bold text-gray-900 dark:text-gray-100 ${className}`}
    >
      {children}
    </h3>
  )
}

export interface CardDescriptionProps {
  children: ReactNode
  className?: string
}

const CardDescription = ({
  children,
  className = ''
}: CardDescriptionProps) => {
  return (
    <p className={`text-sm text-gray-500 dark:text-gray-400 ${className}`}>
      {children}
    </p>
  )
}

export interface CardContentProps {
  children: ReactNode
  className?: string
}

const CardContent = ({ children, className = '' }: CardContentProps) => {
  return <div className={className}>{children}</div>
}

export interface CardFooterProps {
  children: ReactNode
  className?: string
}

const CardFooter = ({ children, className = '' }: CardFooterProps) => {
  return (
    <div className={`mt-4 flex items-center gap-2 ${className}`}>
      {children}
    </div>
  )
}

export { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter }
