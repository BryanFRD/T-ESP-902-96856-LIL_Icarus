import { t } from 'i18next'
import {
  AlertCircle,
  Check,
  ChevronDown,
  Loader2,
  Search,
  X
} from 'lucide-react'
import React, { useEffect, useMemo, useRef, useState } from 'react'
import { twMerge } from 'tailwind-merge'

import Checkbox from '@/components/ui/Checkbox'
import useInteractionOutside from '@/hooks/common/useInteractionOutside.ts'

export type SelectOption = {
  label: string
  value: string | number
  image?: string
  disabled?: boolean
} & Record<string, unknown>

export type SelectProps = {
  label?: string
  placeholder?: string
  options?: SelectOption[]
  value?: SelectOption | SelectOption[] | null
  onChange: (value: SelectOption | SelectOption[] | null) => void
  isMulti?: boolean
  isAsync?: boolean
  isSearchable?: boolean
  isLoading?: boolean
  disabled?: boolean
  loadOptions?: (inputValue: string) => Promise<SelectOption[]>
  error?: string
  icon?: React.ElementType
  className?: string
  noOptionsMessage?: string
}

const Select = ({
  label,
  placeholder = t('select.placeholder'),
  options = [],
  value,
  onChange,
  isMulti = false,
  isAsync = false,
  isSearchable = false,
  isLoading = false,
  disabled = false,
  loadOptions,
  error,
  icon: Icon,
  className,
  noOptionsMessage = t('select.noOptions')
}: SelectProps) => {
  const [isOpen, setIsOpen] = useState(false)
  const [searchTerm, setSearchTerm] = useState('')
  const [asyncOptions, setAsyncOptions] = useState<SelectOption[]>([])
  const [isInternalLoading, setIsInternalLoading] = useState(false)
  const searchInputRef = useRef<HTMLInputElement>(null)

  const { ref: containerRef } = useInteractionOutside<HTMLDivElement>(() => {
    setIsOpen(false)
    setSearchTerm('')
  })

  useEffect(() => {
    if (isAsync && loadOptions && isOpen) {
      const timer = setTimeout(async () => {
        setIsInternalLoading(true)
        try {
          const results = await loadOptions(searchTerm)
          setAsyncOptions(results)
        } catch {
          setAsyncOptions([])
        } finally {
          setIsInternalLoading(false)
        }
      }, 300)
      return () => clearTimeout(timer)
    }
  }, [isAsync, loadOptions, searchTerm, isOpen])

  useEffect(() => {
    if (isOpen && (isSearchable || isAsync)) {
      requestAnimationFrame(() => searchInputRef.current?.focus())
    }
  }, [isOpen, isSearchable, isAsync])

  const displayOptions = useMemo(() => {
    if (isAsync) return asyncOptions
    if (!searchTerm) return options
    const lowerTerm = searchTerm.toLowerCase()
    return options.filter(opt => opt.label.toLowerCase().includes(lowerTerm))
  }, [options, asyncOptions, searchTerm, isAsync])

  const currentValues = useMemo(() => {
    if (!value) return []
    return Array.isArray(value) ? value : [value]
  }, [value])

  const selectedValuesSet = useMemo(() => {
    return new Set(currentValues.map(v => v.value))
  }, [currentValues])

  const isAllSelected = useMemo(() => {
    if (!isMulti || displayOptions.length === 0) return false
    const visibleIds = displayOptions.filter(o => !o.disabled).map(o => o.value)
    return visibleIds.every(id => selectedValuesSet.has(id))
  }, [isMulti, displayOptions, selectedValuesSet])

  const handleSelect = (e: React.MouseEvent, option: SelectOption) => {
    e.preventDefault()
    e.stopPropagation()

    if (disabled || option.disabled) return

    if (isMulti) {
      const exists = selectedValuesSet.has(option.value)
      const newValues = exists
        ? currentValues.filter(v => v.value !== option.value)
        : [...currentValues, option]
      onChange(newValues)
    } else {
      onChange(option)
      setIsOpen(false)
      setSearchTerm('')
    }
  }

  const handleSelectAll = (e: React.MouseEvent) => {
    e.preventDefault()
    e.stopPropagation()

    if (!isMulti) return
    const visibleAndEnabled = displayOptions.filter(o => !o.disabled)

    if (isAllSelected) {
      const idsToRemove = new Set(visibleAndEnabled.map(o => o.value))
      onChange(currentValues.filter(v => !idsToRemove.has(v.value)))
    } else {
      const newValues = [...currentValues]
      visibleAndEnabled.forEach(opt => {
        if (!selectedValuesSet.has(opt.value)) {
          newValues.push(opt)
        }
      })
      onChange(newValues)
    }
  }

  const removeValue = (e: React.MouseEvent, optionToRemove: SelectOption) => {
    e.stopPropagation()

    if (isMulti) {
      onChange(currentValues.filter(v => v.value !== optionToRemove.value))
    } else {
      onChange(null)
    }
  }

  const getBorderClasses = () => {
    if (error) {
      return 'border-red-300 focus:border-red-500 dark:border-red-500'
    }
    return 'border-gray-200 hover:border-gray-300 dark:border-gray-700 dark:hover:border-gray-600'
  }

  const renderTriggerContent = () => {
    if (currentValues.length === 0) {
      return (
        <span className='truncate text-gray-400 dark:text-gray-500'>
          {placeholder}
        </span>
      )
    }

    if (isMulti) {
      return currentValues.map(val => (
        <span
          key={val.value}
          className='animate-in fade-in zoom-in-95 inline-flex items-center gap-1 rounded-md bg-blue-100 px-1.5 py-0.5 text-xs font-medium text-blue-800 dark:bg-blue-900/40 dark:text-blue-200'
        >
          {val.image && (
            <img
              src={val.image}
              alt=''
              className='h-3.5 w-3.5 rounded-full object-cover'
            />
          )}
          <span className='max-w-37.5 truncate'>{val.label}</span>

          <button
            type='button'
            onClick={e => removeValue(e, val)}
            className='ml-0.5 flex h-4 w-4 shrink-0 items-center justify-center rounded-full p-0.5 hover:bg-blue-200/50 focus:ring-2 focus:ring-blue-500 focus:outline-none dark:hover:bg-blue-800/50'
            aria-label={`Supprimer ${val.label}`}
          >
            <X className='h-3 w-3' />
          </button>
        </span>
      ))
    }

    return (
      <div className='flex items-center gap-2 truncate'>
        {currentValues[0].image && (
          <img
            src={currentValues[0].image}
            alt=''
            className='h-5 w-5 shrink-0 rounded-full object-cover'
          />
        )}
        <span className='truncate font-medium text-gray-900 dark:text-gray-100'>
          {currentValues[0].label}
        </span>
      </div>
    )
  }

  const renderOptionsList = () => {
    const loading = isLoading || isInternalLoading

    if (loading && asyncOptions.length === 0) {
      return (
        <div className='px-4 py-3 text-center text-sm text-gray-500 italic'>
          Chargement...
        </div>
      )
    }

    if (displayOptions.length === 0) {
      return (
        <div className='px-4 py-3 text-center text-sm text-gray-500 italic'>
          {noOptionsMessage}
        </div>
      )
    }

    return (
      <>
        {isMulti && !loading && (
          <button
            type='button'
            onClick={handleSelectAll}
            className='flex w-full cursor-pointer items-center gap-3 px-3 py-2 text-left text-sm font-medium text-gray-900 hover:bg-gray-100 focus:bg-gray-100 focus:outline-none dark:text-gray-100 dark:hover:bg-gray-800 dark:focus:bg-gray-800'
          >
            <Checkbox checked={isAllSelected} onCheckedChange={() => {}} />
            <span>{t('select.selectAll', 'Tout sélectionner')}</span>
          </button>
        )}

        {displayOptions.map(option => {
          const selected = selectedValuesSet.has(option.value)

          return (
            <button
              type='button'
              key={option.value}
              disabled={option.disabled}
              onClick={e => handleSelect(e, option)}
              className={twMerge(
                'flex w-full cursor-pointer items-center justify-between px-3 py-2 text-left text-sm transition-colors focus:outline-none',
                option.disabled
                  ? 'cursor-not-allowed opacity-50'
                  : 'hover:bg-gray-100 focus:bg-gray-100 dark:hover:bg-gray-800 dark:focus:bg-gray-800',
                selected &&
                  !isMulti &&
                  'bg-blue-50 text-blue-900 dark:bg-blue-900/30 dark:text-blue-100'
              )}
            >
              <div className='flex items-center gap-3 overflow-hidden'>
                {isMulti && (
                  <Checkbox
                    checked={selected}
                    disabled={option.disabled}
                    onCheckedChange={() => {}}
                  />
                )}
                {option.image && (
                  <img
                    src={option.image}
                    alt=''
                    className='h-6 w-6 shrink-0 rounded-full border border-gray-200 object-cover dark:border-gray-700'
                  />
                )}
                <span
                  className={twMerge('truncate', selected && 'font-medium')}
                >
                  {option.label}
                </span>
              </div>
              {!isMulti && selected && (
                <Check className='h-4 w-4 shrink-0 text-blue-600 dark:text-blue-400' />
              )}
            </button>
          )
        })}
      </>
    )
  }

  const loading = isLoading || isInternalLoading

  return (
    <div
      className={twMerge('w-full space-y-1.5', className)}
      ref={containerRef}
    >
      {label && (
        <label className='flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200'>
          {Icon && <Icon className='h-4 w-4' />}
          {label}
        </label>
      )}

      <div className='relative'>
        <button
          type='button'
          disabled={disabled}
          onClick={() => setIsOpen(!isOpen)}
          className={twMerge(
            'relative flex min-h-10.5 w-full items-center justify-between gap-2 rounded-md border-2 px-3 py-2 text-left transition-all duration-200 outline-none',
            'bg-white dark:bg-gray-900',
            getBorderClasses(),
            disabled &&
              'cursor-not-allowed bg-gray-50 opacity-50 dark:bg-gray-800'
          )}
        >
          <div className='flex flex-wrap gap-1.5 overflow-hidden'>
            {renderTriggerContent()}
          </div>

          <div className='flex shrink-0 items-center gap-2'>
            {loading ? (
              <Loader2 className='h-4 w-4 animate-spin text-gray-400' />
            ) : (
              <ChevronDown
                className={twMerge(
                  'h-4 w-4 text-gray-400 transition-transform duration-200',
                  isOpen && 'rotate-180'
                )}
              />
            )}
          </div>
        </button>

        {isOpen && !disabled && (
          <div className='animate-in fade-in zoom-in-95 absolute top-full left-0 z-50 mt-1 max-h-60 w-full overflow-hidden rounded-md border border-gray-200 bg-white shadow-xs dark:border-gray-700 dark:bg-gray-900'>
            {(isSearchable || isAsync) && (
              <div className='border-b border-gray-100 p-2 dark:border-gray-800'>
                <div className='flex items-center gap-2 rounded-md bg-gray-50 px-2 py-1.5 ring-1 ring-gray-200 focus-within:ring-2'>
                  <Search className='h-4 w-4 text-gray-400' />
                  <input
                    ref={searchInputRef}
                    type='text'
                    className='w-full bg-transparent text-sm text-gray-900 outline-none placeholder:text-gray-400 dark:text-gray-100'
                    placeholder={t('select.search')}
                    value={searchTerm}
                    onChange={e => setSearchTerm(e.target.value)}
                    onClick={e => e.stopPropagation()}
                    onKeyDown={e => e.stopPropagation()}
                  />
                </div>
              </div>
            )}

            <div className='max-h-50 overflow-auto py-1'>
              {renderOptionsList()}
            </div>
          </div>
        )}
      </div>

      {error && (
        <div className='animate-in fade-in slide-in-from-top-1 flex items-center gap-2 text-sm text-red-600 dark:text-red-400'>
          <AlertCircle className='h-4 w-4' />
          {error}
        </div>
      )}
    </div>
  )
}

export default Select
