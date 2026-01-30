import { Check, Copy, Link } from 'lucide-react'
import { useState } from 'react'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'

import Button from '@/components/ui/button/Button'
import type { Invitation } from '@/domain/types/Team.ts'

interface InvitationLinkCopyProps {
  invitation: Invitation
}

const InvitationLinkCopy = ({ invitation }: InvitationLinkCopyProps) => {
  const { t } = useTranslation()
  const [copied, setCopied] = useState(false)

  const inviteLink = `${window.location.origin}/teams/join/${invitation.token}`

  const copyToClipboard = () => {
    navigator.clipboard.writeText(inviteLink)
    setCopied(true)
    toast.success(t('team.invitationLinkCopied'))

    setTimeout(() => {
      setCopied(false)
    }, 2000)
  }

  return (
    <div className='space-y-2 rounded-lg border border-gray-200 bg-gray-50 p-4 dark:border-gray-700 dark:bg-gray-800'>
      <div className='flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-300'>
        <Link className='size-4' />
        {t('team.invitationLink')}
      </div>

      <div className='flex items-center gap-2'>
        <input
          type='text'
          readOnly
          value={inviteLink}
          className='flex-1 rounded-lg border border-gray-200 bg-white px-3 py-2 text-sm text-gray-700 dark:border-gray-600 dark:bg-gray-900 dark:text-gray-300'
        />
        <Button
          type='button'
          variant='secondary'
          size='sm'
          onClick={copyToClipboard}
        >
          {copied ? (
            <Check className='size-4 text-green-500' />
          ) : (
            <Copy className='size-4' />
          )}
        </Button>
      </div>

      <p className='text-xs text-gray-500 dark:text-gray-400'>
        {t('team.invitationLinkHint')}
      </p>
    </div>
  )
}

export default InvitationLinkCopy
