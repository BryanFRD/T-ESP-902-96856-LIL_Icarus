import { motion } from 'framer-motion'
import { Loader2, Plus, Users } from 'lucide-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import TeamCard from '@/components/team/TeamCard.tsx'
import TeamForm from '@/components/team/TeamForm.tsx'
import Button from '@/components/ui/button/Button'
import Modal from '@/components/ui/modal/Modal'
import { useMyTeams } from '@/hooks/team/useMyTeams.ts'

const TeamsScreen = () => {
  const { t } = useTranslation()
  const { data: teams, isLoading } = useMyTeams()
  const [showCreateModal, setShowCreateModal] = useState(false)

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.4 }}
      className='mx-auto max-w-5xl px-4 py-8'
    >
      <div className='mb-8 flex items-center justify-between'>
        <div>
          <h1 className='text-2xl font-bold text-gray-900 dark:text-white'>
            {t('team.title')}
          </h1>
          <p className='mt-1 text-gray-500 dark:text-gray-400'>
            {t('team.subtitle')}
          </p>
        </div>
        <Button
          onClick={() => setShowCreateModal(true)}
          icon={{ type: Plus, position: 'left' }}
        >
          {t('team.createTeam')}
        </Button>
      </div>

      {isLoading ? (
        <div className='flex items-center justify-center py-12'>
          <Loader2 className='h-8 w-8 animate-spin text-orange-500' />
        </div>
      ) : teams && teams.length > 0 ? (
        <div className='grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3'>
          {teams.map((team, index) => (
            <TeamCard key={team.id} team={team} index={index} />
          ))}
        </div>
      ) : (
        <div className='flex flex-col items-center justify-center py-16 text-center'>
          <div className='mb-4 flex h-20 w-20 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-800'>
            <Users className='h-10 w-10 text-gray-400 dark:text-gray-500' />
          </div>
          <h3 className='mb-2 text-xl font-medium text-gray-900 dark:text-white'>
            {t('team.noTeams')}
          </h3>
          <p className='mb-6 max-w-sm text-gray-500 dark:text-gray-400'>
            {t('team.noTeamsDescription')}
          </p>
          <Button
            onClick={() => setShowCreateModal(true)}
            icon={{ type: Plus, position: 'left' }}
          >
            {t('team.createFirstTeam')}
          </Button>
        </div>
      )}

      <Modal
        isOpen={showCreateModal}
        onClose={() => setShowCreateModal(false)}
        title={t('team.createTeamTitle')}
        size='md'
      >
        <TeamForm
          onSuccess={() => setShowCreateModal(false)}
          onCancel={() => setShowCreateModal(false)}
        />
      </Modal>
    </motion.div>
  )
}

export default TeamsScreen
