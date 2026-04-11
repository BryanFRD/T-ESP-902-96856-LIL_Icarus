import { zodResolver } from '@hookform/resolvers/zod'
import { format } from 'date-fns'
import { fr } from 'date-fns/locale'
import { Edit2, Loader2, Send, Trash2 } from 'lucide-react'
import { useState } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle
} from '@/components/ui/alert-dialog'
import { Button } from '@/components/ui/button'
import { Textarea } from '@/components/ui/textarea'
import { useUserContext } from '@/context/user-context.tsx'
import { canEditComment } from '@/domain/permissions/kanban-permissions.ts'
import {
  type CreateCommentSchemaType,
  createCommentSchema
} from '@/domain/validation/kanban-schema.ts'
import {
  useComments,
  useCreateComment,
  useDeleteComment,
  useUpdateComment
} from '@/hooks/kanban/use-comment-mutation.ts'
import { useGetAvatar } from '@/hooks/user/use-get-avatar.ts'

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

  const { register, handleSubmit, reset } = useForm<CreateCommentSchemaType>({
    resolver: zodResolver(createCommentSchema(t)),
    defaultValues: { content: '' }
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
        <Loader2 className='text-primary h-6 w-6 animate-spin' />
      </div>
    )
  }

  return (
    <div className='space-y-4'>
      <h3 className='flex items-center gap-2 text-lg font-semibold'>
        <Send className='h-5 w-5' />
        {t('kanban:comment.title')}
        {comments && comments.length > 0 && (
          <span className='text-muted-foreground text-sm font-normal'>
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
          <p className='text-muted-foreground text-sm'>
            {t('kanban:comment.noComments')}
          </p>
        )}
      </div>

      <form onSubmit={handleSubmit(onSubmit)} className='space-y-3'>
        <Textarea
          placeholder={t('kanban:comment.placeholder')}
          rows={3}
          {...register('content')}
        />
        <div className='flex justify-end'>
          <Button type='submit' size='sm' disabled={isCreating}>
            {isCreating ? (
              <Loader2 className='mr-1 h-4 w-4 animate-spin' />
            ) : (
              <Send className='mr-1 h-4 w-4' />
            )}
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

  const { register, handleSubmit } = useForm({
    defaultValues: { content: comment.content }
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
    <div className='bg-card rounded-lg border p-4'>
      <div className='mb-2 flex items-start justify-between'>
        <div className='flex items-center gap-2'>
          {avatarUrl ? (
            <img
              src={avatarUrl}
              alt={`${comment.author.firstName} ${comment.author.lastName}`}
              className='h-8 w-8 rounded-full object-cover'
            />
          ) : (
            <div className='bg-primary/10 text-primary flex h-8 w-8 items-center justify-center rounded-full text-sm font-semibold'>
              {comment.author.firstName[0]}
              {comment.author.lastName[0]}
            </div>
          )}
          <div>
            <p className='text-sm font-medium'>
              {comment.author.firstName} {comment.author.lastName}
            </p>
            <p className='text-muted-foreground text-xs'>
              {format(new Date(comment.createdAt), 'PPp', { locale: fr })}
              {comment.createdAt !== comment.updatedAt && (
                <span className='ml-1'>({t('common:edited')})</span>
              )}
            </p>
          </div>
        </div>
        {canEdit && !isEditing && (
          <div className='flex gap-1'>
            <Button variant='ghost' size='icon-sm' onClick={onEdit}>
              <Edit2 className='h-4 w-4' />
            </Button>
            <Button variant='ghost' size='icon-sm' onClick={onDelete}>
              <Trash2 className='text-destructive h-4 w-4' />
            </Button>
          </div>
        )}
      </div>

      {isEditing ? (
        <form onSubmit={handleSubmit(onSubmit)} className='space-y-2'>
          <Textarea rows={3} {...register('content')} />
          <div className='flex justify-end gap-2'>
            <Button
              type='button'
              variant='outline'
              size='sm'
              onClick={onCancelEdit}
              disabled={isPending}
            >
              {t('common:actions.cancel')}
            </Button>
            <Button type='submit' size='sm' disabled={isPending}>
              {isPending && <Loader2 className='mr-2 h-4 w-4 animate-spin' />}
              {t('common:actions.save')}
            </Button>
          </div>
        </form>
      ) : (
        <p className='text-sm whitespace-pre-wrap'>{comment.content}</p>
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
    <AlertDialog open onOpenChange={open => !open && onClose()}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>{t('kanban:comment.delete')}</AlertDialogTitle>
          <AlertDialogDescription>
            {t('kanban:comment.deleteConfirm')}
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel disabled={isPending}>
            {t('common:actions.cancel')}
          </AlertDialogCancel>
          <AlertDialogAction
            onClick={handleDelete}
            disabled={isPending}
            className='bg-destructive text-destructive-foreground hover:bg-destructive/90'
          >
            {isPending && <Loader2 className='mr-2 h-4 w-4 animate-spin' />}
            {t('common:actions.delete')}
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  )
}

export default CommentSection
