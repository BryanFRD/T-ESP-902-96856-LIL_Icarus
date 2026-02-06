import { motion } from 'framer-motion'
import { AlertCircle, CheckCircle, Loader2 } from 'lucide-react'
import { useEffect } from 'react'
import { useTranslation } from 'react-i18next'
import { useNavigate, useParams } from 'react-router-dom'

import Button from '@/components/ui/button/Button'
import { useJoinTeam } from '@/hooks/team/useJoinTeam.ts'
import { ROUTES, getRoutePath } from '@/router/routesName'

const attemptedTokens = new Set<string>()

const JoinTeamScreen = () => {
  const { t } = useTranslation()
  const navigate = useNavigate()
  const { token } = useParams<{ token: string }>()

  const { mutate: joinTeam, isPending, isSuccess, isError } = useJoinTeam()

  useEffect(() => {
    if (token && !attemptedTokens.has(token)) {
      attemptedTokens.add(token)
      joinTeam(token)
    }
  }, [token, joinTeam])

  const hasInvalidToken = !token

  const handleGoToTeams = () => {
    navigate(getRoutePath(ROUTES.TEAMS))
  }

  const showLoading = isPending || (!isSuccess && !isError && !hasInvalidToken)
  const showError = isError || hasInvalidToken

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.4 }}
      className='mx-auto flex min-h-[60vh] max-w-md items-center justify-center px-4 py-8'
    >
      <div className='w-full rounded-lg border border-gray-200 bg-white p-8 text-center dark:border-gray-700 dark:bg-gray-800'>
        {showLoading && (
          <>
            <div className='mx-auto mb-6 flex h-20 w-20 items-center justify-center rounded-full bg-orange-100 dark:bg-orange-900/30'>
              <Loader2 className='h-10 w-10 animate-spin text-orange-500' />
            </div>
            <h2 className='mb-2 text-xl font-bold text-gray-900 dark:text-white'>
              {t('team:joiningTeam')}
            </h2>
            <p className='text-gray-500 dark:text-gray-400'>
              {t('team:joiningTeamDescription')}
            </p>
          </>
        )}

        {isSuccess && (
          <>
            <div className='mx-auto mb-6 flex h-20 w-20 items-center justify-center rounded-full bg-green-100 dark:bg-green-900/30'>
              <CheckCircle className='h-10 w-10 text-green-500' />
            </div>
            <h2 className='mb-2 text-xl font-bold text-gray-900 dark:text-white'>
              {t('team:joinedTeamSuccess')}
            </h2>
            <p className='mb-6 text-gray-500 dark:text-gray-400'>
              {t('team:joinedTeamSuccessDescription')}
            </p>
            <Button onClick={handleGoToTeams} fullWidth>
              {t('team:backToTeams')}
            </Button>
          </>
        )}

        {showError && (
          <>
            <div className='mx-auto mb-6 flex h-20 w-20 items-center justify-center rounded-full bg-red-100 dark:bg-red-900/30'>
              <AlertCircle className='h-10 w-10 text-red-500' />
            </div>
            <h2 className='mb-2 text-xl font-bold text-gray-900 dark:text-white'>
              {t('team:joinTeamError')}
            </h2>
            <p className='mb-6 text-gray-500 dark:text-gray-400'>
              {hasInvalidToken
                ? t('team:invalidInvitationToken')
                : t('team:joinTeamErrorDescription')}
            </p>
            <Button variant='secondary' onClick={handleGoToTeams} fullWidth>
              {t('team:backToTeams')}
            </Button>
          </>
        )}
      </div>
    </motion.div>
  )
}

export default JoinTeamScreen
