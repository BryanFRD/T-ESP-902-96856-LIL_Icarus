import { useMutation, useQueryClient } from '@tanstack/react-query'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'

import type {
  CreateTicketPayload,
  MoveTicketPayload,
  UpdateTicketPayload
} from '@/domain/types/kanban.ts'
import {
  createTicket,
  deleteTicket,
  moveTicket,
  updateTicket
} from '@/service/kanban-service.ts'
import { getErrorMessage } from '@/utils/api-utils.ts'

export const useCreateTicket = (auditId: string) => {
  const queryClient = useQueryClient()
  const { t } = useTranslation()

  return useMutation({
    mutationFn: (payload: CreateTicketPayload) =>
      createTicket(auditId, payload),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['kanban', 'board', auditId] })
      queryClient.invalidateQueries({
        queryKey: ['kanban', 'tickets', auditId]
      })
      toast.success(t('kanban:ticket.created'))
    },
    onError: error => {
      const message = getErrorMessage(error, 'kanban:ticket.createError')
      toast.error(message)
    }
  })
}

export const useUpdateTicket = (auditId: string, ticketId: string) => {
  const queryClient = useQueryClient()
  const { t } = useTranslation()

  return useMutation({
    mutationFn: (payload: UpdateTicketPayload) =>
      updateTicket(auditId, ticketId, payload),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['kanban', 'board', auditId] })
      queryClient.invalidateQueries({
        queryKey: ['kanban', 'tickets', auditId]
      })
      queryClient.invalidateQueries({
        queryKey: ['kanban', 'ticket', auditId, ticketId]
      })
      toast.success(t('kanban:ticket.updated'))
    },
    onError: error => {
      const message = getErrorMessage(error, 'kanban:ticket.updateError')
      toast.error(message)
    }
  })
}

export const useDeleteTicket = (auditId: string) => {
  const queryClient = useQueryClient()
  const { t } = useTranslation()

  return useMutation({
    mutationFn: (ticketId: string) => deleteTicket(auditId, ticketId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['kanban', 'board', auditId] })
      queryClient.invalidateQueries({
        queryKey: ['kanban', 'tickets', auditId]
      })
      toast.success(t('kanban:ticket.deleted'))
    },
    onError: error => {
      const message = getErrorMessage(error, 'kanban:ticket.deleteError')
      toast.error(message)
    }
  })
}

export const useMoveTicket = (auditId: string) => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({
      ticketId,
      payload
    }: {
      ticketId: string
      payload: MoveTicketPayload
    }) => moveTicket(auditId, ticketId, payload),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['kanban', 'board', auditId] })
    }
  })
}
