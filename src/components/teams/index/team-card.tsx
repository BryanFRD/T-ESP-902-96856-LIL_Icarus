import { ChevronRight, Users } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Link } from 'react-router-dom'

import {
  Card,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle
} from '@/components/ui/card'
import type { Team } from '@/domain/types/team.ts'

interface TeamCardProps {
  team: Team
}

export function TeamCard({ team }: TeamCardProps) {
  const { t } = useTranslation('team')

  return (
    <Link to={`/teams/${team.id}`} className='group'>
      <Card className='transition-shadow hover:shadow-md'>
        <CardHeader>
          <div className='flex items-start justify-between'>
            <div className='bg-primary/10 text-primary flex size-10 items-center justify-center rounded-xl'>
              <Users className='size-5' />
            </div>
            <ChevronRight className='text-muted-foreground size-4 opacity-0 transition-opacity group-hover:opacity-100' />
          </div>
          <CardTitle>{team.name}</CardTitle>
          {team.description && (
            <CardDescription>{team.description}</CardDescription>
          )}
        </CardHeader>

        {team.members && (
          <CardFooter className='text-muted-foreground border-t text-xs'>
            <div className='flex items-center gap-1.5'>
              <Users className='size-3.5' />
              <span>
                {t('index.teamCard.members', { count: team.members.length })}
              </span>
            </div>
          </CardFooter>
        )}
      </Card>
    </Link>
  )
}
