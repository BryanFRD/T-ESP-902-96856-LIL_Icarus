import { SendIcon } from 'lucide-react'
import { useEffect, useRef, useState } from 'react'
import { useTranslation } from 'react-i18next'

import { Button } from '@/components/ui/button'
import type { AuditMessage } from '@/domain/types/audit.ts'
import { useAuditMessages } from '@/hooks/audit/use-audit-messages.ts'
import { useMe } from '@/hooks/auth/use-me.ts'

function formatTime(date: Date): string {
  return date.toLocaleTimeString('fr-FR', {
    hour: '2-digit',
    minute: '2-digit'
  })
}

function isWithin5Min(a: Date, b: Date): boolean {
  return Math.abs(a.getTime() - b.getTime()) <= 5 * 60_000
}

function getInitials(firstName: string, lastName: string) {
  return `${firstName[0] ?? ''}${lastName[0] ?? ''}`.toUpperCase()
}

function MessageBubble({
  message,
  isCurrentUser,
  showTimestamp,
  isGroupedWithPrev
}: {
  message: AuditMessage
  isCurrentUser: boolean
  showTimestamp: boolean
  isGroupedWithPrev: boolean
}) {
  const sentAt = new Date(message.createdAt)
  const authorName = `${message.author.firstName} ${message.author.lastName}`

  if (isCurrentUser) {
    return (
      <div
        className={`flex flex-col items-end gap-1 ${isGroupedWithPrev ? 'mt-0.5' : 'mt-4'}`}
      >
        <div className='bg-primary text-primary-foreground max-w-[70%] rounded-2xl rounded-tr-sm px-4 py-2.5'>
          <p className='text-sm leading-relaxed'>{message.content}</p>
        </div>
        {showTimestamp && (
          <span className='text-muted-foreground px-1 text-xs'>
            {formatTime(sentAt)}
          </span>
        )}
      </div>
    )
  }

  return (
    <div
      className={`flex items-end gap-2.5 ${isGroupedWithPrev ? 'mt-0.5' : 'mt-4'}`}
    >
      <div
        className={`size-8 shrink-0 ${isGroupedWithPrev ? 'invisible' : 'bg-muted text-muted-foreground flex items-center justify-center rounded-full text-xs font-semibold'}`}
      >
        {!isGroupedWithPrev &&
          getInitials(message.author.firstName, message.author.lastName)}
      </div>
      <div className='flex max-w-[70%] flex-col gap-1'>
        {!isGroupedWithPrev && (
          <span className='text-muted-foreground px-1 text-xs font-medium'>
            {authorName}
          </span>
        )}
        <div className='bg-muted rounded-2xl rounded-tl-sm px-4 py-2.5'>
          <p className='text-sm leading-relaxed'>{message.content}</p>
        </div>
        {showTimestamp && (
          <span className='text-muted-foreground px-1 text-xs'>
            {formatTime(sentAt)}
          </span>
        )}
      </div>
    </div>
  )
}

interface AuditMessagesContentProps {
  auditId: string
}

export function AuditMessagesContent({ auditId }: AuditMessagesContentProps) {
  const { t } = useTranslation('audit')
  const { data: me } = useMe()
  const { messages, isLoading, isSending, send } = useAuditMessages(auditId)
  const [input, setInput] = useState('')
  const inputRef = useRef<HTMLInputElement>(null)
  const bottomRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [messages])

  async function handleSend() {
    const trimmed = input.trim()
    if (!trimmed) return
    setInput('')
    inputRef.current?.focus()
    await send(trimmed)
  }

  function handleKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      void handleSend()
    }
  }

  function isGroupedWithPrev(messages: AuditMessage[], i: number): boolean {
    const prev = messages[i - 1]
    const curr = messages[i]
    if (!prev) return false
    return (
      prev.author.id === curr.author.id &&
      isWithin5Min(new Date(prev.createdAt), new Date(curr.createdAt))
    )
  }

  function showTimestamp(messages: AuditMessage[], i: number): boolean {
    const next = messages[i + 1]
    const curr = messages[i]
    if (!next) return true
    return (
      next.author.id !== curr.author.id ||
      !isWithin5Min(new Date(curr.createdAt), new Date(next.createdAt))
    )
  }

  return (
    <div className='ring-foreground/10 bg-card flex h-120 flex-col overflow-hidden rounded-2xl ring-1'>
      <div className='flex flex-1 flex-col overflow-y-auto p-5'>
        {isLoading ? (
          <div className='text-muted-foreground flex flex-1 items-center justify-center text-sm'>
            {t('show.messages.loading')}
          </div>
        ) : messages.length === 0 ? (
          <div className='text-muted-foreground flex flex-1 items-center justify-center text-sm'>
            {t('show.messages.empty')}
          </div>
        ) : (
          messages.map((message, i) => (
            <MessageBubble
              key={message.id}
              message={message}
              isCurrentUser={message.author.id === me?.id}
              showTimestamp={showTimestamp(messages, i)}
              isGroupedWithPrev={isGroupedWithPrev(messages, i)}
            />
          ))
        )}
        <div ref={bottomRef} />
      </div>

      <div className='border-foreground/10 border-t p-4'>
        <div className='ring-foreground/10 bg-background flex items-center gap-3 rounded-xl px-4 py-2 ring-1'>
          <input
            ref={inputRef}
            type='text'
            value={input}
            onChange={e => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder={t('show.messages.placeholder')}
            className='placeholder:text-muted-foreground flex-1 bg-transparent text-sm outline-none'
            disabled={isSending}
          />
          <Button
            size='icon'
            variant='ghost'
            onClick={() => void handleSend()}
            disabled={!input.trim() || isSending}
            className='size-8 shrink-0'
          >
            <SendIcon className='size-4' />
          </Button>
        </div>
      </div>
    </div>
  )
}
