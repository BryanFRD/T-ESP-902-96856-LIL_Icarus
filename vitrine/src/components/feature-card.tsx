'use client'

import { motion } from 'framer-motion'
import type { LucideIcon } from 'lucide-react'

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle
} from '@/components/ui/card'

interface FeatureCardProps {
  title: string
  description: string
  icon: LucideIcon
  index?: number
  variant?: 'default' | 'primary' | 'destructive'
}

export function FeatureCard({
  title,
  description,
  icon: Icon,
  index = 0,
  variant = 'default'
}: FeatureCardProps) {
  const variants = {
    default: 'hover:border-primary hover:shadow-md',
    primary:
      'border-primary/20 bg-primary/5 hover:border-primary hover:shadow-md',
    destructive:
      'border-destructive/20 bg-destructive/5 hover:border-destructive hover:shadow-md'
  }

  const iconVariants = {
    default: 'bg-primary/10 text-primary',
    primary: 'bg-primary/20 text-primary',
    destructive: 'bg-destructive/10 text-destructive'
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      whileInView={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.5, delay: index * 0.1 }}
      viewport={{ once: true }}
    >
      <Card className={`h-full transition-all ${variants[variant]}`}>
        <CardHeader>
          <div
            className={`mb-3 flex h-12 w-12 items-center justify-center rounded-lg ${iconVariants[variant]}`}
          >
            <Icon className='h-6 w-6' />
          </div>
          <CardTitle className='text-lg'>{title}</CardTitle>
        </CardHeader>
        <CardContent>
          <CardDescription>{description}</CardDescription>
        </CardContent>
      </Card>
    </motion.div>
  )
}
