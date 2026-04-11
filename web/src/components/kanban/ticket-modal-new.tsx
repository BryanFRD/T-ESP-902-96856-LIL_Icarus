import { Loader2, Trash2 } from 'lucide-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle
} from '@/components/ui/alert-dialog'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle
} from '@/components/ui/dialog'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
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

  return (
    <>
      <Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
        <DialogContent className='max-h-[90vh] overflow-y-auto sm:max-w-3xl'>
          {isLoading || !ticket ? (
            <div className='flex items-center justify-center py-12'>
              <Loader2 className='text-primary h-8 w-8 animate-spin' />
            </div>
          ) : (
            <>
              <DialogHeader>
                <DialogTitle className='text-xl'>{ticket.title}</DialogTitle>
              </DialogHeader>

              <Tabs defaultValue='details' className='w-full'>
                <div className='flex items-center justify-between'>
                  <TabsList>
                    <TabsTrigger value='details'>
                      {t('kanban:ticket.details')}
                    </TabsTrigger>
                    <TabsTrigger value='activity'>
                      {t('kanban:ticket.activity')}
                    </TabsTrigger>
                  </TabsList>
                  {canDelete && (
                    <Button
                      variant='ghost'
                      size='sm'
                      onClick={() => setShowDeleteConfirm(true)}
                      className='text-destructive hover:text-destructive'
                    >
                      <Trash2 className='mr-1 h-4 w-4' />
                      {t('common:actions.delete')}
                    </Button>
                  )}
                </div>

                <TabsContent value='details' className='space-y-6'>
                  <TicketDetails
                    ticket={ticket}
                    auditId={auditId}
                    teamId={teamId}
                    availableUsers={availableUsers}
                    availableLabels={availableLabels}
                  />
                  <AttachmentSection auditId={auditId} ticketId={ticketId} />
                  <CommentSection auditId={auditId} ticketId={ticketId} />
                </TabsContent>

                <TabsContent value='activity'>
                  <TicketActivity ticket={ticket} />
                </TabsContent>
              </Tabs>
            </>
          )}
        </DialogContent>
      </Dialog>

      <AlertDialog open={showDeleteConfirm} onOpenChange={setShowDeleteConfirm}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>
              {t('kanban:ticket.deleteTitle')}
            </AlertDialogTitle>
            <AlertDialogDescription>
              {t('kanban:ticket.deleteDescription')}
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel disabled={isDeleting}>
              {t('common:actions.cancel')}
            </AlertDialogCancel>
            <AlertDialogAction
              onClick={handleDelete}
              disabled={isDeleting}
              className='bg-destructive text-destructive-foreground hover:bg-destructive/90'
            >
              {isDeleting && <Loader2 className='mr-2 h-4 w-4 animate-spin' />}
              {t('common:actions.delete')}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </>
  )
}

export default TicketModal
