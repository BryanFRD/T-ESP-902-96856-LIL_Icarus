import { AlertCircle } from 'lucide-react'
import { forwardRef } from 'react'
import { twMerge } from 'tailwind-merge'

type TextareaProps = {
  label?: string
  error?: string
  className?: string
  rows?: number
} & React.TextareaHTMLAttributes<HTMLTextAreaElement>

const Textarea = forwardRef<HTMLTextAreaElement, TextareaProps>(
  ({ label, error, className, rows = 3, ...props }, ref) => {
    const getBorderClasses = () => {
      if (error)
        return 'border-red-300 focus:border-red-500 dark:border-red-500'
      return 'border-gray-200 focus:border-orange-500 dark:border-gray-700 dark:focus:border-orange-500'
    }

    return (
      <div className={twMerge('w-full space-y-1.5', className)}>
        {label && (
          <label className='text-sm font-medium text-gray-700 dark:text-gray-200'>
            {label}
          </label>
        )}

        <textarea
          ref={ref}
          rows={rows}
          className={twMerge(
            'w-full rounded-md border-2 px-3 py-2 text-sm transition-all duration-200 outline-none',
            'bg-white dark:bg-gray-900',
            'text-gray-900 dark:text-gray-100',
            'placeholder:text-gray-400 dark:placeholder:text-gray-500',
            'resize-y',
            getBorderClasses(),
            props.disabled &&
              'cursor-not-allowed bg-gray-50 opacity-50 dark:bg-gray-800'
          )}
          {...props}
        />

        {error && (
          <div className='animate-in fade-in slide-in-from-top-1 flex items-center gap-2 text-sm text-red-600 dark:text-red-400'>
            <AlertCircle className='h-4 w-4' />
            {error}
          </div>
        )}
      </div>
    )
  }
)

Textarea.displayName = 'Textarea'

export default Textarea
