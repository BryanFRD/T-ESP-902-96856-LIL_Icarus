import { AnimatePresence, motion } from 'framer-motion'
import { PanelLeft, PanelLeftClose } from 'lucide-react'
import React from 'react'

type SidebarToggleIconProps = {
  isOpen: boolean
  className?: string
  size?: number
}

const SidebarToggleIcon: React.FC<SidebarToggleIconProps> = ({
  isOpen,
  className = '',
  size = 20
}) => {
  return (
    <span className={`flex ${className}`}>
      <AnimatePresence mode='wait' initial={false}>
        {isOpen ? (
          <motion.span
            key='close'
            initial={{ opacity: 0, rotate: -90, scale: 0.8 }}
            animate={{ opacity: 1, rotate: 0, scale: 1 }}
            exit={{ opacity: 0, rotate: 90, scale: 0.8 }}
            transition={{ duration: 0.2 }}
            className='flex items-center justify-center'
          >
            <PanelLeftClose size={size} />
          </motion.span>
        ) : (
          <motion.span
            key='open'
            initial={{ opacity: 0, rotate: 90, scale: 0.8 }}
            animate={{ opacity: 1, rotate: 0, scale: 1 }}
            exit={{ opacity: 0, rotate: -90, scale: 0.8 }}
            transition={{ duration: 0.2 }}
            className='flex items-center justify-center'
          >
            <PanelLeft size={size} />
          </motion.span>
        )}
      </AnimatePresence>
    </span>
  )
}

export default SidebarToggleIcon
