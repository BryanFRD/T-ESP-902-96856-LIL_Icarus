import { useMutation, useQueryClient } from '@tanstack/react-query'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'

import type {
  CreateLabelPayload,
  UpdateLabelPayload
} from '@/domain/types/Kanban'
import {
  addLabelToTicket,
  createLabel,
  deleteLabel,
  removeLabelFromTicket,
  updateLabel
} from '@/service/kanbanService'
import { getErrorMessage } from '@/utils/apiUtils'

export const useCreateLabel = (auditId: string) => {
  const queryClient = useQueryClient()
  const { t } = useTranslation()

  return useMutation({
    mutationFn: (payload: CreateLabelPayload) => createLabel(auditId, payload),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['kanban', 'board', auditId] })
      queryClient.invalidateQueries({ queryKey: ['kanban', 'labels', auditId] })
      toast.success(t('kanban:label.created'))
    },
    onError: error => {
      const message = getErrorMessage(error, 'kanban:label.createError')
      toast.error(message)
    }
  })
}

export const useUpdateLabel = (auditId: string, labelId: string) => {
  const queryClient = useQueryClient()
  const { t } = useTranslation()

  return useMutation({
    mutationFn: (payload: UpdateLabelPayload) =>
      updateLabel(auditId, labelId, payload),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['kanban', 'board', auditId] })
      queryClient.invalidateQueries({ queryKey: ['kanban', 'labels', auditId] })
      toast.success(t('kanban:label.updated'))
    },
    onError: error => {
      const message = getErrorMessage(error, 'kanban:label.updateError')
      toast.error(message)
    }
  })
}

export const useDeleteLabel = (auditId: string) => {
  const queryClient = useQueryClient()
  const { t } = useTranslation()

  return useMutation({
    mutationFn: (labelId: string) => deleteLabel(auditId, labelId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['kanban', 'board', auditId] })
      queryClient.invalidateQueries({ queryKey: ['kanban', 'labels', auditId] })
      toast.success(t('kanban:label.deleted'))
    },
    onError: error => {
      const message = getErrorMessage(error, 'kanban:label.deleteError')
      toast.error(message)
    }
  })
}

export const useAddLabelToTicket = (auditId: string, ticketId: string) => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (labelId: string) =>
      addLabelToTicket(auditId, ticketId, labelId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['kanban', 'board', auditId] })
      queryClient.invalidateQueries({
        queryKey: ['kanban', 'ticket', auditId, ticketId]
      })
    }
  })
}

export const useRemoveLabelFromTicket = (auditId: string, ticketId: string) => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (labelId: string) =>
      removeLabelFromTicket(auditId, ticketId, labelId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['kanban', 'board', auditId] })
      queryClient.invalidateQueries({
        queryKey: ['kanban', 'ticket', auditId, ticketId]
      })
    }
  })
}
