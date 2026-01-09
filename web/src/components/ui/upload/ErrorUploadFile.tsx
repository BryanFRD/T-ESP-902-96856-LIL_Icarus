import { t } from 'i18next'
import { AlertCircle } from 'lucide-react'

interface ErrorUploadFileProps {
  errors: string[]
  compact?: boolean
}

export const ErrorUploadFile = ({
  errors,
  compact = false
}: ErrorUploadFileProps) => {
  if (errors.length === 0) return null

  return (
    <div
      className={`rounded-lg border border-red-200 bg-red-50 dark:border-red-800 dark:bg-red-900/20 ${compact ? 'mt-3 p-3' : 'mt-4 p-4'}`}
    >
      <div className='flex items-start space-x-2'>
        <AlertCircle
          className={`mt-0.5 flex-shrink-0 text-red-500 dark:text-red-400 ${compact ? 'h-4 w-4' : 'h-5 w-5'}`}
        />
        <div>
          <h4
            className={`font-medium text-red-800 dark:text-red-200 ${compact ? 'text-sm' : 'mb-1'}`}
          >
            {t('file_upload.validation_errors')}
          </h4>
          <ul
            className={`space-y-1 text-red-700 dark:text-red-300 ${compact ? 'mt-1 text-xs' : 'text-sm'}`}
          >
            {errors.map(error => (
              <li key={error}>• {error}</li>
            ))}
          </ul>
        </div>
      </div>
    </div>
  )
}
