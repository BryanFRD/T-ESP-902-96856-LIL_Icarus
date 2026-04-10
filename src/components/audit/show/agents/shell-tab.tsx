import { FitAddon } from '@xterm/addon-fit'
import { Terminal } from '@xterm/xterm'
import '@xterm/xterm/css/xterm.css'
import { SendIcon } from 'lucide-react'
import { useCallback, useEffect, useRef, useState } from 'react'
import { useTranslation } from 'react-i18next'
import type { Socket } from 'socket.io-client'

import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import type { Agent } from '@/domain/types/agent.ts'
import { useAgentShell } from '@/hooks/agent/use-agent-shell.ts'

interface Props {
  socket: Socket
  agent: Agent
  auditId: string
}

export function ShellTab({ socket, agent, auditId }: Props) {
  const { t } = useTranslation('audit')
  const terminalRef = useRef<HTMLDivElement>(null)
  const xtermRef = useRef<Terminal | null>(null)
  const fitAddonRef = useRef<FitAddon | null>(null)
  const [command, setCommand] = useState('')

  const onOutput = useCallback((data: string) => {
    xtermRef.current?.write(data)
  }, [])

  const { sendCommand } = useAgentShell(socket, auditId, agent.id, onOutput)

  useEffect(() => {
    if (!terminalRef.current) return

    const terminal = new Terminal({
      cursorBlink: false,
      cursorStyle: 'bar',
      cursorInactiveStyle: 'none',
      disableStdin: true,
      fontSize: 13,
      fontFamily: 'monospace',
      theme: {
        background: '#09090b',
        foreground: '#a1a1aa',
        cursor: '#09090b'
      },
      scrollback: 5000
    })

    const fitAddon = new FitAddon()
    terminal.loadAddon(fitAddon)
    terminal.open(terminalRef.current)
    fitAddon.fit()

    xtermRef.current = terminal
    fitAddonRef.current = fitAddon

    const resizeObserver = new ResizeObserver(() => {
      fitAddon.fit()
    })
    resizeObserver.observe(terminalRef.current)

    return () => {
      resizeObserver.disconnect()
      terminal.dispose()
      xtermRef.current = null
      fitAddonRef.current = null
    }
  }, [])

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    if (!command.trim() || !agent.isOnline) return

    xtermRef.current?.writeln(`\x1b[32m$ ${command}\x1b[0m`)
    sendCommand(command)
    setCommand('')
  }

  const disabled = !agent.isOnline || agent.isRevoked

  if (disabled) {
    return (
      <div className='flex h-64 items-center justify-center'>
        <p className='text-muted-foreground text-sm'>
          {t('show.agents.controlPanel.shell.offlineNotice')}
        </p>
      </div>
    )
  }

  return (
    <div className='flex h-full flex-col gap-3'>
      <div
        ref={terminalRef}
        className='min-h-[400px] flex-1 overflow-hidden bg-[#09090b]'
      />
      <form onSubmit={handleSubmit} className='flex gap-2'>
        <Input
          value={command}
          onChange={e => setCommand(e.target.value)}
          placeholder={t('show.agents.controlPanel.shell.placeholder')}
          className='font-mono text-sm'
          autoComplete='off'
          spellCheck={false}
        />
        <Button type='submit' size='icon' disabled={!command.trim()}>
          <SendIcon className='size-4' />
        </Button>
      </form>
    </div>
  )
}
