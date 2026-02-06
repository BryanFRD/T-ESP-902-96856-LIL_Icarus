import { Users } from 'lucide-react'
import { useTranslation } from 'react-i18next'

import type { TeamMember } from '@/domain/types/Team.ts'

import MemberCard from './MemberCard.tsx'

interface MembersListProps {
  members: TeamMember[]
  canManage?: boolean
  onChangeRole?: (member: TeamMember) => void
  onRemove?: (member: TeamMember) => void
}

const MembersList = ({
  members,
  canManage = false,
  onChangeRole,
  onRemove
}: MembersListProps) => {
  const { t } = useTranslation()

  if (members.length === 0) {
    return (
      <div className='flex flex-col items-center justify-center py-12 text-center'>
        <div className='mb-4 flex size-16 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-800'>
          <Users className='size-8 text-gray-400 dark:text-gray-500' />
        </div>
        <h3 className='mb-2 text-lg font-medium text-gray-900 dark:text-white'>
          {t('team:noMembers')}
        </h3>
        <p className='text-sm text-gray-500 dark:text-gray-400'>
          {t('team:noMembersDescription')}
        </p>
      </div>
    )
  }

  return (
    <div className='space-y-3'>
      {members.map((member, index) => (
        <MemberCard
          key={member.id}
          member={member}
          index={index}
          canManage={canManage}
          onChangeRole={onChangeRole}
          onRemove={onRemove}
        />
      ))}
    </div>
  )
}

export default MembersList
