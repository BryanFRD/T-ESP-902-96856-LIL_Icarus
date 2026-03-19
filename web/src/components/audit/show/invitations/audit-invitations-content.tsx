import { MailIcon, PlusIcon } from 'lucide-react'
import { useTranslation } from 'react-i18next'

import { Button } from '@/components/ui/button'
import {
  Empty,
  EmptyContent,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle
} from '@/components/ui/empty'

export function AuditInvitationsContent() {
  const { t } = useTranslation('audit')

  return (
    <div className='space-y-4'>
      <div className='flex justify-end'>
        <Button size='sm' onClick={() => {}}>
          <PlusIcon className='size-4' />
          {t('show.invitations.create')}
        </Button>
      </div>

      <Empty>
        <EmptyHeader>
          <EmptyMedia variant='icon'>
            <MailIcon />
          </EmptyMedia>
          <EmptyTitle>{t('show.invitations.empty')}</EmptyTitle>
        </EmptyHeader>
        <EmptyContent />
      </Empty>
    </div>
  )
}
