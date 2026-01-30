import { Mail } from 'lucide-react'
import { useTranslation } from 'react-i18next'

import type { Invitation } from '@/domain/types/Team.ts'

import InvitationCard from './InvitationCard.tsx'

interface InvitationsListProps {
  invitations: Invitation[]
  canManage?: boolean
  onCancel?: (invitation: Invitation) => void
}

const InvitationsList = ({
  invitations,
  canManage = false,
  onCancel
}: InvitationsListProps) => {
  const { t } = useTranslation()

  if (invitations.length === 0) {
    return (
      <div className='flex flex-col items-center justify-center py-12 text-center'>
        <div className='mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-800'>
          <Mail className='h-8 w-8 text-gray-400 dark:text-gray-500' />
        </div>
        <h3 className='mb-2 text-lg font-medium text-gray-900 dark:text-white'>
          {t('team.noInvitations')}
        </h3>
        <p className='text-sm text-gray-500 dark:text-gray-400'>
          {t('team.noInvitationsDescription')}
        </p>
      </div>
    )
  }

  return (
    <div className='space-y-3'>
      {invitations.map((invitation, index) => (
        <InvitationCard
          key={invitation.id}
          invitation={invitation}
          index={index}
          canManage={canManage}
          onCancel={onCancel}
        />
      ))}
    </div>
  )
}

export default InvitationsList
