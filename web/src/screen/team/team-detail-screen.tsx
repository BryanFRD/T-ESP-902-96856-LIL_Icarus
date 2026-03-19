import { Users } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Link, useParams } from 'react-router-dom'

import { TeamDetailCard } from '@/components/teams/show/team-detail-card'
import { TeamTabs } from '@/components/teams/show/team-tabs'
import { Button } from '@/components/ui/button'
import { Spinner } from '@/components/ui/spinner'
import { useMyTeam } from '@/hooks/team/use-my-team.ts'
import { ROUTES, getRoutePath } from '@/router/routes-name.ts'

const TeamDetailScreen = () => {
  const { t } = useTranslation(['team', 'common', 'messages'])
  const { teamId } = useParams<{ teamId: string }>()

  const { data: teams, isLoading: isLoadingTeams } = useMyTeam()

  const team = teams?.find(t => t.id === teamId)

  if (isLoadingTeams) {
    return (
      <div className='flex items-center justify-center py-24'>
        <Spinner className='size-8' />
      </div>
    )
  }

  if (!team) {
    return (
      <div className='mx-auto max-w-5xl px-4 py-8'>
        <div className='flex flex-col items-center justify-center py-16 text-center'>
          <div className='bg-muted mb-4 flex size-20 items-center justify-center rounded-full'>
            <Users className='text-muted-foreground size-10' />
          </div>
          <h3 className='mb-2 text-lg font-medium'>
            {t('teamNotFound.title')}
          </h3>
          <p className='text-muted-foreground mb-6 text-sm'>
            {t('teamNotFound.description')}
          </p>
          <Button asChild>
            <Link to={getRoutePath(ROUTES.TEAMS)}>
              {t('teamNotFound.backToTeams')}
            </Link>
          </Button>
        </div>
      </div>
    )
  }

  return (
    <div className='mx-auto max-w-5xl space-y-6 px-4 py-8'>
      <TeamDetailCard team={team} />

      <TeamTabs team={team} />
    </div>
  )
}

export default TeamDetailScreen
