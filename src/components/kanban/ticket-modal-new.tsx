import { Trash2, X } from 'lucide-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import Button from '@/components/oldUi/button/Button'
import Modal from '@/components/oldUi/modal/Modal'
import ModalConfirm from '@/components/oldUi/modal/ModalConfirm'
import { useUserContext } from '@/context/user-context.tsx'
import { canDeleteTicket } from '@/domain/permissions/kanban-permissions.ts'
import type { LabelResponse, TicketUser } from '@/domain/types/kanban.ts'
import { useAuditRole } from '@/hooks/kanban/use-audit-role.ts'
import { useDeleteTicket } from '@/hooks/kanban/use-ticket-mutation.ts'
import { useTicket } from '@/hooks/kanban/use-ticket.ts'

import AttachmentSection from './attachment-section.tsx'
import CommentSection from './comment-section.tsx'
import TicketActivity from './ticket-activity.tsx'
import TicketDetails from './ticket-details.tsx'

type TicketModalProps = {
  isOpen: boolean
  onClose: () => void
  auditId: string
  ticketId: string
  teamId: string
  availableUsers?: TicketUser[]
  availableLabels?: LabelResponse[]
}

const TicketModal = ({
  isOpen,
  onClose,
  auditId,
  ticketId,
  teamId,
  availableUsers = [],
  availableLabels = []
}: TicketModalProps) => {
  const { t } = useTranslation()
  const { user } = useUserContext()
  const { role } = useAuditRole(teamId)
  const { data: ticket, isLoading } = useTicket(auditId, ticketId)
  const { mutateAsync: deleteTicket, isPending: isDeleting } =
    useDeleteTicket(auditId)
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false)
  const [activeTab, setActiveTab] = useState<'details' | 'activity'>('details')

  const handleDelete = async () => {
    try {
      await deleteTicket(ticketId)
      setShowDeleteConfirm(false)
      onClose()
    } catch {
      // Error handled by mutation
    }
  }

  const canDelete = role && user && ticket ? canDeleteTicket(role) : false

  if (isLoading || !ticket) {
    return (
      <Modal isOpen={isOpen} onClose={onClose} title={t('kanban:loading')}>
        <div className='flex items-center justify-center py-12'>
          <div className='h-8 w-8 animate-spin rounded-full border-4 border-gray-200 border-t-orange-500' />
        </div>
      </Modal>
    )
  }

  return (
    <>
      <Modal isOpen={isOpen} onClose={onClose} size='xl' title={ticket.title}>
        <div className='flex h-[85vh] flex-col'>
          <div className='flex items-center justify-between border-b border-gray-200 px-6 py-4 dark:border-gray-700'>
            <div className='flex items-center gap-3'>
              <button
                type='button'
                onClick={() => setActiveTab('details')}
                className={`px-4 py-2 text-sm font-medium transition-colors ${
                  activeTab === 'details'
                    ? 'border-b-2 border-orange-500 text-gray-900 dark:text-gray-100'
                    : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200'
                }`}
              >
                {t('kanban:ticket.details')}
              </button>
              <button
                type='button'
                onClick={() => setActiveTab('activity')}
                className={`px-4 py-2 text-sm font-medium transition-colors ${
                  activeTab === 'activity'
                    ? 'border-b-2 border-orange-500 text-gray-900 dark:text-gray-100'
                    : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200'
                }`}
              >
                {t('kanban:ticket.activity')}
              </button>
            </div>
            <div className='flex items-center gap-2'>
              {canDelete && (
                <Button
                  variant='ghost'
                  size='sm'
                  onClick={() => setShowDeleteConfirm(true)}
                >
                  <Trash2 className='h-4 w-4 text-red-500' />
                </Button>
              )}
              <button
                type='button'
                onClick={onClose}
                className='rounded-md p-1 text-gray-400 hover:bg-gray-100 hover:text-gray-600 dark:hover:bg-gray-800 dark:hover:text-gray-300'
              >
                <X className='h-5 w-5' />
              </button>
            </div>
          </div>

          <div className='flex flex-1 overflow-hidden'>
            <div className='flex-1 overflow-y-auto px-6 py-6'>
              {activeTab === 'details' ? (
                <div className='space-y-6'>
                  <TicketDetails
                    ticket={ticket}
                    auditId={auditId}
                    teamId={teamId}
                    availableUsers={availableUsers}
                    availableLabels={availableLabels}
                  />

                  <div className='space-y-6'>
                    <AttachmentSection auditId={auditId} ticketId={ticketId} />
                    <CommentSection auditId={auditId} ticketId={ticketId} />
                  </div>
                </div>
              ) : (
                <TicketActivity ticket={ticket} />
              )}
            </div>
          </div>
        </div>
      </Modal>

      <ModalConfirm
        isOpen={showDeleteConfirm}
        onClose={() => setShowDeleteConfirm(false)}
        onConfirm={handleDelete}
        title={t('kanban:ticket.deleteTitle')}
        confirmText={t('common:delete')}
        isLoading={isDeleting}
        variant='danger'
        description={t('kanban:ticket.deleteDescription')}
        confirmMatch={t('kanban:ticket.deleteConfirmMatch')}
      />
    </>
  )
}

export default TicketModal
