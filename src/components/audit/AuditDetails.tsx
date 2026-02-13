import { format } from 'date-fns'
import { fr } from 'date-fns/locale'
import { motion } from 'framer-motion'
import {
  ArrowLeft,
  Building2,
  Calendar,
  KanbanIcon,
  Mail,
  MoreVertical,
  Pencil,
  Plus,
  Trash2,
  Users
} from 'lucide-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { Link, useLocation } from 'react-router-dom'
import { twMerge } from 'tailwind-merge'

import KanbanBoard from '@/components/kanban/KanbanBoard.tsx'
import Button from '@/components/ui/button/Button'
import type { UserAudit } from '@/domain/types/Audit.ts'
import useInteractionOutside from '@/hooks/common/useInteractionOutside.ts'
import { ROUTES, getRoutePath } from '@/router/routesName'

type AuditTab = 'kanban' | 'clients' | 'invitations'

const AuditDetails = () => {
  const { t } = useTranslation()
  const location = useLocation()
  const audit = location.state?.audit as UserAudit

  const [showMenu, setShowMenu] = useState(false)
  const [activeTab, setActiveTab] = useState<AuditTab>('kanban')

  const { ref: menuRef } = useInteractionOutside<HTMLDivElement>(
    () => setShowMenu(false),
    { enabled: showMenu, eventType: 'click' }
  )

  if (!audit) {
    return (
      <div className='mx-auto max-w-7xl px-4 py-8'>
        <div className='flex flex-col items-center justify-center py-16 text-center'>
          <div className='mb-4 flex h-20 w-20 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-800'>
            <Building2 className='h-10 w-10 text-gray-400 dark:text-gray-500' />
          </div>
          <h3 className='mb-2 text-xl font-medium text-gray-900 dark:text-white'>
            {t('audit:details.notFound')}
          </h3>
          <p className='mb-6 text-gray-500 dark:text-gray-400'>
            {t('audit:details.notFoundDescription')}
          </p>
          <Button as='link' to={getRoutePath(ROUTES.AUDITS)}>
            {t('audit:backToAudits')}
          </Button>
        </div>
      </div>
    )
  }

  const tabs = [
    { id: 'kanban' as const, label: t('audit:tabs.kanban'), icon: KanbanIcon },
    { id: 'clients' as const, label: t('audit:tabs.clients'), icon: Users },
    {
      id: 'invitations' as const,
      label: t('audit:tabs.invitations'),
      icon: Mail
    }
  ]

  const renderTabContent = () => {
    switch (activeTab) {
      case 'kanban':
        return <KanbanBoard auditId={audit.id} teamId={audit.teamId} />
      case 'clients':
        return (
          <div className='space-y-3'>
            {audit.clients && audit.clients.length > 0 ? (
              audit.clients.map(client => (
                <div
                  key={client.id}
                  className='flex items-center justify-between rounded-lg border border-gray-200 bg-white p-4 dark:border-gray-700 dark:bg-gray-800'
                >
                  <div className='flex items-center gap-4'>
                    <img
                      src={
                        client.avatarUrl ||
                        `https://ui-avatars.com/api/?name=${client.firstName}+${client.lastName}`
                      }
                      alt={client.username}
                      className='h-10 w-10 rounded-full'
                    />
                    <div>
                      <p className='font-medium text-gray-900 dark:text-white'>
                        {client.firstName} {client.lastName}
                      </p>
                      <p className='text-sm text-gray-500 dark:text-gray-400'>
                        {client.email}
                      </p>
                    </div>
                  </div>
                </div>
              ))
            ) : (
              <div className='flex flex-col items-center justify-center py-12 text-center'>
                <Users className='mb-3 h-12 w-12 text-gray-400' />
                <p className='text-sm text-gray-500 dark:text-gray-400'>
                  {t('audit:noClients')}
                </p>
              </div>
            )}
          </div>
        )
      case 'invitations':
        return (
          <div className='space-y-3'>
            <div className='flex justify-end'>
              <Button
                size='sm'
                onClick={() => {}}
                icon={{ type: Plus, position: 'left' }}
              >
                {t('audit:createInvitation')}
              </Button>
            </div>
            <div className='flex flex-col items-center justify-center py-12 text-center'>
              <Mail className='mb-3 h-12 w-12 text-gray-400' />
              <p className='text-sm text-gray-500 dark:text-gray-400'>
                {t('audit:noInvitations')}
              </p>
            </div>
          </div>
        )
      default:
        return null
    }
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.4 }}
      className='mx-auto flex h-full max-w-7xl flex-col px-4 py-8'
    >
      <div className='mb-6 flex items-center justify-between'>
        <Link
          to={getRoutePath(ROUTES.AUDITS)}
          className='inline-flex items-center gap-2 text-sm text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
        >
          <ArrowLeft className='h-4 w-4' />
          {t('audit:backToAudits')}
        </Link>

        <div className='relative' ref={menuRef}>
          <Button
            variant='ghost'
            size='sm'
            onClick={() => setShowMenu(!showMenu)}
            aria-label={t('common:actions.more')}
          >
            <MoreVertical className='h-5 w-5' />
          </Button>

          {showMenu && (
            <div className='absolute right-0 z-10 mt-2 w-48 rounded-lg border border-gray-200 bg-white py-1 shadow-lg dark:border-gray-700 dark:bg-gray-800'>
              <button
                type='button'
                onClick={() => {
                  setShowMenu(false)
                }}
                className='flex w-full items-center gap-2 px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100 dark:text-gray-200 dark:hover:bg-gray-700'
              >
                <Pencil className='h-4 w-4' />
                {t('common:actions.edit')}
              </button>
              <button
                type='button'
                onClick={() => {
                  setShowMenu(false)
                }}
                className='flex w-full items-center gap-2 px-4 py-2 text-left text-sm text-red-600 hover:bg-gray-100 dark:text-red-400 dark:hover:bg-gray-700'
              >
                <Trash2 className='h-4 w-4' />
                {t('common:actions.delete')}
              </button>
            </div>
          )}
        </div>
      </div>

      <div className='mb-6'>
        <div className='flex items-center gap-4'>
          <div className='flex h-12 w-12 items-center justify-center rounded-lg bg-orange-50 text-orange-500 dark:bg-orange-900/20 dark:text-orange-500'>
            <Building2 className='h-6 w-6' />
          </div>
          <div>
            <div className='flex items-center gap-3'>
              <h1 className='text-2xl font-bold text-gray-900 dark:text-white'>
                {audit.name}
              </h1>
              <span
                className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ${
                  audit.isArchived
                    ? 'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-300'
                    : 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-300'
                }`}
              >
                {audit.isArchived
                  ? t('audit:status.archived')
                  : t('audit:status.active')}
              </span>
            </div>
            <div className='mt-1 flex items-center gap-4 text-sm text-gray-500 dark:text-gray-400'>
              <span className='flex items-center gap-1.5'>
                <Building2 size={14} />
                {audit.company.name}
              </span>
              <span className='flex items-center gap-1.5'>
                <Calendar size={14} />
                {format(new Date(audit.endDate), 'dd MMMM yyyy', {
                  locale: fr
                })}
              </span>
            </div>
          </div>
        </div>
      </div>

      <div className='border-b border-gray-200 dark:border-gray-700'>
        <nav className='-mb-px flex space-x-8'>
          {tabs.map(tab => {
            const Icon = tab.icon
            const isActive = activeTab === tab.id

            return (
              <button
                key={tab.id}
                type='button'
                onClick={() => setActiveTab(tab.id)}
                className={twMerge(
                  'flex items-center gap-2 border-b-2 px-1 py-4 text-sm font-medium transition-colors',
                  isActive
                    ? 'border-orange-600 text-orange-700 dark:border-orange-500 dark:text-orange-500'
                    : 'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 dark:text-gray-400 dark:hover:border-gray-600 dark:hover:text-gray-300'
                )}
              >
                <Icon className='size-4' />
                {tab.label}
              </button>
            )
          })}
        </nav>
      </div>

      <div className='flex-1 overflow-hidden pt-6'>{renderTabContent()}</div>
    </motion.div>
  )
}

export default AuditDetails
