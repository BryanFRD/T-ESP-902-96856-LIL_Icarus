import { AlertCircle, Eye, EyeOff } from 'lucide-react'
import type { FieldValues } from 'react-hook-form'

import useToggle from '@/hooks/common/use-toggle.ts'
import type { InputFieldProps } from '@/types/global.ts'

const InputField = <T extends FieldValues>({
  label,
  type,
  icon: Icon,
  placeholder,
  register,
  error,
  name,
  autoComplete = 'off',
  registerOptions
}: InputFieldProps<T>) => {
  const { toggle: togglePassword, value: showPassword } = useToggle(false)
  const inputType = type === 'password' && showPassword ? 'text' : type

  return (
    <div className='space-y-2'>
      <label className='flex items-center gap-2 text-sm font-medium text-gray-700 transition-all duration-300 dark:text-gray-200'>
        <Icon className='h-4 w-4' />
        {label}
      </label>
      <div className='relative'>
        <input
          type={inputType}
          {...register(name, registerOptions)}
          className={`w-full px-3 py-3 transition-all duration-300 ${
            type === 'password' ? 'pr-12' : 'pr-4'
          } rounded-md border-2 bg-white focus:ring-4 focus:ring-blue-100 focus:outline-none dark:border-gray-700 dark:bg-gray-900 dark:focus:ring-orange-900 ${
            error
              ? 'border-red-300 focus:border-red-500 dark:border-red-500'
              : 'border-gray-200'
          } text-gray-900 placeholder-gray-400 dark:text-gray-100 dark:placeholder-gray-500`}
          placeholder={placeholder}
          autoComplete={autoComplete}
        />

        {type === 'password' && (
          <button
            type='button'
            onClick={togglePassword}
            className='absolute top-1/2 right-4 -translate-y-1/2 transform text-gray-400 transition-all duration-300 hover:text-gray-600 dark:text-gray-500 dark:hover:text-gray-300'
          >
            {showPassword ? (
              <EyeOff className='h-5 w-5 cursor-pointer' />
            ) : (
              <Eye className='h-5 w-5 cursor-pointer' />
            )}
          </button>
        )}
      </div>

      {error && (
        <div className='mt-1 flex items-center gap-2 text-sm text-red-600 transition-all duration-300 dark:text-red-400'>
          <AlertCircle className='h-4 w-4' />
          {error}
        </div>
      )}
    </div>
  )
}

export default InputField
