import { useTranslation } from 'react-i18next'

const ThreatLevel = ({
  level
}: {
  level: 'low' | 'medium' | 'high' | 'critical'
}) => {
  const { t } = useTranslation()

  const colors = {
    low: 'bg-green-500',
    medium: 'bg-yellow-500',
    high: 'bg-orange-500',
    critical: 'bg-red-500'
  }

  return (
    <div className='flex items-center space-x-2'>
      <div
        className={`h-2 w-2 ${colors[level]} animate-pulse rounded-full`}
      ></div>
      <span className='text-sm text-gray-300 capitalize'>
        {t(`threatLevel.${level}`)}
      </span>
    </div>
  )
}

export default ThreatLevel
