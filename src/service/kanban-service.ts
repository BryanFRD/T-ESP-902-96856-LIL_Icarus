import { getAxiosClient } from '@/configuration/api/axios-client.ts'
import type {
  AttachmentResponse,
  BoardResponse,
  ColumnResponse,
  CommentResponse,
  CreateColumnPayload,
  CreateCommentPayload,
  CreateLabelPayload,
  CreateTicketPayload,
  LabelResponse,
  MoveTicketPayload,
  ReorderColumnsPayload,
  TicketResponse,
  TicketSummary,
  UpdateColumnPayload,
  UpdateCommentPayload,
  UpdateLabelPayload,
  UpdateTicketPayload
} from '@/domain/types/kanban.ts'

export const getKanbanBoard = async (
  auditId: string
): Promise<BoardResponse> => {
  return getAxiosClient()
    .get<BoardResponse>(`/audits/${auditId}/board`)
    .then(res => res.data)
}

export const getTickets = async (auditId: string): Promise<TicketSummary[]> => {
  return getAxiosClient()
    .get<TicketSummary[]>(`/audits/${auditId}/tickets`)
    .then(res => res.data)
}

export const getTicket = async (
  auditId: string,
  ticketId: string
): Promise<TicketResponse> => {
  return getAxiosClient()
    .get<TicketResponse>(`/audits/${auditId}/tickets/${ticketId}`)
    .then(res => res.data)
}

export const createTicket = async (
  auditId: string,
  payload: CreateTicketPayload
): Promise<TicketResponse> => {
  return getAxiosClient()
    .post<TicketResponse>(`/audits/${auditId}/tickets`, payload)
    .then(res => res.data)
}

export const updateTicket = async (
  auditId: string,
  ticketId: string,
  payload: UpdateTicketPayload
): Promise<TicketResponse> => {
  return getAxiosClient()
    .patch<TicketResponse>(`/audits/${auditId}/tickets/${ticketId}`, payload)
    .then(res => res.data)
}

export const deleteTicket = async (
  auditId: string,
  ticketId: string
): Promise<void> => {
  return getAxiosClient()
    .delete(`/audits/${auditId}/tickets/${ticketId}`)
    .then(res => res.data)
}

export const moveTicket = async (
  auditId: string,
  ticketId: string,
  payload: MoveTicketPayload
): Promise<TicketResponse> => {
  return getAxiosClient()
    .put<TicketResponse>(`/audits/${auditId}/tickets/${ticketId}/move`, payload)
    .then(res => res.data)
}

export const getTicketSubtasks = async (
  auditId: string,
  ticketId: string
): Promise<TicketSummary[]> => {
  return getAxiosClient()
    .get<TicketSummary[]>(`/audits/${auditId}/tickets/${ticketId}/subtasks`)
    .then(res => res.data)
}

export const getColumns = async (
  auditId: string
): Promise<ColumnResponse[]> => {
  return getAxiosClient()
    .get<ColumnResponse[]>(`/audits/${auditId}/columns`)
    .then(res => res.data)
}

export const createColumn = async (
  auditId: string,
  payload: CreateColumnPayload
): Promise<ColumnResponse> => {
  return getAxiosClient()
    .post<ColumnResponse>(`/audits/${auditId}/columns`, payload)
    .then(res => res.data)
}

export const updateColumn = async (
  auditId: string,
  columnId: string,
  payload: UpdateColumnPayload
): Promise<ColumnResponse> => {
  return getAxiosClient()
    .patch<ColumnResponse>(`/audits/${auditId}/columns/${columnId}`, payload)
    .then(res => res.data)
}

export const deleteColumn = async (
  auditId: string,
  columnId: string
): Promise<void> => {
  return getAxiosClient()
    .delete(`/audits/${auditId}/columns/${columnId}`)
    .then(res => res.data)
}

export const reorderColumns = async (
  auditId: string,
  payload: ReorderColumnsPayload
): Promise<void> => {
  return getAxiosClient()
    .put(`/audits/${auditId}/columns/reorder`, payload)
    .then(res => res.data)
}

export const getLabels = async (auditId: string): Promise<LabelResponse[]> => {
  return getAxiosClient()
    .get<LabelResponse[]>(`/audits/${auditId}/labels`)
    .then(res => res.data)
}

export const createLabel = async (
  auditId: string,
  payload: CreateLabelPayload
): Promise<LabelResponse> => {
  return getAxiosClient()
    .post<LabelResponse>(`/audits/${auditId}/labels`, payload)
    .then(res => res.data)
}

export const updateLabel = async (
  auditId: string,
  labelId: string,
  payload: UpdateLabelPayload
): Promise<LabelResponse> => {
  return getAxiosClient()
    .patch<LabelResponse>(`/audits/${auditId}/labels/${labelId}`, payload)
    .then(res => res.data)
}

export const deleteLabel = async (
  auditId: string,
  labelId: string
): Promise<void> => {
  return getAxiosClient()
    .delete(`/audits/${auditId}/labels/${labelId}`)
    .then(res => res.data)
}

export const addLabelToTicket = async (
  auditId: string,
  ticketId: string,
  labelId: string
): Promise<void> => {
  return getAxiosClient()
    .post(`/audits/${auditId}/tickets/${ticketId}/labels/${labelId}`)
    .then(res => res.data)
}

export const removeLabelFromTicket = async (
  auditId: string,
  ticketId: string,
  labelId: string
): Promise<void> => {
  return getAxiosClient()
    .delete(`/audits/${auditId}/tickets/${ticketId}/labels/${labelId}`)
    .then(res => res.data)
}

export const getComments = async (
  auditId: string,
  ticketId: string
): Promise<CommentResponse[]> => {
  return getAxiosClient()
    .get<CommentResponse[]>(`/audits/${auditId}/tickets/${ticketId}/comments`)
    .then(res => res.data)
}

export const createComment = async (
  auditId: string,
  ticketId: string,
  payload: CreateCommentPayload
): Promise<CommentResponse> => {
  return getAxiosClient()
    .post<CommentResponse>(
      `/audits/${auditId}/tickets/${ticketId}/comments`,
      payload
    )
    .then(res => res.data)
}

export const updateComment = async (
  auditId: string,
  ticketId: string,
  commentId: string,
  payload: UpdateCommentPayload
): Promise<CommentResponse> => {
  return getAxiosClient()
    .patch<CommentResponse>(
      `/audits/${auditId}/tickets/${ticketId}/comments/${commentId}`,
      payload
    )
    .then(res => res.data)
}

export const deleteComment = async (
  auditId: string,
  ticketId: string,
  commentId: string
): Promise<void> => {
  return getAxiosClient()
    .delete(`/audits/${auditId}/tickets/${ticketId}/comments/${commentId}`)
    .then(res => res.data)
}

export const getAttachments = async (
  auditId: string,
  ticketId: string
): Promise<AttachmentResponse[]> => {
  return getAxiosClient()
    .get<AttachmentResponse[]>(
      `/audits/${auditId}/tickets/${ticketId}/attachments`
    )
    .then(res => res.data)
}

export const uploadAttachment = async (
  auditId: string,
  ticketId: string,
  file: File
): Promise<AttachmentResponse> => {
  const formData = new FormData()
  formData.append('file', file)

  return getAxiosClient()
    .post<AttachmentResponse>(
      `/audits/${auditId}/tickets/${ticketId}/attachments`,
      formData,
      {
        headers: {
          'Content-Type': 'multipart/form-data'
        }
      }
    )
    .then(res => res.data)
}

export const downloadAttachment = async (
  auditId: string,
  ticketId: string,
  attachmentId: string
): Promise<Blob> => {
  return getAxiosClient()
    .get<Blob>(
      `/audits/${auditId}/tickets/${ticketId}/attachments/${attachmentId}`,
      {
        responseType: 'blob'
      }
    )
    .then(res => res.data)
}

export const getAttachmentPreview = async (
  auditId: string,
  ticketId: string,
  attachmentId: string
): Promise<Blob> => {
  return getAxiosClient()
    .get<Blob>(
      `/audits/${auditId}/tickets/${ticketId}/attachments/${attachmentId}`,
      {
        responseType: 'blob'
      }
    )
    .then(res => res.data)
}

export const deleteAttachment = async (
  auditId: string,
  ticketId: string,
  attachmentId: string
): Promise<void> => {
  return getAxiosClient()
    .delete(
      `/audits/${auditId}/tickets/${ticketId}/attachments/${attachmentId}`
    )
    .then(res => res.data)
}
