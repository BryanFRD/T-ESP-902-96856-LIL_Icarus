import type { TFunction } from 'i18next'
import z from 'zod'

export const ticketPriorities = ['LOW', 'MEDIUM', 'HIGH', 'CRITICAL'] as const

export const createTicketSchema = (t: TFunction) => {
  return z.object({
    title: z
      .string()
      .min(1, t('kanban:validation.ticket.title.required'))
      .max(200, t('kanban:validation.ticket.title.max')),
    columnId: z.string().min(1, t('kanban:validation.ticket.column.required')),
    description: z
      .string()
      .max(5000, t('kanban:validation.ticket.description.max'))
      .optional()
      .nullable(),
    assigneeId: z.string().optional().nullable(),
    priority: z.enum(ticketPriorities).optional().nullable(),
    dueDate: z
      .date()
      .optional()
      .nullable()
      .transform(date => (date ? date.toISOString() : null)),
    parentId: z.string().optional().nullable()
  })
}

export const updateTicketSchema = (t: TFunction) => {
  return z.object({
    title: z
      .string()
      .min(1, t('kanban:validation.ticket.title.required'))
      .max(200, t('kanban:validation.ticket.title.max'))
      .optional()
      .nullable(),
    description: z
      .string()
      .max(5000, t('kanban:validation.ticket.description.max'))
      .optional()
      .nullable(),
    assigneeId: z.string().optional().nullable(),
    priority: z.enum(ticketPriorities).optional().nullable(),
    dueDate: z
      .date()
      .optional()
      .nullable()
      .transform(date => (date ? date.toISOString() : null)),
    parentId: z.string().optional().nullable(),
    clearAssignee: z.boolean().optional(),
    clearDueDate: z.boolean().optional(),
    clearParent: z.boolean().optional()
  })
}

export const createColumnSchema = (t: TFunction) => {
  return z.object({
    name: z
      .string()
      .min(1, t('kanban:validation.column.name.required'))
      .max(50, t('kanban:validation.column.name.max'))
  })
}

export const updateColumnSchema = (t: TFunction) => {
  return z.object({
    name: z
      .string()
      .min(1, t('kanban:validation.column.name.required'))
      .max(50, t('kanban:validation.column.name.max'))
  })
}

export const createLabelSchema = (t: TFunction) => {
  return z.object({
    name: z
      .string()
      .min(1, t('kanban:validation.label.name.required'))
      .max(30, t('kanban:validation.label.name.max')),
    color: z
      .string()
      .regex(/^#[0-9A-F]{6}$/i, t('kanban:validation.label.color.invalid'))
  })
}

export const updateLabelSchema = (t: TFunction) => {
  return z.object({
    name: z
      .string()
      .min(1, t('kanban:validation.label.name.required'))
      .max(30, t('kanban:validation.label.name.max'))
      .optional()
      .nullable(),
    color: z
      .string()
      .regex(/^#[0-9A-F]{6}$/i, t('kanban:validation.label.color.invalid'))
      .optional()
      .nullable()
  })
}

export const createCommentSchema = (t: TFunction) => {
  return z.object({
    content: z
      .string()
      .min(1, t('kanban:validation.comment.content.required'))
      .max(2000, t('kanban:validation.comment.content.max'))
  })
}

export const updateCommentSchema = (t: TFunction) => {
  return z.object({
    content: z
      .string()
      .min(1, t('kanban:validation.comment.content.required'))
      .max(2000, t('kanban:validation.comment.content.max'))
  })
}

// Types inférés des schémas Zod pour utilisation avec react-hook-form

export type CreateTicketSchemaType = z.infer<
  ReturnType<typeof createTicketSchema>
>
export type UpdateTicketSchemaType = z.infer<
  ReturnType<typeof updateTicketSchema>
>
export type CreateColumnSchemaType = z.infer<
  ReturnType<typeof createColumnSchema>
>
export type UpdateColumnSchemaType = z.infer<
  ReturnType<typeof updateColumnSchema>
>
export type CreateLabelSchemaType = z.infer<
  ReturnType<typeof createLabelSchema>
>
export type UpdateLabelSchemaType = z.infer<
  ReturnType<typeof updateLabelSchema>
>
export type CreateCommentSchemaType = z.infer<
  ReturnType<typeof createCommentSchema>
>
export type UpdateCommentSchemaType = z.infer<
  ReturnType<typeof updateCommentSchema>
>
