import { motion } from 'framer-motion'
import { Check } from 'lucide-react'
import type { LucideIcon } from 'lucide-react'
import { useMemo } from 'react'

interface Step {
  number: number
  title: string
  icon: LucideIcon
}

interface StepperProps {
  steps: Step[]
  currentStep: number
  className?: string
}

type StepStatus = 'completed' | 'current' | 'pending'

const getStepStatus = (stepNumber: number, currentStep: number): StepStatus => {
  if (currentStep > stepNumber) return 'completed'
  if (currentStep === stepNumber) return 'current'
  return 'pending'
}

const calculateProgress = (currentStep: number, totalSteps: number): number => {
  return ((currentStep - 1) / (totalSteps - 1)) * 100
}

const ProgressBar = ({ progress }: { progress: number }) => (
  <div className='absolute top-1/2 right-0 left-0 flex -translate-y-1/2 items-center px-[15.5%]'>
    <div className='relative h-0.5 w-full bg-gray-200 dark:bg-gray-700'>
      <motion.div
        className='absolute top-0 left-0 h-full bg-gradient-to-r from-red-500 to-orange-500 dark:from-red-600 dark:to-orange-600'
        initial={{ width: 0 }}
        animate={{ width: `${progress}%` }}
        transition={{ duration: 0.5, ease: 'easeInOut' }}
      />
    </div>
  </div>
)

interface StepCircleProps {
  Icon: LucideIcon
  status: StepStatus
}

const StepCircle = ({ Icon, status }: StepCircleProps) => {
  const isActive = status === 'current' || status === 'completed'

  const circleVariants = {
    completed: { scale: 1, opacity: 1 },
    current: { scale: 1.1, opacity: 1 },
    pending: { scale: 1, opacity: 1 }
  }

  const iconVariants = {
    hidden: { scale: 0, opacity: 0 },
    visible: { scale: 1, opacity: 1 }
  }

  return (
    <motion.div
      variants={circleVariants}
      animate={status}
      transition={{ duration: 0.3, ease: 'easeOut' }}
      className={`relative z-10 flex h-11 w-11 items-center justify-center rounded-full border-2 ${
        isActive
          ? 'border-transparent bg-orange-500'
          : 'border-gray-300 bg-white dark:border-gray-600 dark:bg-gray-800'
      } `}
    >
      {status === 'completed' ? (
        <motion.div
          variants={iconVariants}
          initial='hidden'
          animate='visible'
          transition={{ duration: 0.2, delay: 0.1 }}
        >
          <Check className='h-5 w-5 text-white' strokeWidth={3} />
        </motion.div>
      ) : (
        <Icon
          className={`h-5 w-5 ${
            isActive ? 'text-white' : 'text-gray-400 dark:text-gray-500'
          }`}
          strokeWidth={2}
        />
      )}
    </motion.div>
  )
}

interface StepLabelProps {
  title: string
  status: StepStatus
}

const StepLabel = ({ title, status }: StepLabelProps) => {
  const colorClasses = {
    completed: 'text-gray-700 dark:text-gray-300',
    current: 'text-orange-600 dark:text-orange-400 font-bold',
    pending: 'text-gray-500 dark:text-gray-400'
  }

  return (
    <motion.span
      initial={{ opacity: 0, y: 5 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.3 }}
      className={`mt-3 max-w-[120px] text-center text-xs font-semibold transition-colors duration-300 ${colorClasses[status]} `}
    >
      {title}
    </motion.span>
  )
}

const Stepper = ({ steps, currentStep, className = '' }: StepperProps) => {
  const progress = useMemo(
    () => calculateProgress(currentStep, steps.length),
    [currentStep, steps.length]
  )

  return (
    <div className={`relative w-full px-8 py-6 ${className}`}>
      <div className='relative flex items-start justify-between'>
        <div className='absolute top-[1.375rem] right-0 left-0'>
          <ProgressBar progress={progress} />
        </div>

        {steps.map(step => {
          const status = getStepStatus(step.number, currentStep)

          return (
            <div
              key={step.number}
              className='flex flex-col items-center'
              style={{ width: `${100 / steps.length}%` }}
            >
              <StepCircle Icon={step.icon} status={status} />
              <StepLabel title={step.title} status={status} />
            </div>
          )
        })}
      </div>
    </div>
  )
}

export default Stepper
