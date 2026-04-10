import { PlayIcon, SquareIcon, TrashIcon } from 'lucide-react'
import { useEffect } from 'react'
import { useTranslation } from 'react-i18next'
import type { Socket } from 'socket.io-client'

import { Button } from '@/components/ui/button'
import type { Agent } from '@/domain/types/agent.ts'
import { useAgentKeylogger } from '@/hooks/agent/use-agent-keylogger.ts'

interface Props {
  socket: Socket
  agent: Agent
  auditId: string
}

export function KeyloggerTab({ socket, agent, auditId }: Props) {
  const { t } = useTranslation('audit')
  const { isRunning, output, outputRef, start, stop, clearOutput } =
    useAgentKeylogger(socket, auditId, agent.id)

  useEffect(() => {
    if (outputRef.current) {
      outputRef.current.scrollTop = outputRef.current.scrollHeight
    }
  }, [output, outputRef])

  const disabled = !agent.isOnline || agent.isRevoked

  if (disabled) {
    return (
      <div className='flex h-64 items-center justify-center'>
        <p className='text-muted-foreground text-sm'>
          {t('show.agents.controlPanel.keylogger.offlineNotice')}
        </p>
      </div>
    )
  }

  return (
    <div className='flex h-full flex-col gap-3'>
      <div className='flex items-center gap-2'>
        {isRunning ? (
          <Button variant='destructive' size='sm' onClick={stop}>
            <SquareIcon className='size-4' />
            {t('show.agents.controlPanel.keylogger.stop')}
          </Button>
        ) : (
          <Button size='sm' onClick={start}>
            <PlayIcon className='size-4' />
            {t('show.agents.controlPanel.keylogger.start')}
          </Button>
        )}
        {output && (
          <Button variant='ghost' size='sm' onClick={clearOutput}>
            <TrashIcon className='size-4' />
          </Button>
        )}
      </div>

      <pre
        ref={outputRef}
        className='min-h-[400px] flex-1 overflow-auto rounded-lg bg-zinc-950 p-4 font-mono text-xs text-green-400'
      >
        {output || (
          <span className='text-zinc-600'>
            {t('show.agents.controlPanel.keylogger.empty')}
          </span>
        )}
      </pre>
    </div>
  )
}
