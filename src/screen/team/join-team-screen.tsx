import { AlertCircle, CheckCircle } from 'lucide-react'
import { useEffect } from 'react'
import { useTranslation } from 'react-i18next'
import { useNavigate, useParams } from 'react-router-dom'

import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Spinner } from '@/components/ui/spinner'
import { useJoinTeam } from '@/hooks/team/use-join-team.ts'
import { ROUTES, getRoutePath } from '@/router/routes-name.ts'

const attemptedTokens = new Set<string>()

const JoinTeamScreen = () => {
  const { t } = useTranslation('team')
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
    <div className='mx-auto flex min-h-[60vh] max-w-md items-center justify-center px-4 py-8'>
      <Card className='w-full'>
        <CardContent className='flex flex-col items-center py-10 text-center'>
          {showLoading && (
            <>
              <div className='bg-primary/10 mb-6 flex size-20 items-center justify-center rounded-full'>
                <Spinner className='text-primary size-10' />
              </div>
              <h2 className='mb-2 text-xl font-semibold'>
                {t('joinTeam.loading.title')}
              </h2>
              <p className='text-muted-foreground text-sm'>
                {t('joinTeam.loading.description')}
              </p>
            </>
          )}

          {isSuccess && (
            <>
              <div className='mb-6 flex size-20 items-center justify-center rounded-full bg-green-500/10'>
                <CheckCircle className='size-10 text-green-500' />
              </div>
              <h2 className='mb-2 text-xl font-semibold'>
                {t('joinTeam.success.title')}
              </h2>
              <p className='text-muted-foreground mb-6 text-sm'>
                {t('joinTeam.success.description')}
              </p>
              <Button className='w-full' onClick={handleGoToTeams}>
                {t('joinTeam.backToTeams')}
              </Button>
            </>
          )}

          {showError && (
            <>
              <div className='bg-destructive/10 mb-6 flex size-20 items-center justify-center rounded-full'>
                <AlertCircle className='text-destructive size-10' />
              </div>
              <h2 className='mb-2 text-xl font-semibold'>
                {t('joinTeam.error.title')}
              </h2>
              <p className='text-muted-foreground mb-6 text-sm'>
                {hasInvalidToken
                  ? t('joinTeam.error.invalidToken')
                  : t('joinTeam.error.description')}
              </p>
              <Button
                variant='outline'
                className='w-full'
                onClick={handleGoToTeams}
              >
                {t('joinTeam.backToTeams')}
              </Button>
            </>
          )}
        </CardContent>
      </Card>
    </div>
  )
}

export default JoinTeamScreen
