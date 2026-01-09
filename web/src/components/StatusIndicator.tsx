import { Zap } from 'lucide-react'
import { useTranslation } from 'react-i18next'

import ThreatLevel from '@/components/ThreatLevel.tsx'
import type { ServerStatusEnum } from '@/domain/enums/serverStatusEnum.ts'

const StatusIndicator = ({ status }: { status?: ServerStatusEnum }) => {
  const { t } = useTranslation()

  if (!status) return null

  return (
    <div className='flex items-center space-x-3 rounded-xl bg-gray-800/30 px-4 py-2 backdrop-blur-sm'>
      <div className='flex items-center space-x-2'>
        <Zap size={16} className='text-green-400' />
        <span className='text-sm text-gray-300'>
          {t('serverStatus', { status })}
        </span>
      </div>
      <div className='h-4 w-px bg-gray-700'></div>
      <ThreatLevel level='medium' />
    </div>
  )
}

export default StatusIndicator
