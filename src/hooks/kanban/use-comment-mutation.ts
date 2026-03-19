import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'

import type {
  CreateCommentPayload,
  UpdateCommentPayload
} from '@/domain/types/kanban.ts'
import {
  createComment,
  deleteComment,
  getComments,
  updateComment
} from '@/service/kanban-service.ts'
import { getErrorMessage } from '@/utils/api-utils.ts'

export const useComments = (auditId: string, ticketId: string) => {
  return useQuery({
    queryKey: ['kanban', 'comments', auditId, ticketId],
    queryFn: () => getComments(auditId, ticketId),
    enabled: !!auditId && !!ticketId
  })
}

export const useCreateComment = (auditId: string, ticketId: string) => {
  const queryClient = useQueryClient()
  const { t } = useTranslation()

  return useMutation({
    mutationFn: (payload: CreateCommentPayload) =>
      createComment(auditId, ticketId, payload),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['kanban', 'comments', auditId, ticketId]
      })
      queryClient.invalidateQueries({
        queryKey: ['kanban', 'ticket', auditId, ticketId]
      })
      queryClient.invalidateQueries({ queryKey: ['kanban', 'board', auditId] })
      toast.success(t('kanban:comment.created'))
    },
    onError: error => {
      const message = getErrorMessage(error, 'kanban:comment.createError')
      toast.error(message)
    }
  })
}

export const useUpdateComment = (
  auditId: string,
  ticketId: string,
  commentId: string
) => {
  const queryClient = useQueryClient()
  const { t } = useTranslation()

  return useMutation({
    mutationFn: (payload: UpdateCommentPayload) =>
      updateComment(auditId, ticketId, commentId, payload),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['kanban', 'comments', auditId, ticketId]
      })
      toast.success(t('kanban:comment.updated'))
    },
    onError: error => {
      const message = getErrorMessage(error, 'kanban:comment.updateError')
      toast.error(message)
    }
  })
}

export const useDeleteComment = (auditId: string, ticketId: string) => {
  const queryClient = useQueryClient()
  const { t } = useTranslation()

  return useMutation({
    mutationFn: (commentId: string) =>
      deleteComment(auditId, ticketId, commentId),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['kanban', 'comments', auditId, ticketId]
      })
      queryClient.invalidateQueries({
        queryKey: ['kanban', 'ticket', auditId, ticketId]
      })
      queryClient.invalidateQueries({ queryKey: ['kanban', 'board', auditId] })
      toast.success(t('kanban:comment.deleted'))
    },
    onError: error => {
      const message = getErrorMessage(error, 'kanban:comment.deleteError')
      toast.error(message)
    }
  })
}
