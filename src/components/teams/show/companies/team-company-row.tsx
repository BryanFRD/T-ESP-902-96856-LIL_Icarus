import {
  Building2Icon,
  DotIcon,
  EllipsisIcon,
  PencilIcon,
  Trash2Icon
} from 'lucide-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger
} from '@/components/ui/dropdown-menu'
import type { Company } from '@/domain/types/team.ts'

import { CompanyFormDialog } from './company-form-dialog'
import { DeleteCompanyDialog } from './delete-company-dialog'

interface Props {
  company: Company
  canManage: boolean
  teamId: string
}

export function CompanyRow({ company, canManage, teamId }: Props) {
  const { t } = useTranslation('team')

  const [editOpen, setEditOpen] = useState(false)
  const [deleteOpen, setDeleteOpen] = useState(false)

  return (
    <Card className='flex-row items-center gap-x-2 px-4'>
      <div className='bg-muted flex size-9 items-center justify-center rounded-full'>
        <Building2Icon className='text-muted-foreground size-4' />
      </div>

      <div className='flex flex-1 flex-col gap-y-0.5'>
        <div className='flex items-center gap-x-1 text-sm'>
          <p className='font-medium'>{company.name}</p>
          <DotIcon className='text-muted-foreground size-3' />
          <p className='text-muted-foreground'>{company.siret}</p>
        </div>

        {company.description && (
          <p className='text-muted-foreground truncate text-xs'>
            {company.description}
          </p>
        )}
      </div>

      {canManage && (
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant='ghost' size='icon'>
              <EllipsisIcon className='size-4' />
            </Button>
          </DropdownMenuTrigger>

          <DropdownMenuContent align='end'>
            <DropdownMenuItem onSelect={() => setEditOpen(true)}>
              <PencilIcon className='size-4' />
              <span>{t('show.companies.actions.edit')}</span>
            </DropdownMenuItem>

            <DropdownMenuItem
              variant='destructive'
              onSelect={() => setDeleteOpen(true)}
            >
              <Trash2Icon className='size-4' />
              <span>{t('show.companies.actions.delete')}</span>
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      )}

      <CompanyFormDialog
        open={editOpen}
        onOpenChange={setEditOpen}
        teamId={teamId}
        company={company}
      />

      <DeleteCompanyDialog
        open={deleteOpen}
        onOpenChange={setDeleteOpen}
        teamId={teamId}
        company={company}
      />
    </Card>
  )
}
