import { Check } from 'lucide-react'
import { twMerge } from 'tailwind-merge'

type CheckboxProps = {
  checked?: boolean
  onCheckedChange: (checked: boolean) => void
  label?: string
  disabled?: boolean
  className?: string
}

const Checkbox = ({
  checked = false,
  onCheckedChange,
  label,
  disabled = false,
  className
}: CheckboxProps) => {
  return (
    <button
      type='button'
      role='checkbox'
      aria-checked={checked}
      disabled={disabled}
      onClick={e => {
        e.stopPropagation()
        onCheckedChange(!checked)
      }}
      className={twMerge(
        'group flex cursor-pointer items-center gap-2 text-left outline-none select-none',
        disabled && 'cursor-not-allowed opacity-50',
        className
      )}
    >
      <div
        className={twMerge(
          'flex h-4 w-4 shrink-0 items-center justify-center rounded border transition-all duration-200',
          'group-focus-visible:ring-2 group-focus-visible:ring-blue-500 group-focus-visible:ring-offset-2 dark:group-focus-visible:ring-offset-gray-900',
          checked
            ? 'border-orange-600 bg-orange-500 text-white dark:border-blue-600 dark:bg-blue-600'
            : 'border-gray-300 bg-white dark:border-gray-600 dark:bg-gray-800',
          disabled && 'bg-gray-100 dark:bg-gray-700'
        )}
      >
        {checked && <Check className='h-3 w-3' strokeWidth={3} />}
      </div>

      {label && (
        <span className='text-sm text-gray-700 dark:text-gray-200'>
          {label}
        </span>
      )}
    </button>
  )
}

export default Checkbox
