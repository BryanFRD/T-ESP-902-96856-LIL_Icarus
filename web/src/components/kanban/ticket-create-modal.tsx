import {
  ChevronDown,
  ChevronUp,
  ChevronsUp,
  Equal,
  Loader2
} from 'lucide-react'
import { Controller, useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'

import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from '@/components/ui/select'
import { Textarea } from '@/components/ui/textarea'
import type { TicketUser } from '@/domain/types/kanban.ts'
import { useCreateTicket } from '@/hooks/kanban/use-ticket-mutation.ts'

type TicketFormData = {
  title: string
  columnId: string
  description?: string | null
  assigneeId?: string | null
  priority?: 'LOW' | 'MEDIUM' | 'HIGH' | 'CRITICAL' | null
  dueDate?: string | null
  parentId?: string | null
}

type TicketCreateModalProps = {
  isOpen: boolean
  onClose: () => void
  auditId: string
  columnId: string
  availableUsers?: TicketUser[]
}

const PRIORITY_CONFIG = {
  CRITICAL: {
    icon: ChevronsUp,
    color: 'text-red-500'
  },
  HIGH: {
    icon: ChevronUp,
    color: 'text-red-500'
  },
  MEDIUM: {
    icon: Equal,
    color: 'text-orange-500'
  },
  LOW: {
    icon: ChevronDown,
    color: 'text-blue-500'
  }
} as const

const PRIORITIES = ['CRITICAL', 'HIGH', 'MEDIUM', 'LOW'] as const

function PriorityOption({
  priority
}: {
  priority: (typeof PRIORITIES)[number]
}) {
  const { t } = useTranslation()
  const config = PRIORITY_CONFIG[priority]
  const Icon = config.icon

  return (
    <span className={`flex items-center gap-2 ${config.color}`}>
      <Icon className='h-4 w-4' strokeWidth={2.5} />
      <span>{t(`kanban:priority.${priority.toLowerCase()}`)}</span>
    </span>
  )
}

function UserOption({ user }: { user: TicketUser }) {
  return (
    <span className='flex items-center gap-2'>
      <Avatar className='h-5 w-5'>
        <AvatarFallback className='bg-primary/10 text-primary text-[9px]'>
          {user.firstName[0]}
          {user.lastName[0]}
        </AvatarFallback>
      </Avatar>
      <span>
        {user.firstName} {user.lastName}
      </span>
    </span>
  )
}

const TicketCreateModal = ({
  isOpen,
  onClose,
  auditId,
  columnId,
  availableUsers = []
}: TicketCreateModalProps) => {
  const { t } = useTranslation()
  const { mutateAsync: createTicket, isPending } = useCreateTicket(auditId)

  const {
    register,
    handleSubmit,
    control,
    reset,
    formState: { errors }
  } = useForm<TicketFormData>({
    defaultValues: {
      columnId,
      title: '',
      description: null,
      assigneeId: null,
      priority: 'MEDIUM',
      dueDate: null,
      parentId: null
    }
  })

  const onSubmit = async (data: TicketFormData) => {
    try {
      await createTicket({
        ...data,
        dueDate: data.dueDate || null
      })
      reset()
      onClose()
    } catch {
      // Error handled by mutation
    }
  }

  const handleClose = () => {
    reset()
    onClose()
  }

  return (
    <Dialog open={isOpen} onOpenChange={open => !open && handleClose()}>
      <DialogContent className='sm:max-w-lg'>
        <DialogHeader>
          <DialogTitle>{t('kanban:createTicket')}</DialogTitle>
        </DialogHeader>

        <form onSubmit={handleSubmit(onSubmit)} className='space-y-4'>
          <div className='space-y-2'>
            <Label htmlFor='title'>{t('kanban:ticket.title')}</Label>
            <Input
              id='title'
              placeholder={t('kanban:ticket.titlePlaceholder')}
              {...register('title', { required: true })}
            />
            {errors.title && (
              <p className='text-destructive text-xs'>
                {t('kanban:ticket.titleRequired')}
              </p>
            )}
          </div>

          <div className='space-y-2'>
            <Label htmlFor='description'>
              {t('kanban:ticket.description')}
            </Label>
            <Textarea
              id='description'
              placeholder={t('kanban:ticket.descriptionPlaceholder')}
              rows={3}
              {...register('description')}
            />
          </div>

          <div className='grid grid-cols-2 gap-4'>
            <Controller
              name='priority'
              control={control}
              render={({ field }) => (
                <div className='space-y-2'>
                  <Label>{t('kanban:ticket.priority')}</Label>
                  <Select
                    value={field.value ?? undefined}
                    onValueChange={field.onChange}
                  >
                    <SelectTrigger>
                      <SelectValue
                        placeholder={t('kanban:ticket.priorityPlaceholder')}
                      >
                        {field.value && (
                          <PriorityOption priority={field.value} />
                        )}
                      </SelectValue>
                    </SelectTrigger>
                    <SelectContent>
                      {PRIORITIES.map(p => (
                        <SelectItem key={p} value={p}>
                          <PriorityOption priority={p} />
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
              )}
            />

            <Controller
              name='assigneeId'
              control={control}
              render={({ field }) => (
                <div className='space-y-2'>
                  <Label>{t('kanban:ticket.assignee')}</Label>
                  <Select
                    value={field.value ?? 'none'}
                    onValueChange={v => field.onChange(v === 'none' ? null : v)}
                  >
                    <SelectTrigger>
                      <SelectValue
                        placeholder={t('kanban:ticket.assigneePlaceholder')}
                      >
                        {field.value
                          ? (() => {
                              const user = availableUsers.find(
                                u => u.id === field.value
                              )
                              return user ? <UserOption user={user} /> : null
                            })()
                          : t('kanban:ticket.unassigned')}
                      </SelectValue>
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value='none'>
                        <span className='text-muted-foreground'>
                          {t('kanban:ticket.unassigned')}
                        </span>
                      </SelectItem>
                      {availableUsers.map(user => (
                        <SelectItem key={user.id} value={user.id}>
                          <UserOption user={user} />
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
              )}
            />
          </div>

          <Controller
            name='dueDate'
            control={control}
            render={({ field }) => (
              <div className='space-y-2'>
                <Label htmlFor='dueDate'>{t('kanban:ticket.dueDate')}</Label>
                <Input
                  id='dueDate'
                  type='date'
                  value={field.value ?? ''}
                  onChange={e => field.onChange(e.target.value || null)}
                />
              </div>
            )}
          />

          <div className='flex justify-end gap-3 pt-2'>
            <Button
              type='button'
              variant='outline'
              onClick={handleClose}
              disabled={isPending}
            >
              {t('common:actions.cancel')}
            </Button>
            <Button type='submit' disabled={isPending}>
              {isPending && <Loader2 className='mr-2 h-4 w-4 animate-spin' />}
              {t('kanban:createTicket')}
            </Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  )
}

export default TicketCreateModal
