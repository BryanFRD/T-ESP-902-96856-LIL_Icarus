import { format } from 'date-fns'
import { fr } from 'date-fns/locale'
import {
  Download,
  FileIcon,
  Image as ImageIcon,
  Loader2,
  Paperclip,
  Trash2,
  Upload,
  X
} from 'lucide-react'
import { useRef, useState } from 'react'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'

import Button from '@/components/oldUi/button/Button'
import ModalConfirm from '@/components/oldUi/modal/ModalConfirm'
import { useUserContext } from '@/context/user-context.tsx'
import type { AttachmentResponse } from '@/domain/types/kanban.ts'
import {
  useAttachmentPreview,
  useAttachments,
  useDeleteAttachment,
  useDownloadAttachment,
  useUploadAttachment
} from '@/hooks/kanban/use-attachment-mutation.ts'

const AttachmentThumbnail = ({
  auditId,
  ticketId,
  attachment
}: {
  auditId: string
  ticketId: string
  attachment: AttachmentResponse
}) => {
  const { data: previewUrl, isLoading } = useAttachmentPreview(
    auditId,
    ticketId,
    attachment.id
  )

  if (isLoading) {
    return (
      <div className='flex h-16 w-16 items-center justify-center rounded-lg border border-gray-200 bg-gray-50 dark:border-gray-600 dark:bg-gray-700'>
        <Loader2 className='h-4 w-4 animate-spin text-gray-400' />
      </div>
    )
  }

  return (
    <a
      href={previewUrl}
      target='_blank'
      rel='noopener noreferrer'
      className='h-16 w-16 overflow-hidden rounded-lg border border-gray-200 hover:opacity-80 dark:border-gray-600'
      title={attachment.fileName}
    >
      <img
        src={previewUrl}
        alt={attachment.fileName}
        className='h-full w-full object-cover'
      />
    </a>
  )
}

type AttachmentSectionProps = {
  auditId: string
  ticketId: string
}

const AttachmentSection = ({ auditId, ticketId }: AttachmentSectionProps) => {
  const { t } = useTranslation()
  const { user } = useUserContext()
  const { data: attachments, isLoading } = useAttachments(auditId, ticketId)
  const { mutateAsync: uploadAttachment, isPending: isUploading } =
    useUploadAttachment(auditId, ticketId)
  const { mutateAsync: downloadAttachment } = useDownloadAttachment(
    auditId,
    ticketId
  )
  const fileInputRef = useRef<HTMLInputElement>(null)
  const [deletingAttachmentId, setDeletingAttachmentId] = useState<
    string | null
  >(null)
  const [previewFile, setPreviewFile] = useState<{
    file: File
    preview: string
  } | null>(null)

  const handleFileSelect = async (
    event: React.ChangeEvent<HTMLInputElement>
  ) => {
    const file = event.target.files?.[0]
    if (!file) return

    const maxSize = 10 * 1024 * 1024
    if (file.size > maxSize) {
      toast.error(t('kanban:attachment.fileTooLarge'))
      return
    }

    if (file.type.startsWith('image/')) {
      const preview = URL.createObjectURL(file)
      setPreviewFile({ file, preview })
    } else {
      await uploadFile(file)
    }

    if (fileInputRef.current) {
      fileInputRef.current.value = ''
    }
  }

  const uploadFile = async (file: File) => {
    try {
      await uploadAttachment(file)
    } catch {
      // Error handled by mutation
    }
  }

  const handleConfirmUpload = async () => {
    if (!previewFile) return
    await uploadFile(previewFile.file)
    URL.revokeObjectURL(previewFile.preview)
    setPreviewFile(null)
  }

  const handleCancelUpload = () => {
    if (previewFile) {
      URL.revokeObjectURL(previewFile.preview)
      setPreviewFile(null)
    }
  }

  const handleDownload = async (attachmentId: string, fileName: string) => {
    try {
      const blob = await downloadAttachment(attachmentId)
      const url = window.URL.createObjectURL(blob)
      const link = document.createElement('a')
      link.href = url
      link.download = fileName
      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)
      window.URL.revokeObjectURL(url)
    } catch {
      // Error handled by mutation
    }
  }

  const formatFileSize = (bytes: number): string => {
    if (bytes === 0) return '0 B'
    const k = 1024
    const sizes = ['B', 'KB', 'MB', 'GB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`
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
      <div className='flex items-center justify-between'>
        <h3 className='flex items-center gap-2 text-lg font-semibold text-gray-900 dark:text-gray-100'>
          <Paperclip className='h-5 w-5' />
          {t('kanban:attachment.title')}
          {attachments && attachments.length > 0 && (
            <span className='text-sm font-normal text-gray-500'>
              ({attachments.length})
            </span>
          )}
        </h3>
        <Button
          variant='ghost'
          size='sm'
          onClick={() => fileInputRef.current?.click()}
          disabled={isUploading}
        >
          {isUploading ? (
            <Loader2 className='h-4 w-4 animate-spin' />
          ) : (
            <Upload className='h-4 w-4' />
          )}
          {t('kanban:attachment.add')}
        </Button>
        <input
          ref={fileInputRef}
          type='file'
          className='hidden'
          onChange={handleFileSelect}
          accept='*/*'
        />
      </div>

      {previewFile && (
        <div className='rounded-lg border-2 border-orange-500 bg-orange-50 p-4 dark:border-orange-700 dark:bg-orange-900/20'>
          <div className='mb-3 flex items-center justify-between'>
            <div className='flex items-center gap-2'>
              <ImageIcon className='h-5 w-5 text-orange-600 dark:text-orange-400' />
              <span className='font-medium text-gray-900 dark:text-gray-100'>
                {t('kanban:attachment.preview')}
              </span>
            </div>
            <Button variant='ghost' size='sm' onClick={handleCancelUpload}>
              <X className='h-4 w-4' />
            </Button>
          </div>
          <div className='mb-3'>
            <img
              src={previewFile.preview}
              alt='Preview'
              className='max-h-60 w-full rounded border border-gray-200 object-contain dark:border-gray-700'
            />
          </div>
          <div className='mb-3 text-sm text-gray-700 dark:text-gray-300'>
            <p className='font-medium'>{previewFile.file.name}</p>
            <p className='text-gray-500'>
              {formatFileSize(previewFile.file.size)}
            </p>
          </div>
          <div className='flex gap-2'>
            <Button
              size='sm'
              variant='primary'
              onClick={handleConfirmUpload}
              isLoading={isUploading}
            >
              {t('kanban:attachment.upload')}
            </Button>
            <Button
              size='sm'
              variant='ghost'
              onClick={handleCancelUpload}
              disabled={isUploading}
            >
              {t('common:cancel')}
            </Button>
          </div>
        </div>
      )}

      <div className='space-y-2'>
        {attachments?.map(attachment => {
          const isImage = attachment.fileName.match(
            /\.(jpg|jpeg|png|gif|webp|svg)$/i
          )

          return (
            <div
              key={attachment.id}
              className='flex items-center justify-between rounded-lg border border-gray-200 bg-white p-3 dark:border-gray-700 dark:bg-gray-800'
            >
              <div className='flex items-center gap-3'>
                {isImage ? (
                  <AttachmentThumbnail
                    auditId={auditId}
                    ticketId={ticketId}
                    attachment={attachment}
                  />
                ) : (
                  <div className='flex h-16 w-16 items-center justify-center rounded-lg bg-gray-100 dark:bg-gray-700'>
                    <FileIcon className='h-8 w-8 text-gray-600 dark:text-gray-400' />
                  </div>
                )}
                <div className='flex-1'>
                  <p className='text-sm font-medium text-gray-900 dark:text-gray-100'>
                    {attachment.fileName}
                  </p>
                  <div className='flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400'>
                    <span>{formatFileSize(attachment.fileSize)}</span>
                    <span>•</span>
                    <span>
                      {attachment.uploadedBy.firstName}{' '}
                      {attachment.uploadedBy.lastName}
                    </span>
                    <span>•</span>
                    <span>
                      {format(new Date(attachment.createdAt), 'PPp', {
                        locale: fr
                      })}
                    </span>
                  </div>
                </div>
              </div>
              <div className='flex gap-1'>
                <Button
                  variant='ghost'
                  size='sm'
                  onClick={() =>
                    handleDownload(attachment.id, attachment.fileName)
                  }
                >
                  <Download className='h-4 w-4' />
                </Button>
                {user && user.id === attachment.uploadedBy.id && (
                  <Button
                    variant='ghost'
                    size='sm'
                    onClick={() => setDeletingAttachmentId(attachment.id)}
                  >
                    <Trash2 className='h-4 w-4 text-red-500' />
                  </Button>
                )}
              </div>
            </div>
          )
        })}

        {(!attachments || attachments.length === 0) && (
          <p className='text-sm text-gray-500 dark:text-gray-400'>
            {t('kanban:attachment.noAttachments')}
          </p>
        )}
      </div>

      {deletingAttachmentId && (
        <DeleteAttachmentModal
          auditId={auditId}
          ticketId={ticketId}
          attachmentId={deletingAttachmentId}
          onClose={() => setDeletingAttachmentId(null)}
        />
      )}
    </div>
  )
}

type DeleteAttachmentModalProps = {
  auditId: string
  ticketId: string
  attachmentId: string
  onClose: () => void
}

const DeleteAttachmentModal = ({
  auditId,
  ticketId,
  attachmentId,
  onClose
}: DeleteAttachmentModalProps) => {
  const { t } = useTranslation()
  const { mutateAsync: deleteAttachment, isPending } = useDeleteAttachment(
    auditId,
    ticketId
  )

  const handleDelete = async () => {
    try {
      await deleteAttachment(attachmentId)
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
      title={t('kanban:attachment.delete')}
      confirmText={t('common:delete')}
      isLoading={isPending}
      variant='danger'
      description={t('kanban:attachment.deleteConfirm')}
      confirmMatch={t('kanban:attachment.confirmDelete')}
    />
  )
}

export default AttachmentSection
