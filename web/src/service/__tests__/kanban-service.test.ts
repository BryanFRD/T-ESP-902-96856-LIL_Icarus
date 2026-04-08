import { beforeEach, describe, expect, it, vi } from 'vitest'

import { getAxiosClient } from '@/configuration/api/axios-client.ts'

import {
  addLabelToTicket,
  createColumn,
  createComment,
  createLabel,
  createTicket,
  deleteAttachment,
  deleteColumn,
  deleteComment,
  deleteLabel,
  deleteTicket,
  downloadAttachment,
  getAttachmentPreview,
  getAttachments,
  getColumns,
  getComments,
  getKanbanBoard,
  getLabels,
  getTicket,
  getTicketSubtasks,
  getTickets,
  moveTicket,
  removeLabelFromTicket,
  reorderColumns,
  updateColumn,
  updateComment,
  updateLabel,
  updateTicket,
  uploadAttachment
} from '../kanban-service.ts'

vi.mock('@/configuration/api/axios-client.ts', () => ({
  getAxiosClient: vi.fn()
}))

const mockClient = {
  get: vi.fn(),
  post: vi.fn(),
  put: vi.fn(),
  patch: vi.fn(),
  delete: vi.fn()
}

const AUDIT = 'a1'
const TICKET = 't1'
const COLUMN = 'c1'
const LABEL = 'l1'
const COMMENT = 'cm1'
const ATTACHMENT = 'at1'

describe('kanban-service', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    vi.mocked(getAxiosClient).mockReturnValue(mockClient as never)
  })

  describe('board', () => {
    it('getKanbanBoard should GET /audits/:auditId/board', async () => {
      const mockBoard = { auditId: AUDIT, columns: [], labels: [] }
      mockClient.get.mockResolvedValue({ data: mockBoard })
      const result = await getKanbanBoard(AUDIT)
      expect(mockClient.get).toHaveBeenCalledWith(`/audits/${AUDIT}/board`)
      expect(result).toEqual(mockBoard)
    })
  })

  describe('tickets', () => {
    it('getTickets should GET /audits/:auditId/tickets', async () => {
      const mockTickets = [{ id: TICKET, title: 'Ticket 1' }]
      mockClient.get.mockResolvedValue({ data: mockTickets })
      const result = await getTickets(AUDIT)
      expect(mockClient.get).toHaveBeenCalledWith(`/audits/${AUDIT}/tickets`)
      expect(result).toEqual(mockTickets)
    })

    it('getTicket should GET /audits/:auditId/tickets/:ticketId', async () => {
      const mockTicket = { id: TICKET, title: 'Ticket 1' }
      mockClient.get.mockResolvedValue({ data: mockTicket })
      const result = await getTicket(AUDIT, TICKET)
      expect(mockClient.get).toHaveBeenCalledWith(
        `/audits/${AUDIT}/tickets/${TICKET}`
      )
      expect(result).toEqual(mockTicket)
    })

    it('createTicket should POST /audits/:auditId/tickets', async () => {
      const payload = { title: 'New', columnId: COLUMN }
      const mockTicket = { id: TICKET, ...payload }
      mockClient.post.mockResolvedValue({ data: mockTicket })
      const result = await createTicket(AUDIT, payload)
      expect(mockClient.post).toHaveBeenCalledWith(
        `/audits/${AUDIT}/tickets`,
        payload
      )
      expect(result).toEqual(mockTicket)
    })

    it('updateTicket should PATCH /audits/:auditId/tickets/:ticketId', async () => {
      const payload = { title: 'Updated' }
      const mockTicket = { id: TICKET, title: 'Updated' }
      mockClient.patch.mockResolvedValue({ data: mockTicket })
      const result = await updateTicket(AUDIT, TICKET, payload)
      expect(mockClient.patch).toHaveBeenCalledWith(
        `/audits/${AUDIT}/tickets/${TICKET}`,
        payload
      )
      expect(result).toEqual(mockTicket)
    })

    it('deleteTicket should DELETE /audits/:auditId/tickets/:ticketId', async () => {
      mockClient.delete.mockResolvedValue({ data: undefined })
      await deleteTicket(AUDIT, TICKET)
      expect(mockClient.delete).toHaveBeenCalledWith(
        `/audits/${AUDIT}/tickets/${TICKET}`
      )
    })

    it('moveTicket should PUT /audits/:auditId/tickets/:ticketId/move', async () => {
      const payload = { columnId: 'c2', position: 0 }
      const mockTicket = { id: TICKET }
      mockClient.put.mockResolvedValue({ data: mockTicket })
      const result = await moveTicket(AUDIT, TICKET, payload)
      expect(mockClient.put).toHaveBeenCalledWith(
        `/audits/${AUDIT}/tickets/${TICKET}/move`,
        payload
      )
      expect(result).toEqual(mockTicket)
    })

    it('getTicketSubtasks should GET /audits/:auditId/tickets/:ticketId/subtasks', async () => {
      const mockSubtasks = [{ id: 't2' }]
      mockClient.get.mockResolvedValue({ data: mockSubtasks })
      const result = await getTicketSubtasks(AUDIT, TICKET)
      expect(mockClient.get).toHaveBeenCalledWith(
        `/audits/${AUDIT}/tickets/${TICKET}/subtasks`
      )
      expect(result).toEqual(mockSubtasks)
    })
  })

  describe('columns', () => {
    it('getColumns should GET /audits/:auditId/columns', async () => {
      const mockColumns = [{ id: COLUMN, name: 'Todo' }]
      mockClient.get.mockResolvedValue({ data: mockColumns })
      const result = await getColumns(AUDIT)
      expect(mockClient.get).toHaveBeenCalledWith(`/audits/${AUDIT}/columns`)
      expect(result).toEqual(mockColumns)
    })

    it('createColumn should POST /audits/:auditId/columns', async () => {
      const payload = { name: 'New Column' }
      mockClient.post.mockResolvedValue({ data: { id: COLUMN, ...payload } })
      const result = await createColumn(AUDIT, payload)
      expect(mockClient.post).toHaveBeenCalledWith(
        `/audits/${AUDIT}/columns`,
        payload
      )
      expect(result).toEqual({ id: COLUMN, ...payload })
    })

    it('updateColumn should PATCH /audits/:auditId/columns/:columnId', async () => {
      const payload = { name: 'Renamed' }
      mockClient.patch.mockResolvedValue({ data: { id: COLUMN, ...payload } })
      const result = await updateColumn(AUDIT, COLUMN, payload)
      expect(mockClient.patch).toHaveBeenCalledWith(
        `/audits/${AUDIT}/columns/${COLUMN}`,
        payload
      )
      expect(result).toEqual({ id: COLUMN, ...payload })
    })

    it('deleteColumn should DELETE /audits/:auditId/columns/:columnId', async () => {
      mockClient.delete.mockResolvedValue({ data: undefined })
      await deleteColumn(AUDIT, COLUMN)
      expect(mockClient.delete).toHaveBeenCalledWith(
        `/audits/${AUDIT}/columns/${COLUMN}`
      )
    })

    it('reorderColumns should PUT /audits/:auditId/columns/reorder', async () => {
      const payload = { columnIds: ['c1', 'c2', 'c3'] }
      mockClient.put.mockResolvedValue({ data: undefined })
      await reorderColumns(AUDIT, payload)
      expect(mockClient.put).toHaveBeenCalledWith(
        `/audits/${AUDIT}/columns/reorder`,
        payload
      )
    })
  })

  describe('labels', () => {
    it('getLabels should GET /audits/:auditId/labels', async () => {
      mockClient.get.mockResolvedValue({ data: [{ id: LABEL }] })
      const result = await getLabels(AUDIT)
      expect(mockClient.get).toHaveBeenCalledWith(`/audits/${AUDIT}/labels`)
      expect(result).toEqual([{ id: LABEL }])
    })

    it('createLabel should POST /audits/:auditId/labels', async () => {
      const payload = { name: 'Bug', color: '#FF0000' }
      mockClient.post.mockResolvedValue({ data: { id: LABEL, ...payload } })
      const result = await createLabel(AUDIT, payload)
      expect(mockClient.post).toHaveBeenCalledWith(
        `/audits/${AUDIT}/labels`,
        payload
      )
      expect(result).toEqual({ id: LABEL, ...payload })
    })

    it('updateLabel should PATCH /audits/:auditId/labels/:labelId', async () => {
      const payload = { name: 'Feature' }
      mockClient.patch.mockResolvedValue({ data: { id: LABEL, ...payload } })
      const result = await updateLabel(AUDIT, LABEL, payload)
      expect(mockClient.patch).toHaveBeenCalledWith(
        `/audits/${AUDIT}/labels/${LABEL}`,
        payload
      )
      expect(result).toEqual({ id: LABEL, ...payload })
    })

    it('deleteLabel should DELETE /audits/:auditId/labels/:labelId', async () => {
      mockClient.delete.mockResolvedValue({ data: undefined })
      await deleteLabel(AUDIT, LABEL)
      expect(mockClient.delete).toHaveBeenCalledWith(
        `/audits/${AUDIT}/labels/${LABEL}`
      )
    })

    it('addLabelToTicket should POST /audits/:auditId/tickets/:ticketId/labels/:labelId', async () => {
      mockClient.post.mockResolvedValue({ data: undefined })
      await addLabelToTicket(AUDIT, TICKET, LABEL)
      expect(mockClient.post).toHaveBeenCalledWith(
        `/audits/${AUDIT}/tickets/${TICKET}/labels/${LABEL}`
      )
    })

    it('removeLabelFromTicket should DELETE /audits/:auditId/tickets/:ticketId/labels/:labelId', async () => {
      mockClient.delete.mockResolvedValue({ data: undefined })
      await removeLabelFromTicket(AUDIT, TICKET, LABEL)
      expect(mockClient.delete).toHaveBeenCalledWith(
        `/audits/${AUDIT}/tickets/${TICKET}/labels/${LABEL}`
      )
    })
  })

  describe('comments', () => {
    it('getComments should GET /audits/:auditId/tickets/:ticketId/comments', async () => {
      mockClient.get.mockResolvedValue({ data: [{ id: COMMENT }] })
      const result = await getComments(AUDIT, TICKET)
      expect(mockClient.get).toHaveBeenCalledWith(
        `/audits/${AUDIT}/tickets/${TICKET}/comments`
      )
      expect(result).toEqual([{ id: COMMENT }])
    })

    it('createComment should POST /audits/:auditId/tickets/:ticketId/comments', async () => {
      const payload = { content: 'Hello' }
      mockClient.post.mockResolvedValue({ data: { id: COMMENT, ...payload } })
      const result = await createComment(AUDIT, TICKET, payload)
      expect(mockClient.post).toHaveBeenCalledWith(
        `/audits/${AUDIT}/tickets/${TICKET}/comments`,
        payload
      )
      expect(result).toEqual({ id: COMMENT, ...payload })
    })

    it('updateComment should PATCH /audits/:auditId/tickets/:ticketId/comments/:commentId', async () => {
      const payload = { content: 'Updated' }
      mockClient.patch.mockResolvedValue({ data: { id: COMMENT, ...payload } })
      const result = await updateComment(AUDIT, TICKET, COMMENT, payload)
      expect(mockClient.patch).toHaveBeenCalledWith(
        `/audits/${AUDIT}/tickets/${TICKET}/comments/${COMMENT}`,
        payload
      )
      expect(result).toEqual({ id: COMMENT, ...payload })
    })

    it('deleteComment should DELETE /audits/:auditId/tickets/:ticketId/comments/:commentId', async () => {
      mockClient.delete.mockResolvedValue({ data: undefined })
      await deleteComment(AUDIT, TICKET, COMMENT)
      expect(mockClient.delete).toHaveBeenCalledWith(
        `/audits/${AUDIT}/tickets/${TICKET}/comments/${COMMENT}`
      )
    })
  })

  describe('attachments', () => {
    it('getAttachments should GET /audits/:auditId/tickets/:ticketId/attachments', async () => {
      mockClient.get.mockResolvedValue({ data: [{ id: ATTACHMENT }] })
      const result = await getAttachments(AUDIT, TICKET)
      expect(mockClient.get).toHaveBeenCalledWith(
        `/audits/${AUDIT}/tickets/${TICKET}/attachments`
      )
      expect(result).toEqual([{ id: ATTACHMENT }])
    })

    it('uploadAttachment should POST with FormData', async () => {
      const file = new File(['content'], 'doc.pdf', { type: 'application/pdf' })
      mockClient.post.mockResolvedValue({
        data: { id: ATTACHMENT, fileName: 'doc.pdf' }
      })
      const result = await uploadAttachment(AUDIT, TICKET, file)
      expect(mockClient.post).toHaveBeenCalledWith(
        `/audits/${AUDIT}/tickets/${TICKET}/attachments`,
        expect.any(FormData),
        { headers: { 'Content-Type': 'multipart/form-data' } }
      )
      expect(result).toEqual({ id: ATTACHMENT, fileName: 'doc.pdf' })
    })

    it('downloadAttachment should GET with blob responseType', async () => {
      const mockBlob = new Blob(['content'])
      mockClient.get.mockResolvedValue({ data: mockBlob })
      const result = await downloadAttachment(AUDIT, TICKET, ATTACHMENT)
      expect(mockClient.get).toHaveBeenCalledWith(
        `/audits/${AUDIT}/tickets/${TICKET}/attachments/${ATTACHMENT}`,
        { responseType: 'blob' }
      )
      expect(result).toEqual(mockBlob)
    })

    it('getAttachmentPreview should GET with blob responseType', async () => {
      const mockBlob = new Blob(['content'])
      mockClient.get.mockResolvedValue({ data: mockBlob })
      const result = await getAttachmentPreview(AUDIT, TICKET, ATTACHMENT)
      expect(mockClient.get).toHaveBeenCalledWith(
        `/audits/${AUDIT}/tickets/${TICKET}/attachments/${ATTACHMENT}`,
        { responseType: 'blob' }
      )
      expect(result).toEqual(mockBlob)
    })

    it('deleteAttachment should DELETE', async () => {
      mockClient.delete.mockResolvedValue({ data: undefined })
      await deleteAttachment(AUDIT, TICKET, ATTACHMENT)
      expect(mockClient.delete).toHaveBeenCalledWith(
        `/audits/${AUDIT}/tickets/${TICKET}/attachments/${ATTACHMENT}`
      )
    })
  })
})
