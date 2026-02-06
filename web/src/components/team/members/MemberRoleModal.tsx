import { Shield, User } from 'lucide-react'
import { useState } from 'react'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'
import { twMerge } from 'tailwind-merge'

import Button from '@/components/ui/button/Button'
import Modal from '@/components/ui/modal/Modal'
import { MemberRole } from '@/domain/enum/MemberRole.ts'
import type { TeamMember } from '@/domain/types/Team.ts'
import { useUpdateMemberRole } from '@/hooks/team/useUpdateMemberRole.ts'

interface MemberRoleModalProps {
  isOpen: boolean
  onClose: () => void
  teamId: string
  member: TeamMember | null
}

const roleOptions = [
  {
    value: MemberRole.MODERATOR,
    icon: Shield,
    color: 'text-blue-500'
  },
  {
    value: MemberRole.MEMBER,
    icon: User,
    color: 'text-gray-500'
  },
  {
    value: MemberRole.CLIENT,
    icon: User,
    color: 'text-green-500'
  }
]

const MemberRoleModal = ({
  isOpen,
  onClose,
  teamId,
  member
}: MemberRoleModalProps) => {
  const { t } = useTranslation()
  const { mutate: updateMemberRole, isPending } = useUpdateMemberRole()
  const [selectedRole, setSelectedRole] = useState<MemberRole | null>(null)

  const handleSave = () => {
    if (!member || !selectedRole) return

    toast.promise(
      new Promise<void>((resolve, reject) => {
        updateMemberRole(
          {
            teamId,
            memberId: member.id,
            payload: { role: selectedRole }
          },
          {
            onSuccess: () => {
              onClose()
              resolve()
            },
            onError: error => reject(error)
          }
        )
      }),
      {
        loading: t('messages:loading.savingChanges'),
        success: t('messages:success.memberRoleUpdated'),
        error: t('messages:error.memberRoleUpdateFailed')
      }
    )
  }

  if (!member) return null

  const displayName =
    member.firstName && member.lastName
      ? `${member.firstName} ${member.lastName}`
      : member.username || member.email || t('team:unknownMember')

  return (
    <Modal
      isOpen={isOpen}
      onClose={onClose}
      title={t('team:changeRoleTitle')}
      footer={
        <div className='flex justify-end gap-3'>
          <Button type='button' variant='secondary' onClick={onClose}>
            {t('common:actions.cancel')}
          </Button>
          <Button
            type='button'
            onClick={handleSave}
            isLoading={isPending}
            disabled={!selectedRole || selectedRole === member.teamRole}
          >
            {t('common:actions.save')}
          </Button>
        </div>
      }
    >
      <div className='space-y-4'>
        <p className='text-sm text-gray-600 dark:text-gray-400'>
          {t('team:changeRoleDescription', { name: displayName })}
        </p>

        <div className='space-y-2'>
          {roleOptions.map(option => {
            const Icon = option.icon
            const isSelected =
              selectedRole === option.value ||
              (!selectedRole && member.teamRole === option.value)

            return (
              <button
                key={option.value}
                type='button'
                onClick={() =>
                  setSelectedRole(option.value.toUpperCase() as MemberRole)
                }
                className={twMerge(
                  'flex w-full items-center gap-3 rounded-lg border-2 p-4 text-left transition-all',
                  isSelected
                    ? 'border-orange-500 bg-orange-50 dark:border-orange-400 dark:bg-orange-900/20'
                    : 'border-gray-200 hover:border-gray-300 dark:border-gray-700 dark:hover:border-gray-600'
                )}
              >
                <Icon className={twMerge('size-5', option.color)} />
                <div>
                  <p className='font-medium text-gray-900 dark:text-white'>
                    {t(`teams.roles.${option.value.toLowerCase()}`)}
                  </p>
                  <p className='text-sm text-gray-500 dark:text-gray-400'>
                    {t(`teams.roleDescriptions.${option.value.toLowerCase()}`)}
                  </p>
                </div>
              </button>
            )
          })}
        </div>
      </div>
    </Modal>
  )
}

export default MemberRoleModal
