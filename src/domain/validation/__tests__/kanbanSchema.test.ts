import { describe, expect, it } from 'vitest'

import {
  createColumnSchema,
  createCommentSchema,
  createLabelSchema,
  createTicketSchema,
  updateLabelSchema,
  updateTicketSchema
} from '../kanban-schema.ts'

const t = (key: string) => key

describe('kanban schemas', () => {
  describe('createTicketSchema', () => {
    const schema = createTicketSchema(t as never)

    it('should accept valid ticket', () => {
      const result = schema.safeParse({ title: 'Bug fix', columnId: 'col1' })
      expect(result.success).toBe(true)
    })

    it('should reject empty title', () => {
      const result = schema.safeParse({ title: '', columnId: 'col1' })
      expect(result.success).toBe(false)
    })

    it('should reject title longer than 200 chars', () => {
      const result = schema.safeParse({
        title: 'a'.repeat(201),
        columnId: 'col1'
      })
      expect(result.success).toBe(false)
    })

    it('should accept optional fields', () => {
      const result = schema.safeParse({
        title: 'Task',
        columnId: 'col1',
        description: 'Details',
        priority: 'HIGH',
        assigneeId: 'user1'
      })
      expect(result.success).toBe(true)
    })
  })

  describe('updateTicketSchema', () => {
    const schema = updateTicketSchema(t as never)

    it('should accept partial update', () => {
      const result = schema.safeParse({ title: 'Updated' })
      expect(result.success).toBe(true)
    })

    it('should accept clear flags', () => {
      const result = schema.safeParse({
        clearAssignee: true,
        clearDueDate: true
      })
      expect(result.success).toBe(true)
    })
  })

  describe('createColumnSchema', () => {
    const schema = createColumnSchema(t as never)

    it('should accept valid column name', () => {
      const result = schema.safeParse({ name: 'Todo' })
      expect(result.success).toBe(true)
    })

    it('should reject empty name', () => {
      const result = schema.safeParse({ name: '' })
      expect(result.success).toBe(false)
    })

    it('should reject name longer than 50 chars', () => {
      const result = schema.safeParse({ name: 'a'.repeat(51) })
      expect(result.success).toBe(false)
    })
  })

  describe('createLabelSchema', () => {
    const schema = createLabelSchema(t as never)

    it('should accept valid label', () => {
      const result = schema.safeParse({ name: 'Bug', color: '#FF0000' })
      expect(result.success).toBe(true)
    })

    it('should reject invalid hex color', () => {
      const result = schema.safeParse({ name: 'Bug', color: 'red' })
      expect(result.success).toBe(false)
    })

    it('should reject name longer than 30 chars', () => {
      const result = schema.safeParse({
        name: 'a'.repeat(31),
        color: '#FF0000'
      })
      expect(result.success).toBe(false)
    })
  })

  describe('updateLabelSchema', () => {
    const schema = updateLabelSchema(t as never)

    it('should accept partial update', () => {
      const result = schema.safeParse({ name: 'Feature' })
      expect(result.success).toBe(true)
    })

    it('should accept empty object', () => {
      const result = schema.safeParse({})
      expect(result.success).toBe(true)
    })
  })

  describe('createCommentSchema', () => {
    const schema = createCommentSchema(t as never)

    it('should accept valid comment', () => {
      const result = schema.safeParse({ content: 'Nice work!' })
      expect(result.success).toBe(true)
    })

    it('should reject empty content', () => {
      const result = schema.safeParse({ content: '' })
      expect(result.success).toBe(false)
    })

    it('should reject content longer than 2000 chars', () => {
      const result = schema.safeParse({ content: 'a'.repeat(2001) })
      expect(result.success).toBe(false)
    })
  })
})
