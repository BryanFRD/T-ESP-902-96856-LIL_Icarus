import { PlusIcon, UsersIcon } from 'lucide-react'
import { useTranslation } from 'react-i18next'

import { TeamCard } from '@/components/teams/index/team-card'
import { TeamFormDialog } from '@/components/teams/team-form-dialog'
import { Button } from '@/components/ui/button'
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle
} from '@/components/ui/empty'
import { Skeleton } from '@/components/ui/skeleton'
import { useMyTeam } from '@/hooks/team/use-my-team.ts'

const TeamScreen = () => {
  const { t } = useTranslation('team')

  const { data: teams, isLoading } = useMyTeam()

  return (
    <div className='space-y-8 px-4 py-8'>
      <div className='flex items-center justify-between'>
        <div className='space-y-1'>
          <h1 className='text-2xl font-semibold'>{t('index.title')}</h1>
          <p className='text-muted-foreground text-sm'>
            {t('index.description')}
          </p>
        </div>

        <TeamFormDialog>
          <Button>
            <PlusIcon className='size-4' />
            {t('index.createTeam')}
          </Button>
        </TeamFormDialog>
      </div>

      <div className='grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3'>
        {isLoading ? (
          Array.from({ length: 3 }).map((_, index) => (
            <Skeleton key={index} className='h-52 w-full rounded-2xl' />
          ))
        ) : teams && teams.length > 0 ? (
          teams.map(team => <TeamCard key={team.id} team={team} />)
        ) : (
          <Empty className='col-span-3 w-full'>
            <EmptyHeader>
              <EmptyMedia variant='icon'>
                <UsersIcon />
              </EmptyMedia>
              <EmptyTitle>{t('index.empty.title')}</EmptyTitle>
              <EmptyDescription>
                {t('index.empty.description')}
              </EmptyDescription>
            </EmptyHeader>

            <EmptyContent>
              <TeamFormDialog>
                <Button>
                  <PlusIcon className='size-4' />
                  {t('index.createTeam')}
                </Button>
              </TeamFormDialog>
            </EmptyContent>
          </Empty>
        )}
      </div>
    </div>
  )
}

export default TeamScreen
