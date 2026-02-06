import { motion } from 'framer-motion'
import {
  ArrowLeft,
  Loader2,
  MoreVertical,
  Pencil,
  Plus,
  Trash2,
  Users
} from 'lucide-react'
import { useState } from 'react'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'
import { Link, useNavigate, useParams } from 'react-router-dom'

import TeamForm from '@/components/team/TeamForm.tsx'
import TeamTabs, { type TeamTab } from '@/components/team/TeamTabs.tsx'
import CompaniesList from '@/components/team/companies/CompaniesList.tsx'
import CompanyForm from '@/components/team/companies/CompanyForm.tsx'
import InvitationForm from '@/components/team/invitations/InvitationForm.tsx'
import InvitationLinkCopy from '@/components/team/invitations/InvitationLinkCopy.tsx'
import InvitationsList from '@/components/team/invitations/InvitationsList.tsx'
import MemberRoleModal from '@/components/team/members/MemberRoleModal.tsx'
import MembersList from '@/components/team/members/MembersList.tsx'
import Button from '@/components/ui/button/Button'
import Modal from '@/components/ui/modal/Modal'
import { useUserContext } from '@/context/UserContext.tsx'
import { MemberRole } from '@/domain/enum/MemberRole.ts'
import type { Company, Invitation, TeamMember } from '@/domain/types/Team.ts'
import useInteractionOutside from '@/hooks/common/useInteractionOutside.ts'
import { useCancelInvitation } from '@/hooks/team/useCancelInvitation.ts'
import { useDeleteCompany } from '@/hooks/team/useDeleteCompany.ts'
import { useDeleteTeam } from '@/hooks/team/useDeleteTeam.ts'
import { useMyTeams } from '@/hooks/team/useMyTeams.ts'
import { useRemoveMember } from '@/hooks/team/useRemoveMember.ts'
import { useTeamCompanies } from '@/hooks/team/useTeamCompanies.ts'
import { useTeamInvitations } from '@/hooks/team/useTeamInvitations.ts'
import { ROUTES, getRoutePath } from '@/router/routesName'

const TeamDetailScreen = () => {
  const { t } = useTranslation()
  const navigate = useNavigate()
  const { teamId } = useParams<{ teamId: string }>()
  const { user } = useUserContext()

  const { data: teams, isLoading: isLoadingTeams } = useMyTeams()
  const { data: companies, isLoading: isLoadingCompanies } = useTeamCompanies(
    teamId || ''
  )
  const { data: invitations, isLoading: isLoadingInvitations } =
    useTeamInvitations(teamId || '')

  const { mutate: deleteTeam, isPending: isDeletingTeam } = useDeleteTeam()
  const { mutate: removeMember } = useRemoveMember()
  const { mutate: deleteCompany } = useDeleteCompany()
  const { mutate: cancelInvitation } = useCancelInvitation()

  const [activeTab, setActiveTab] = useState<TeamTab>('members')
  const [showMenu, setShowMenu] = useState(false)
  const [showEditModal, setShowEditModal] = useState(false)
  const [showDeleteModal, setShowDeleteModal] = useState(false)
  const [showCompanyModal, setShowCompanyModal] = useState(false)
  const [showInvitationModal, setShowInvitationModal] = useState(false)
  const [showRoleModal, setShowRoleModal] = useState(false)
  const [selectedMember, setSelectedMember] = useState<TeamMember | null>(null)
  const [editingCompany, setEditingCompany] = useState<Company | null>(null)
  const [createdInvitation, setCreatedInvitation] = useState<Invitation | null>(
    null
  )

  const { ref: menuRef } = useInteractionOutside<HTMLDivElement>(
    () => setShowMenu(false),
    { enabled: showMenu, eventType: 'click' }
  )

  const team = teams?.find(t => t.id === teamId)

  const currentUserMember = team?.members?.find(m => m.id === user?.id)
  const canManage =
    currentUserMember?.teamRole === MemberRole.OWNER ||
    currentUserMember?.teamRole === MemberRole.MODERATOR

  const handleDeleteTeam = () => {
    if (!teamId) return

    toast.promise(
      new Promise<void>((resolve, reject) => {
        deleteTeam(teamId, {
          onSuccess: () => {
            navigate(getRoutePath(ROUTES.TEAMS))
            resolve()
          },
          onError: error => reject(error)
        })
      }),
      {
        loading: t('messages:loading.deletingTeam'),
        success: t('messages:success.teamDeleted'),
        error: t('messages:error.teamDeletionFailed')
      }
    )
  }

  const handleRemoveMember = (member: TeamMember) => {
    if (!teamId) return

    toast.promise(
      new Promise<void>((resolve, reject) => {
        removeMember(
          { teamId, memberId: member.id },
          {
            onSuccess: () => resolve(),
            onError: error => reject(error)
          }
        )
      }),
      {
        loading: t('messages:loading.removingMember'),
        success: t('messages:success.memberRemoved'),
        error: t('messages:error.memberRemovalFailed')
      }
    )
  }

  const handleChangeRole = (member: TeamMember) => {
    setSelectedMember(member)
    setShowRoleModal(true)
  }

  const handleEditCompany = (company: Company) => {
    setEditingCompany(company)
    setShowCompanyModal(true)
  }

  const handleDeleteCompany = (company: Company) => {
    if (!teamId) return

    toast.promise(
      new Promise<void>((resolve, reject) => {
        deleteCompany(
          { teamId, companyId: company.id },
          {
            onSuccess: () => resolve(),
            onError: error => reject(error)
          }
        )
      }),
      {
        loading: t('messages:loading.deletingCompany'),
        success: t('messages:success.companyDeleted'),
        error: t('messages:error.companyDeletionFailed')
      }
    )
  }

  const handleCancelInvitation = (invitation: Invitation) => {
    if (!teamId) return

    toast.promise(
      new Promise<void>((resolve, reject) => {
        cancelInvitation(
          { teamId, invitationId: invitation.id },
          {
            onSuccess: () => resolve(),
            onError: error => reject(error)
          }
        )
      }),
      {
        loading: t('messages:loading.cancellingInvitation'),
        success: t('messages:success.invitationCancelled'),
        error: t('messages:error.invitationCancellationFailed')
      }
    )
  }

  if (isLoadingTeams) {
    return (
      <div className='flex items-center justify-center py-24'>
        <Loader2 className='h-8 w-8 animate-spin text-orange-500' />
      </div>
    )
  }

  if (!team) {
    return (
      <div className='mx-auto max-w-5xl px-4 py-8'>
        <div className='flex flex-col items-center justify-center py-16 text-center'>
          <div className='mb-4 flex h-20 w-20 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-800'>
            <Users className='h-10 w-10 text-gray-400 dark:text-gray-500' />
          </div>
          <h3 className='mb-2 text-xl font-medium text-gray-900 dark:text-white'>
            {t('team:teamNotFound')}
          </h3>
          <p className='mb-6 text-gray-500 dark:text-gray-400'>
            {t('team:teamNotFoundDescription')}
          </p>
          <Button as='link' to={getRoutePath(ROUTES.TEAMS)}>
            {t('team:backToTeams')}
          </Button>
        </div>
      </div>
    )
  }

  const renderTabContent = () => {
    switch (activeTab) {
      case 'members':
        return (
          <>
            <MembersList
              members={team.members || []}
              canManage={canManage}
              onChangeRole={handleChangeRole}
              onRemove={handleRemoveMember}
            />
          </>
        )
      case 'companies':
        if (isLoadingCompanies) {
          return (
            <div className='flex items-center justify-center py-12'>
              <Loader2 className='h-6 w-6 animate-spin text-orange-500' />
            </div>
          )
        }
        return (
          <CompaniesList
            companies={companies || []}
            canManage={canManage}
            onEdit={handleEditCompany}
            onDelete={handleDeleteCompany}
          />
        )
      case 'invitations':
        if (isLoadingInvitations) {
          return (
            <div className='flex items-center justify-center py-12'>
              <Loader2 className='h-6 w-6 animate-spin text-orange-500' />
            </div>
          )
        }
        return (
          <InvitationsList
            invitations={invitations || []}
            canManage={canManage}
            onCancel={handleCancelInvitation}
          />
        )
      default:
        return null
    }
  }

  const renderAddButton = () => {
    if (!canManage) return null

    switch (activeTab) {
      case 'companies':
        return (
          <Button
            size='sm'
            onClick={() => {
              setEditingCompany(null)
              setShowCompanyModal(true)
            }}
            icon={{ type: Plus, position: 'left' }}
          >
            {t('team:addCompany')}
          </Button>
        )
      case 'invitations':
        return (
          <Button
            size='sm'
            onClick={() => {
              setCreatedInvitation(null)
              setShowInvitationModal(true)
            }}
            icon={{ type: Plus, position: 'left' }}
          >
            {t('team:createInvitation')}
          </Button>
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
      className='mx-auto max-w-5xl px-4 py-8'
    >
      <Link
        to={getRoutePath(ROUTES.TEAMS)}
        className='mb-6 inline-flex items-center gap-2 text-sm text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
      >
        <ArrowLeft className='h-4 w-4' />
        {t('team:backToTeams')}
      </Link>

      <div className='mb-8 rounded-lg border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-800'>
        <div className='flex items-start justify-between'>
          <div className='flex items-center gap-4'>
            <div className='flex h-14 w-14 items-center justify-center rounded-lg bg-gradient-to-r from-red-500 to-orange-500'>
              <Users className='h-7 w-7 text-white' />
            </div>
            <div>
              <h1 className='text-xl font-bold text-gray-900 dark:text-white'>
                {team.name}
              </h1>
              {team.description && (
                <p className='mt-1 text-gray-500 dark:text-gray-400'>
                  {team.description}
                </p>
              )}
            </div>
          </div>

          {canManage && (
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
                      setShowEditModal(true)
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
                      setShowDeleteModal(true)
                    }}
                    className='flex w-full items-center gap-2 px-4 py-2 text-left text-sm text-red-600 hover:bg-gray-100 dark:text-red-400 dark:hover:bg-gray-700'
                  >
                    <Trash2 className='h-4 w-4' />
                    {t('common:actions.delete')}
                  </button>
                </div>
              )}
            </div>
          )}
        </div>
      </div>

      <div className='rounded-lg border border-gray-200 bg-white dark:border-gray-700 dark:bg-gray-800'>
        <div className='flex items-center justify-between px-6 pt-4'>
          <TeamTabs activeTab={activeTab} onTabChange={setActiveTab} />
          {renderAddButton()}
        </div>
        <div className='p-6'>{renderTabContent()}</div>
      </div>

      {/* Edit Team Modal */}
      <Modal
        isOpen={showEditModal}
        onClose={() => setShowEditModal(false)}
        title={t('team:editTeam')}
        size='md'
      >
        <TeamForm
          team={team}
          onSuccess={() => setShowEditModal(false)}
          onCancel={() => setShowEditModal(false)}
        />
      </Modal>

      {/* Delete Team Modal */}
      <Modal
        isOpen={showDeleteModal}
        onClose={() => setShowDeleteModal(false)}
        title={t('team:deleteTeam')}
        size='sm'
        footer={
          <div className='flex justify-end gap-3'>
            <Button
              type='button'
              variant='secondary'
              onClick={() => setShowDeleteModal(false)}
            >
              {t('common:actions.cancel')}
            </Button>
            <Button
              type='button'
              variant='danger'
              onClick={handleDeleteTeam}
              isLoading={isDeletingTeam}
            >
              {t('common:actions.delete')}
            </Button>
          </div>
        }
      >
        <p className='text-gray-600 dark:text-gray-400'>
          {t('team:deleteTeamConfirmation', { name: team.name })}
        </p>
      </Modal>

      {/* Company Modal */}
      <Modal
        isOpen={showCompanyModal}
        onClose={() => {
          setShowCompanyModal(false)
          setEditingCompany(null)
        }}
        title={
          editingCompany ? t('team:editCompany') : t('team:addCompanyTitle')
        }
        size='md'
      >
        <CompanyForm
          teamId={teamId || ''}
          company={editingCompany || undefined}
          onSuccess={() => {
            setShowCompanyModal(false)
            setEditingCompany(null)
          }}
          onCancel={() => {
            setShowCompanyModal(false)
            setEditingCompany(null)
          }}
        />
      </Modal>

      {/* Invitation Modal */}
      <Modal
        isOpen={showInvitationModal}
        onClose={() => {
          setShowInvitationModal(false)
          setCreatedInvitation(null)
        }}
        title={t('team:createInvitationTitle')}
        size='md'
      >
        {createdInvitation ? (
          <div className='space-y-4'>
            <p className='text-gray-600 dark:text-gray-400'>
              {t('team:invitationCreatedSuccess')}
            </p>
            <InvitationLinkCopy invitation={createdInvitation} />
            <div className='flex justify-end pt-4'>
              <Button
                type='button'
                onClick={() => {
                  setShowInvitationModal(false)
                  setCreatedInvitation(null)
                }}
              >
                {t('common:actions.done')}
              </Button>
            </div>
          </div>
        ) : (
          <InvitationForm
            teamId={teamId || ''}
            onSuccess={invitation => {
              setCreatedInvitation(invitation)
            }}
            onCancel={() => setShowInvitationModal(false)}
          />
        )}
      </Modal>

      {/* Member Role Modal */}
      <MemberRoleModal
        isOpen={showRoleModal}
        onClose={() => {
          setShowRoleModal(false)
          setSelectedMember(null)
        }}
        teamId={teamId || ''}
        member={selectedMember}
      />
    </motion.div>
  )
}

export default TeamDetailScreen
