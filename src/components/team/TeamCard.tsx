import { ChevronRight, Users } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Link } from 'react-router-dom'

import {
  Card,
  CardContent,
  CardFooter,
  CardHeader
} from '@/components/ui/card/Card.tsx'
import type { Team } from '@/domain/types/Team.ts'

interface TeamCardProps {
  team: Team
  index?: number
}

const TeamCard = ({ team, index = 0 }: TeamCardProps) => {
  const { t } = useTranslation()

  return (
    <Link to={`/teams/${team.id}`} className='group block'>
      <Card
        hover
        animate
        animationDelay={index * 0.1}
        className='flex h-full flex-col justify-between overflow-hidden'
      >
        <CardHeader>
          <div className='mb-3 flex items-start justify-between'>
            <div className='flex h-10 w-10 items-center justify-center rounded-lg bg-orange-50 text-orange-600 dark:bg-orange-900/20 dark:text-orange-400'>
              <Users size={20} />
            </div>
            <span className='inline-flex items-center rounded-full bg-orange-100 px-2.5 py-0.5 text-xs font-medium text-orange-800 dark:bg-orange-900/30 dark:text-orange-300'>
              Team
            </span>
          </div>
          <h3 className='mb-1 truncate text-lg font-bold text-gray-900 dark:text-gray-100'>
            {team.name}
          </h3>
        </CardHeader>

        <CardContent className='pb-4'>
          {team.description && (
            <p className='line-clamp-2 text-sm text-gray-600 dark:text-gray-400'>
              {team.description}
            </p>
          )}
        </CardContent>

        {team.members && (
          <CardFooter className='mt-auto border-t border-gray-100 pt-4 dark:border-gray-800'>
            <div className='flex w-full items-center justify-between'>
              <div className='flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400'>
                <Users className='size-4' />
                <span>{t('team:members', { count: team.members.length })}</span>
              </div>
              <div className='flex items-center text-orange-500 opacity-0 transition-opacity duration-300 group-hover:opacity-100'>
                <ChevronRight size={18} />
              </div>
            </div>
          </CardFooter>
        )}
      </Card>
    </Link>
  )
}

export default TeamCard
