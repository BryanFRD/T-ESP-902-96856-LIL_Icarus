import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'

import {
  deleteAttachment,
  downloadAttachment,
  getAttachmentPreview,
  getAttachments,
  uploadAttachment
} from '@/service/kanbanService'
import { getErrorMessage } from '@/utils/apiUtils'

export const useAttachments = (auditId: string, ticketId: string) => {
  return useQuery({
    queryKey: ['kanban', 'attachments', auditId, ticketId],
    queryFn: () => getAttachments(auditId, ticketId),
    enabled: !!auditId && !!ticketId
  })
}

export const useUploadAttachment = (auditId: string, ticketId: string) => {
  const queryClient = useQueryClient()
  const { t } = useTranslation()

  return useMutation({
    mutationFn: (file: File) => uploadAttachment(auditId, ticketId, file),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['kanban', 'attachments', auditId, ticketId]
      })
      queryClient.invalidateQueries({
        queryKey: ['kanban', 'ticket', auditId, ticketId]
      })
      queryClient.invalidateQueries({ queryKey: ['kanban', 'board', auditId] })
      toast.success(t('kanban:attachment.uploaded'))
    },
    onError: error => {
      const message = getErrorMessage(error, 'kanban:attachment.uploadError')
      toast.error(message)
    }
  })
}

export const useDownloadAttachment = (auditId: string, ticketId: string) => {
  return useMutation({
    mutationFn: (attachmentId: string) =>
      downloadAttachment(auditId, ticketId, attachmentId),
    onError: error => {
      const message = getErrorMessage(error, 'kanban:attachment.downloadError')
      toast.error(message)
    }
  })
}

export const useAttachmentPreview = (
  auditId: string,
  ticketId: string,
  attachmentId: string
) => {
  return useQuery({
    queryKey: [
      'kanban',
      'attachment',
      'preview',
      auditId,
      ticketId,
      attachmentId
    ],
    queryFn: async () => {
      const blob = await getAttachmentPreview(auditId, ticketId, attachmentId)
      return URL.createObjectURL(blob)
    },
    enabled: !!auditId && !!ticketId && !!attachmentId,
    staleTime: 1000 * 60 * 5
  })
}

export const useDeleteAttachment = (auditId: string, ticketId: string) => {
  const queryClient = useQueryClient()
  const { t } = useTranslation()

  return useMutation({
    mutationFn: (attachmentId: string) =>
      deleteAttachment(auditId, ticketId, attachmentId),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['kanban', 'attachments', auditId, ticketId]
      })
      queryClient.invalidateQueries({
        queryKey: ['kanban', 'ticket', auditId, ticketId]
      })
      queryClient.invalidateQueries({ queryKey: ['kanban', 'board', auditId] })
      toast.success(t('kanban:attachment.deleted'))
    },
    onError: error => {
      const message = getErrorMessage(error, 'kanban:attachment.deleteError')
      toast.error(message)
    }
  })
}
