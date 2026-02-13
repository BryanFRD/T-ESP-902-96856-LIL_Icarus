export type TicketPriority = 'LOW' | 'MEDIUM' | 'HIGH' | 'CRITICAL'

export type TicketUser = {
  id: string
  username: string
  firstName: string
  lastName: string
  avatarUrl: string | null
}

export type LabelResponse = {
  id: string
  name: string
  color: string
  createdAt: string
  updatedAt: string
}

export type TicketSummary = {
  id: string
  title: string
  columnId: string
  position: number
  priority: TicketPriority
  dueDate: string | null
  assignee: TicketUser | null
  labels: LabelResponse[]
  parentId: string | null
  commentCount: number
  attachmentCount: number
  subtaskCount: number
}

export type TicketResponse = {
  id: string
  title: string
  description: string | null
  auditId: string
  columnId: string
  position: number
  priority: TicketPriority
  dueDate: string | null
  assignee: TicketUser | null
  createdBy: TicketUser
  labels: LabelResponse[]
  parentId: string | null
  commentCount: number
  attachmentCount: number
  subtaskCount: number
  createdAt: string
  updatedAt: string
}

export type ColumnResponse = {
  id: string
  name: string
  position: number
  createdAt: string
  updatedAt: string
}

export type BoardColumn = {
  id: string
  name: string
  position: number
  tickets: TicketSummary[]
}

export type BoardResponse = {
  auditId: string
  columns: BoardColumn[]
  labels: LabelResponse[]
}

export type CommentResponse = {
  id: string
  ticketId: string
  content: string
  author: TicketUser
  createdAt: string
  updatedAt: string
}

export type AttachmentResponse = {
  id: string
  ticketId: string
  fileName: string
  fileSize: number
  fileUrl: string
  uploadedBy: TicketUser
  createdAt: string
}

export type CreateTicketPayload = {
  title: string
  columnId: string
  description?: string | null
  assigneeId?: string | null
  priority?: TicketPriority | null
  dueDate?: string | null
  parentId?: string | null
}

export type UpdateTicketPayload = {
  title?: string | null
  description?: string | null
  assigneeId?: string | null
  priority?: TicketPriority | null
  dueDate?: string | null
  parentId?: string | null
  clearAssignee?: boolean
  clearDueDate?: boolean
  clearParent?: boolean
}

export type MoveTicketPayload = {
  columnId: string
  position: number
}

export type CreateColumnPayload = {
  name: string
}

export type UpdateColumnPayload = {
  name: string
}

export type ReorderColumnsPayload = {
  columnIds: string[]
}

export type CreateLabelPayload = {
  name: string
  color: string
}

export type UpdateLabelPayload = {
  name?: string | null
  color?: string | null
}

export type CreateCommentPayload = {
  content: string
}

export type UpdateCommentPayload = {
  content: string
}
