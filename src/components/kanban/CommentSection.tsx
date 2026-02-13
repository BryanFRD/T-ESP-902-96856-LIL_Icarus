import { zodResolver } from '@hookform/resolvers/zod'
import { format } from 'date-fns'
import { fr } from 'date-fns/locale'
import { Edit2, Loader2, Send, Trash2 } from 'lucide-react'
import { useState } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'

import Textarea from '@/components/ui/Textarea'
import Button from '@/components/ui/button/Button'
import ModalConfirm from '@/components/ui/modal/ModalConfirm'
import { useUserContext } from '@/context/UserContext'
import { canEditComment } from '@/domain/permissions/kanbanPermissions'
import {
  type CreateCommentSchemaType,
  createCommentSchema
} from '@/domain/validation/kanbanSchema'
import {
  useComments,
  useCreateComment,
  useDeleteComment,
  useUpdateComment
} from '@/hooks/kanban/useCommentMutations'
import { useGetAvatar } from '@/hooks/user/useGetAvatar'

type CommentSectionProps = {
  auditId: string
  ticketId: string
}

const CommentSection = ({ auditId, ticketId }: CommentSectionProps) => {
  const { t } = useTranslation()
  const { user } = useUserContext()
  const { data: comments, isLoading } = useComments(auditId, ticketId)
  const { mutateAsync: createComment, isPending: isCreating } =
    useCreateComment(auditId, ticketId)
  const [editingCommentId, setEditingCommentId] = useState<string | null>(null)
  const [deletingCommentId, setDeletingCommentId] = useState<string | null>(
    null
  )

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors }
  } = useForm<CreateCommentSchemaType>({
    resolver: zodResolver(createCommentSchema(t)),
    defaultValues: {
      content: ''
    }
  })

  const onSubmit = async (data: CreateCommentSchemaType) => {
    try {
      await createComment(data)
      reset()
    } catch {
      // Error handled by mutation
    }
  }

  if (isLoading) {
    return (
      <div className='flex items-center justify-center py-8'>
        <Loader2 className='h-6 w-6 animate-spin text-orange-500' />
      </div>
    )
  }

  return (
    <div className='space-y-4'>
      <h3 className='flex items-center gap-2 text-lg font-semibold text-gray-900 dark:text-gray-100'>
        <Send className='h-5 w-5' />
        {t('kanban:comment.title')}
        {comments && comments.length > 0 && (
          <span className='text-sm font-normal text-gray-500'>
            ({comments.length})
          </span>
        )}
      </h3>

      <div className='space-y-3'>
        {comments?.map(comment => (
          <CommentItem
            key={comment.id}
            comment={comment}
            auditId={auditId}
            ticketId={ticketId}
            isEditing={editingCommentId === comment.id}
            onEdit={() => setEditingCommentId(comment.id)}
            onCancelEdit={() => setEditingCommentId(null)}
            onDelete={() => setDeletingCommentId(comment.id)}
            canEdit={user ? canEditComment(user.id, comment.author.id) : false}
          />
        ))}

        {(!comments || comments.length === 0) && (
          <p className='text-sm text-gray-500 dark:text-gray-400'>
            {t('kanban:comment.noComments')}
          </p>
        )}
      </div>

      <form onSubmit={handleSubmit(onSubmit)} className='space-y-3'>
        <Textarea
          placeholder={t('kanban:comment.placeholder')}
          error={errors.content?.message}
          rows={3}
          {...register('content')}
        />
        <div className='flex justify-end'>
          <Button
            type='submit'
            variant='primary'
            size='sm'
            isLoading={isCreating}
          >
            <Send className='h-4 w-4' />
            {t('kanban:comment.add')}
          </Button>
        </div>
      </form>

      {deletingCommentId && (
        <DeleteCommentModal
          auditId={auditId}
          ticketId={ticketId}
          commentId={deletingCommentId}
          onClose={() => setDeletingCommentId(null)}
        />
      )}
    </div>
  )
}

type CommentItemProps = {
  comment: {
    id: string
    content: string
    author: {
      id: string
      firstName: string
      lastName: string
      avatarUrl: string | null
    }
    createdAt: string
    updatedAt: string
  }
  auditId: string
  ticketId: string
  isEditing: boolean
  onEdit: () => void
  onCancelEdit: () => void
  onDelete: () => void
  canEdit: boolean
}

const CommentItem = ({
  comment,
  auditId,
  ticketId,
  isEditing,
  onEdit,
  onCancelEdit,
  onDelete,
  canEdit
}: CommentItemProps) => {
  const { t } = useTranslation()
  const { mutateAsync: updateComment, isPending } = useUpdateComment(
    auditId,
    ticketId,
    comment.id
  )
  const { data: avatarUrl } = useGetAvatar(
    comment.author.avatarUrl || undefined
  )

  const {
    register,
    handleSubmit,
    formState: { errors }
  } = useForm({
    defaultValues: {
      content: comment.content
    }
  })

  const onSubmit = async (data: { content: string }) => {
    try {
      await updateComment(data)
      onCancelEdit()
    } catch {
      // Error handled by mutation
    }
  }

  return (
    <div className='rounded-lg border border-gray-200 bg-white p-4 dark:border-gray-700 dark:bg-gray-800'>
      <div className='mb-2 flex items-start justify-between'>
        <div className='flex items-center gap-2'>
          {avatarUrl ? (
            <img
              src={avatarUrl}
              alt={`${comment.author.firstName} ${comment.author.lastName}`}
              className='h-8 w-8 rounded-full object-cover'
            />
          ) : (
            <div className='flex h-8 w-8 items-center justify-center rounded-full bg-orange-100 text-sm font-semibold text-orange-600 dark:bg-orange-900/20 dark:text-orange-400'>
              {comment.author.firstName[0]}
              {comment.author.lastName[0]}
            </div>
          )}
          <div>
            <p className='text-sm font-medium text-gray-900 dark:text-gray-100'>
              {comment.author.firstName} {comment.author.lastName}
            </p>
            <p className='text-xs text-gray-500 dark:text-gray-400'>
              {format(new Date(comment.createdAt), 'PPp', { locale: fr })}
              {comment.createdAt !== comment.updatedAt && (
                <span className='ml-1'>({t('common:edited')})</span>
              )}
            </p>
          </div>
        </div>
        {canEdit && !isEditing && (
          <div className='flex gap-1'>
            <Button variant='ghost' size='sm' onClick={onEdit}>
              <Edit2 className='h-4 w-4' />
            </Button>
            <Button variant='ghost' size='sm' onClick={onDelete}>
              <Trash2 className='h-4 w-4 text-red-500' />
            </Button>
          </div>
        )}
      </div>

      {isEditing ? (
        <form onSubmit={handleSubmit(onSubmit)} className='space-y-2'>
          <Textarea
            error={errors.content?.message}
            rows={3}
            {...register('content')}
          />
          <div className='flex justify-end gap-2'>
            <Button
              type='button'
              variant='ghost'
              size='sm'
              onClick={onCancelEdit}
              disabled={isPending}
            >
              {t('common:cancel')}
            </Button>
            <Button
              type='submit'
              variant='primary'
              size='sm'
              isLoading={isPending}
            >
              {t('common:save')}
            </Button>
          </div>
        </form>
      ) : (
        <p className='text-sm whitespace-pre-wrap text-gray-700 dark:text-gray-300'>
          {comment.content}
        </p>
      )}
    </div>
  )
}

type DeleteCommentModalProps = {
  auditId: string
  ticketId: string
  commentId: string
  onClose: () => void
}

const DeleteCommentModal = ({
  auditId,
  ticketId,
  commentId,
  onClose
}: DeleteCommentModalProps) => {
  const { t } = useTranslation()
  const { mutateAsync: deleteComment, isPending } = useDeleteComment(
    auditId,
    ticketId
  )

  const handleDelete = async () => {
    try {
      await deleteComment(commentId)
      onClose()
    } catch {
      // Error handled by mutation
    }
  }

  return (
    <ModalConfirm
      isOpen
      onClose={onClose}
      onConfirm={handleDelete}
      title={t('kanban:comment.delete')}
      confirmText={t('common:delete')}
      isLoading={isPending}
      variant='danger'
      description={t('kanban:comment.deleteConfirm')}
      confirmMatch={t('kanban:comment.confirmDelete')}
    />
  )
}

export default CommentSection
