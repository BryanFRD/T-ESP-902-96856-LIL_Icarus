import { motion } from 'framer-motion'
import { Clock, Copy, Link2, Trash2, Users } from 'lucide-react'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'
import { twMerge } from 'tailwind-merge'

import Button from '@/components/ui/button/Button'
import type { Invitation } from '@/domain/types/Team.ts'
import { useDateFormatter } from '@/utils/dateFormatter'

interface InvitationCardProps {
  invitation: Invitation
  index?: number
  canManage?: boolean
  onCancel?: (invitation: Invitation) => void
}

const InvitationCard = ({
  invitation,
  index = 0,
  canManage = false,
  onCancel
}: InvitationCardProps) => {
  const { t } = useTranslation()
  const { dateLong } = useDateFormatter()

  const isExpired = invitation.expiresAt
    ? new Date(invitation.expiresAt) < new Date()
    : false
  const isFullyUsed = invitation.usedCount >= invitation.maxUses
  const isDisabled = invitation.isCanceled || isExpired || isFullyUsed

  const copyInviteLink = () => {
    const link = `${window.location.origin}/teams/join/${invitation.token}`
    navigator.clipboard.writeText(link)
    toast.success(t('team.invitationLinkCopied'))
  }

  const getStatusText = () => {
    if (invitation.isCanceled)
      return t('team.invitationCanceledAt', {
        date: dateLong(invitation.updatedAt)
      })
    if (isFullyUsed) return t('team.invitationFullyUsed')
    if (isExpired) return t('team.invitationExpired')
    if (invitation.expiresAt) {
      return t('team.expiresAtDate', { date: dateLong(invitation.expiresAt) })
    }
    return t('team.noExpiration')
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: 10 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ delay: index * 0.05 }}
      className={twMerge(
        'rounded-lg border bg-white p-4 dark:bg-gray-800',
        isDisabled
          ? 'border-gray-200 bg-gray-50 dark:border-gray-700 dark:bg-gray-800/50'
          : 'border-gray-200 dark:border-gray-700'
      )}
    >
      <div className='flex items-center justify-between gap-4'>
        <div className='flex min-w-0 flex-1 items-center gap-3'>
          <div
            className={twMerge(
              'flex h-8 w-8 flex-shrink-0 items-center justify-center rounded-full',
              isDisabled
                ? 'bg-gray-100 dark:bg-gray-700'
                : 'bg-purple-100 dark:bg-purple-900'
            )}
          >
            <Link2
              className={twMerge(
                'size-4',
                isDisabled
                  ? 'text-gray-400 dark:text-gray-500'
                  : 'text-purple-600 dark:text-purple-400'
              )}
            />
          </div>

          <div className='flex min-w-0 flex-1 flex-wrap items-center gap-x-3 gap-y-1'>
            <span
              className={twMerge(
                'inline-flex items-center gap-1 text-sm font-medium',
                isDisabled
                  ? 'text-gray-400 dark:text-gray-500'
                  : 'text-gray-700 dark:text-gray-200'
              )}
            >
              <Users className='size-3.5' />
              {t('team.usageCount', {
                used: invitation.usedCount,
                max: invitation.maxUses
              })}
            </span>

            <span
              className={twMerge(
                'inline-flex items-center gap-1 text-sm',
                isDisabled
                  ? 'text-red-500 dark:text-red-400'
                  : 'text-gray-500 dark:text-gray-400'
              )}
            >
              <Clock className='size-3.5' />
              {getStatusText()}
            </span>
          </div>
        </div>

        <div className='flex flex-shrink-0 items-center gap-1'>
          {!isDisabled && (
            <Button
              variant='ghost'
              size='sm'
              onClick={copyInviteLink}
              aria-label={t('team.copyInvitationLink')}
            >
              <Copy className='size-4' />
            </Button>
          )}

          {canManage && !invitation.isCanceled && (
            <Button
              variant='ghost'
              size='sm'
              onClick={() => onCancel?.(invitation)}
              aria-label={t('team.cancelInvitation')}
              className='text-red-500 hover:text-red-600 dark:text-red-400'
            >
              <Trash2 className='size-4' />
            </Button>
          )}
        </div>
      </div>
    </motion.div>
  )
}

export default InvitationCard
