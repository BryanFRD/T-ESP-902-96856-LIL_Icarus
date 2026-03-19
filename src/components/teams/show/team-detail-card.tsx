import { UsersIcon } from 'lucide-react'

import { Card } from '@/components/ui/card'
import type { Team } from '@/domain/types/team.ts'

interface Props {
  team: Team
}

export function TeamDetailCard({ team }: Props) {
  return (
    <Card className='p-6'>
      <div className='flex items-center gap-4'>
        <div className='bg-primary/10 text-primary flex size-12 items-center justify-center rounded-xl'>
          <UsersIcon className='size-6' />
        </div>

        <div className='space-y-0.5'>
          <h1 className='text-lg font-semibold'>{team.name}</h1>

          {team.description && (
            <p className='text-muted-foreground'>{team.description}</p>
          )}
        </div>
      </div>
    </Card>
  )
}
