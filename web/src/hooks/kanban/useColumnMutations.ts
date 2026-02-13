import { useMutation, useQueryClient } from '@tanstack/react-query'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'

import type {
  CreateColumnPayload,
  ReorderColumnsPayload,
  UpdateColumnPayload
} from '@/domain/types/Kanban'
import {
  createColumn,
  deleteColumn,
  reorderColumns,
  updateColumn
} from '@/service/kanbanService'
import { getErrorMessage } from '@/utils/apiUtils'

export const useCreateColumn = (auditId: string) => {
  const queryClient = useQueryClient()
  const { t } = useTranslation()

  return useMutation({
    mutationFn: (payload: CreateColumnPayload) =>
      createColumn(auditId, payload),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['kanban', 'board', auditId] })
      queryClient.invalidateQueries({
        queryKey: ['kanban', 'columns', auditId]
      })
      toast.success(t('kanban:column.created'))
    },
    onError: error => {
      const message = getErrorMessage(error, 'kanban:column.createError')
      toast.error(message)
    }
  })
}

export const useUpdateColumn = (auditId: string, columnId: string) => {
  const queryClient = useQueryClient()
  const { t } = useTranslation()

  return useMutation({
    mutationFn: (payload: UpdateColumnPayload) =>
      updateColumn(auditId, columnId, payload),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['kanban', 'board', auditId] })
      queryClient.invalidateQueries({
        queryKey: ['kanban', 'columns', auditId]
      })
      toast.success(t('kanban:column.updated'))
    },
    onError: error => {
      const message = getErrorMessage(error, 'kanban:column.updateError')
      toast.error(message)
    }
  })
}

export const useDeleteColumn = (auditId: string) => {
  const queryClient = useQueryClient()
  const { t } = useTranslation()

  return useMutation({
    mutationFn: (columnId: string) => deleteColumn(auditId, columnId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['kanban', 'board', auditId] })
      queryClient.invalidateQueries({
        queryKey: ['kanban', 'columns', auditId]
      })
      toast.success(t('kanban:column.deleted'))
    },
    onError: error => {
      const message = getErrorMessage(error, 'kanban:column.deleteError')
      toast.error(message)
    }
  })
}

export const useReorderColumns = (auditId: string) => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (payload: ReorderColumnsPayload) =>
      reorderColumns(auditId, payload),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['kanban', 'board', auditId] })
      queryClient.invalidateQueries({
        queryKey: ['kanban', 'columns', auditId]
      })
    },
    onError: error => {
      const message = getErrorMessage(error, 'kanban:column.reorderError')
      toast.error(message)
    }
  })
}
