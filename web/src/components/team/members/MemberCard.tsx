import { motion } from 'framer-motion'
import { Crown, MoreVertical, Shield, User, UserMinus } from 'lucide-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import Button from '@/components/ui/button/Button'
import { MemberRole } from '@/domain/enum/MemberRole.ts'
import type { TeamMember } from '@/domain/types/Team.ts'
import useInteractionOutside from '@/hooks/common/useInteractionOutside.ts'
import { useGetAvatar } from '@/hooks/user/useGetAvatar.ts'

interface MemberCardProps {
  member: TeamMember
  index?: number
  canManage?: boolean
  onChangeRole?: (member: TeamMember) => void
  onRemove?: (member: TeamMember) => void
}

const roleIcons = {
  [MemberRole.OWNER]: Crown,
  [MemberRole.MODERATOR]: Shield,
  [MemberRole.MEMBER]: User,
  [MemberRole.CLIENT]: User
}

const roleColors = {
  [MemberRole.OWNER]:
    'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200',
  [MemberRole.MODERATOR]:
    'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200',
  [MemberRole.MEMBER]:
    'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200',
  [MemberRole.CLIENT]:
    'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200'
}

const MemberCard = ({
  member,
  index = 0,
  canManage = false,
  onChangeRole,
  onRemove
}: MemberCardProps) => {
  const { t } = useTranslation()
  const [showMenu, setShowMenu] = useState(false)
  const { data: avatarUrl } = useGetAvatar(member.avatarUrl)

  const { ref } = useInteractionOutside<HTMLDivElement>(
    () => setShowMenu(false),
    { enabled: showMenu, eventType: 'click' }
  )

  const RoleIcon = roleIcons[member.teamRole] || User
  const roleColor = roleColors[member.teamRole] || roleColors[MemberRole.MEMBER]

  const displayName =
    member.firstName && member.lastName
      ? `${member.firstName} ${member.lastName}`
      : member.username || member.email || t('team.unknownMember')

  return (
    <motion.div
      initial={{ opacity: 0, y: 10 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ delay: index * 0.05 }}
      className='flex items-center justify-between rounded-lg border border-gray-200 bg-white p-4 dark:border-gray-700 dark:bg-gray-800'
    >
      <div className='flex items-center gap-4'>
        <div className='flex h-10 w-10 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-700'>
          {avatarUrl ? (
            <img
              src={avatarUrl}
              alt={displayName}
              className='h-10 w-10 rounded-full object-cover'
            />
          ) : (
            <User className='h-5 w-5 text-gray-500 dark:text-gray-400' />
          )}
        </div>
        <div>
          <p className='font-medium text-gray-900 dark:text-white'>
            {displayName}
          </p>
          {member.email && (
            <p className='text-sm text-gray-500 dark:text-gray-400'>
              {member.email}
            </p>
          )}
        </div>
      </div>

      <div className='flex items-center gap-3'>
        <span
          className={`inline-flex items-center gap-1 rounded-full px-2.5 py-0.5 text-xs font-medium ${roleColor}`}
        >
          <RoleIcon className='h-3 w-3' />
          {t(`memberRoles.${member.teamRole.toLowerCase()}`)}
        </span>

        {canManage && member.teamRole !== MemberRole.OWNER && (
          <div className='relative' ref={ref}>
            <Button
              variant='ghost'
              size='sm'
              onClick={() => setShowMenu(!showMenu)}
              aria-label={t('actions.more')}
            >
              <MoreVertical className='h-4 w-4' />
            </Button>

            {showMenu && (
              <div className='absolute right-0 z-10 mt-2 w-48 rounded-lg border border-gray-200 bg-white py-1 shadow-lg dark:border-gray-700 dark:bg-gray-800'>
                <button
                  type='button'
                  onClick={() => {
                    setShowMenu(false)
                    onChangeRole?.(member)
                  }}
                  className='flex w-full items-center gap-2 px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100 dark:text-gray-200 dark:hover:bg-gray-700'
                >
                  <Shield className='h-4 w-4' />
                  {t('team.changeRole')}
                </button>
                <button
                  type='button'
                  onClick={() => {
                    setShowMenu(false)
                    onRemove?.(member)
                  }}
                  className='flex w-full items-center gap-2 px-4 py-2 text-left text-sm text-red-600 hover:bg-gray-100 dark:text-red-400 dark:hover:bg-gray-700'
                >
                  <UserMinus className='h-4 w-4' />
                  {t('team.removeMember')}
                </button>
              </div>
            )}
          </div>
        )}
      </div>
    </motion.div>
  )
}

export default MemberCard
