import { motion } from 'framer-motion'
import { ChevronRight, Users } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Link } from 'react-router-dom'

import type { Team } from '@/domain/types/Team.ts'

interface TeamCardProps {
  team: Team
  index?: number
}

const TeamCard = ({ team, index = 0 }: TeamCardProps) => {
  const { t } = useTranslation()

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ delay: index * 0.1 }}
    >
      <Link
        to={`/teams/${team.id}`}
        className='group block space-y-4 rounded-lg border border-gray-200 bg-white p-6 transition-all duration-300 hover:border-orange-300 hover:shadow-lg dark:border-gray-700 dark:bg-gray-800 dark:hover:border-orange-500'
      >
        <div className='flex items-start justify-between'>
          <div className='flex items-center gap-4'>
            <div className='flex size-12 items-center justify-center rounded-lg bg-gradient-to-r from-red-500 to-orange-500'>
              <Users className='size-6 text-white' />
            </div>
            <div className='space-y-1'>
              <h3 className='font-semibold text-gray-900 dark:text-white'>
                {team.name}
              </h3>
              {team.description && (
                <p className='text-sm text-gray-500 dark:text-gray-400'>
                  {team.description}
                </p>
              )}
            </div>
          </div>
          <ChevronRight className='size-5 text-gray-400 transition-transform group-hover:translate-x-1 dark:text-gray-500' />
        </div>

        {team.members && (
          <div className='flex items-center gap-2 text-sm text-gray-500 dark:text-gray-400'>
            <Users className='size-4' />
            <span>{t('team:members', { count: team.members.length })}</span>
          </div>
        )}
      </Link>
    </motion.div>
  )
}

export default TeamCard
