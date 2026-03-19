import { format } from 'date-fns'
import { fr } from 'date-fns/locale'
import {
  AlertCircle,
  Calendar as CalendarIcon,
  ChevronDown,
  ChevronLeft,
  ChevronRight,
  Clock
} from 'lucide-react'
import { useEffect, useMemo, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { twMerge } from 'tailwind-merge'

import useInteractionOutside from '@/hooks/common/use-interaction-outside.ts'

type DatePickerProps = {
  label?: string
  value?: Date | null
  onChange: (date: Date | undefined) => void
  placeholder?: string
  error?: string
  disabled?: boolean
  className?: string
  minDate?: Date
  maxDate?: Date
  includeTime?: boolean
}

const DatePicker = ({
  label,
  value,
  onChange,
  placeholder,
  error,
  disabled = false,
  className,
  minDate,
  maxDate,
  includeTime = false
}: DatePickerProps) => {
  const { t } = useTranslation()
  const [isOpen, setIsOpen] = useState(false)
  const [currentMonth, setCurrentMonth] = useState(() => value || new Date())

  const [selectedTime, setSelectedTime] = useState({
    hours: value ? value.getHours() : 0,
    minutes: value ? value.getMinutes() : 0
  })

  const { ref: containerRef } = useInteractionOutside<HTMLDivElement>(() =>
    setIsOpen(false)
  )

  useEffect(() => {
    if (value) {
      setSelectedTime({ hours: value.getHours(), minutes: value.getMinutes() })
    }
  }, [value])

  const daysOfWeek = useMemo(
    () => [
      t('common:days.mon'),
      t('common:days.tue'),
      t('common:days.wed'),
      t('common:days.thu'),
      t('common:days.fri'),
      t('common:days.sat'),
      t('common:days.sun')
    ],
    [t]
  )

  const monthDays = useMemo(() => {
    const year = currentMonth.getFullYear()
    const month = currentMonth.getMonth()
    const firstDay = new Date(year, month, 1)
    const lastDay = new Date(year, month + 1, 0)
    const daysInMonth = lastDay.getDate()
    const startingDayOfWeek =
      firstDay.getDay() === 0 ? 6 : firstDay.getDay() - 1

    const days: (Date | null)[] = []

    for (let i = 0; i < startingDayOfWeek; i++) {
      const prevMonthDay = new Date(year, month, -startingDayOfWeek + i + 1)
      days.push(prevMonthDay)
    }

    for (let i = 1; i <= daysInMonth; i++) {
      days.push(new Date(year, month, i))
    }

    const remainingDays = 7 - (days.length % 7)
    if (remainingDays < 7) {
      for (let i = 1; i <= remainingDays; i++) {
        days.push(new Date(year, month + 1, i))
      }
    }

    return days
  }, [currentMonth])

  const isDateDisabled = (date: Date) => {
    const dateOnly = new Date(
      date.getFullYear(),
      date.getMonth(),
      date.getDate()
    )
    const minDateOnly = minDate
      ? new Date(minDate.getFullYear(), minDate.getMonth(), minDate.getDate())
      : null
    const maxDateOnly = maxDate
      ? new Date(maxDate.getFullYear(), maxDate.getMonth(), maxDate.getDate())
      : null

    if (minDateOnly && dateOnly < minDateOnly) return true
    if (maxDateOnly && dateOnly > maxDateOnly) return true
    return false
  }

  const isDateSelected = (date: Date) => {
    if (!value) return false
    return date.toDateString() === value.toDateString()
  }

  const isToday = (date: Date) => {
    const today = new Date()
    return date.toDateString() === today.toDateString()
  }

  const isCurrentMonth = (date: Date) => {
    return date.getMonth() === currentMonth.getMonth()
  }

  const handlePreviousMonth = (e: React.MouseEvent) => {
    e.preventDefault()
    e.stopPropagation()
    setCurrentMonth(
      new Date(currentMonth.getFullYear(), currentMonth.getMonth() - 1)
    )
  }

  const handleNextMonth = (e: React.MouseEvent) => {
    e.preventDefault()
    e.stopPropagation()
    setCurrentMonth(
      new Date(currentMonth.getFullYear(), currentMonth.getMonth() + 1)
    )
  }

  const handlePreviousYear = (e: React.MouseEvent) => {
    e.preventDefault()
    e.stopPropagation()
    setCurrentMonth(
      new Date(currentMonth.getFullYear() - 1, currentMonth.getMonth())
    )
  }

  const handleNextYear = (e: React.MouseEvent) => {
    e.preventDefault()
    e.stopPropagation()
    setCurrentMonth(
      new Date(currentMonth.getFullYear() + 1, currentMonth.getMonth())
    )
  }

  const handleDateSelect = (e: React.MouseEvent, date: Date) => {
    e.preventDefault()
    e.stopPropagation()
    if (isDateDisabled(date)) return

    const newDate = new Date(date)

    if (includeTime) {
      if (value) {
        newDate.setHours(value.getHours())
        newDate.setMinutes(value.getMinutes())
      } else {
        newDate.setHours(selectedTime.hours)
        newDate.setMinutes(selectedTime.minutes)
      }
    } else {
      newDate.setHours(0, 0, 0, 0)
    }

    onChange(newDate)
    if (!includeTime) {
      setIsOpen(false)
    }
  }

  const handleTimeChange = (type: 'hours' | 'minutes', val: string) => {
    let numVal = Number.parseInt(val, 10)
    if (Number.isNaN(numVal)) numVal = 0

    if (type === 'hours') {
      numVal = Math.max(0, Math.min(23, numVal))
      setSelectedTime(prev => ({ ...prev, hours: numVal }))

      if (value) {
        const newDate = new Date(value)
        newDate.setHours(numVal)
        onChange(newDate)
      }
    } else {
      numVal = Math.max(0, Math.min(59, numVal))
      setSelectedTime(prev => ({ ...prev, minutes: numVal }))

      if (value) {
        const newDate = new Date(value)
        newDate.setMinutes(numVal)
        onChange(newDate)
      }
    }
  }

  const handleToggle = (e: React.MouseEvent) => {
    e.preventDefault()
    e.stopPropagation()
    if (!disabled) {
      setIsOpen(!isOpen)
    }
  }

  const getBorderClasses = () => {
    if (error) return 'border-red-300 focus:border-red-500 dark:border-red-500'
    return 'border-gray-200 hover:border-gray-300 dark:border-gray-700 dark:hover:border-gray-600'
  }

  const finalPlaceholder = placeholder || t('common:selectDate')

  const displayFormat = includeTime ? 'dd MMMM yyyy HH:mm' : 'dd MMMM yyyy'

  return (
    <div
      className={twMerge('w-full space-y-1.5', className)}
      ref={containerRef}
    >
      {label && (
        <label className='flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200'>
          <CalendarIcon className='h-4 w-4' />
          {label}
        </label>
      )}

      <div className='relative'>
        <button
          type='button'
          disabled={disabled}
          onClick={handleToggle}
          className={twMerge(
            'relative flex min-h-10.5 w-full items-center justify-between gap-2 rounded-md border-2 px-3 py-2 text-left transition-all duration-200 outline-none',
            'bg-white dark:bg-gray-900',
            getBorderClasses(),
            disabled &&
              'cursor-not-allowed bg-gray-50 opacity-50 dark:bg-gray-800'
          )}
        >
          <span
            className={twMerge(
              'truncate text-sm',
              !value && 'text-gray-400 dark:text-gray-500'
            )}
          >
            {value
              ? format(value, displayFormat, { locale: fr })
              : finalPlaceholder}
          </span>

          <ChevronDown
            className={twMerge(
              'h-4 w-4 shrink-0 text-gray-400 transition-transform duration-200',
              isOpen && 'rotate-180'
            )}
          />
        </button>

        {isOpen && !disabled && (
          <div
            className='animate-in fade-in zoom-in-95 absolute top-full left-0 z-50 mt-1 w-auto min-w-full overflow-hidden rounded-md border border-gray-200 bg-white p-4 shadow-xl dark:border-gray-700 dark:bg-gray-900'
            onClick={e => e.stopPropagation()}
          >
            <div className='mb-4 flex items-center justify-between'>
              <button
                type='button'
                onClick={handlePreviousYear}
                className='flex h-8 w-8 items-center justify-center rounded-lg text-gray-600 transition-colors hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'
                aria-label={t('common:time.previousYear')}
              >
                <ChevronLeft className='h-4 w-4' />
                <ChevronLeft className='-ml-3 h-4 w-4' />
              </button>

              <button
                type='button'
                onClick={handlePreviousMonth}
                className='flex h-8 w-8 items-center justify-center rounded-lg text-gray-600 transition-colors hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'
                aria-label={t('common:time.previousMonth')}
              >
                <ChevronLeft className='h-4 w-4' />
              </button>

              <div className='min-w-40 text-center text-sm font-semibold text-gray-900 capitalize dark:text-gray-100'>
                {format(currentMonth, 'MMMM yyyy', { locale: fr })}
              </div>

              <button
                type='button'
                onClick={handleNextMonth}
                className='flex h-8 w-8 items-center justify-center rounded-lg text-gray-600 transition-colors hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'
                aria-label={t('common:time.nextMonth')}
              >
                <ChevronRight className='h-4 w-4' />
              </button>

              <button
                type='button'
                onClick={handleNextYear}
                className='flex h-8 w-8 items-center justify-center rounded-lg text-gray-600 transition-colors hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'
                aria-label={t('common:time.nextYear')}
              >
                <ChevronRight className='h-4 w-4' />
                <ChevronRight className='-ml-3 h-4 w-4' />
              </button>
            </div>

            <div className='grid grid-cols-7 gap-1'>
              {daysOfWeek.map(day => (
                <div
                  key={day}
                  className='flex h-9 items-center justify-center text-xs font-medium text-gray-500 dark:text-gray-400'
                >
                  {day}
                </div>
              ))}

              {monthDays.map((date, index) => {
                if (!date) return <div key={index} />

                const selected = isDateSelected(date)
                const today = isToday(date)
                const currentMonthDay = isCurrentMonth(date)
                const disabled = isDateDisabled(date)

                return (
                  <button
                    key={index}
                    type='button'
                    onClick={e => handleDateSelect(e, date)}
                    disabled={disabled}
                    className={twMerge(
                      'flex h-9 w-9 items-center justify-center rounded-lg text-sm transition-colors',
                      !currentMonthDay && 'text-gray-400 dark:text-gray-600',
                      currentMonthDay &&
                        !selected &&
                        !disabled &&
                        'text-gray-900 hover:bg-gray-100 dark:text-gray-100 dark:hover:bg-gray-800',
                      today &&
                        !selected &&
                        'bg-gray-100 font-semibold dark:bg-gray-800',
                      selected &&
                        'bg-orange-500 font-semibold text-white hover:bg-orange-600 dark:bg-orange-500 dark:hover:bg-orange-600',
                      disabled && 'cursor-not-allowed opacity-40'
                    )}
                    aria-label={format(date, 'dd MMMM yyyy', { locale: fr })}
                  >
                    {date.getDate()}
                  </button>
                )
              })}
            </div>

            {includeTime && (
              <div className='mt-4 border-t border-gray-100 pt-3 dark:border-gray-800'>
                <div className='flex items-center justify-between gap-2'>
                  <div className='flex items-center gap-2 text-sm text-gray-500 dark:text-gray-400'>
                    <Clock className='h-4 w-4' />
                    <span>{t('common:time.label', 'Heure')}</span>
                  </div>
                  <div className='flex items-center gap-1'>
                    <input
                      type='number'
                      min='0'
                      max='23'
                      value={selectedTime.hours.toString().padStart(2, '0')}
                      onChange={e => handleTimeChange('hours', e.target.value)}
                      className='w-12 rounded border border-gray-200 bg-gray-50 p-1 text-center text-sm text-gray-900 focus:border-orange-500 focus:ring-1 focus:ring-orange-500 dark:border-gray-700 dark:bg-gray-800 dark:text-white'
                      placeholder='HH'
                    />
                    <span className='font-bold text-gray-500'>:</span>
                    <input
                      type='number'
                      min='0'
                      max='59'
                      value={selectedTime.minutes.toString().padStart(2, '0')}
                      onChange={e =>
                        handleTimeChange('minutes', e.target.value)
                      }
                      className='w-12 rounded border border-gray-200 bg-gray-50 p-1 text-center text-sm text-gray-900 focus:border-orange-500 focus:ring-1 focus:ring-orange-500 dark:border-gray-700 dark:bg-gray-800 dark:text-white'
                      placeholder='MM'
                    />
                  </div>
                </div>
              </div>
            )}
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

export default DatePicker
