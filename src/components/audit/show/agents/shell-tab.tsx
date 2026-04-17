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
import { useAgentShellHistory } from '@/hooks/agent/use-agent-shell-history.ts'

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

  const { sendCommand, cwd, markReplayComplete } = useAgentShell(
    socket,
    auditId,
    agent.id,
    onOutput
  )

  const disabled = !agent.isOnline || agent.isRevoked
  const { data: history, isSuccess: historyLoaded } = useAgentShellHistory(
    auditId,
    agent.id,
    !disabled
  )
  const replayedRef = useRef(false)

  useEffect(() => {
    if (!terminalRef.current) return

    const terminal = new Terminal({
      cursorBlink: false,
      cursorStyle: 'bar',
      cursorInactiveStyle: 'none',
      disableStdin: true,
      convertEol: true,
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

  useEffect(() => {
    if (replayedRef.current) return
    if (!historyLoaded || !history || !xtermRef.current) return
    replayedRef.current = true

    const term = xtermRef.current
    if (history.capped) {
      term.writeln(
        '\x1b[33m[historique tronqué — entrées plus anciennes omises]\x1b[0m'
      )
    }
    for (const entry of history.entries) {
      if (entry.kind === 'command') {
        term.writeln(`\x1b[32m$ ${entry.command}\x1b[0m`)
      } else {
        term.write(entry.data)
        if (entry.truncated) {
          term.writeln(
            `\x1b[0m\x1b[90m[… ${entry.originalBytes ?? '?'} bytes, tronqué à 4 KB]\x1b[0m`
          )
        }
      }
    }
    if (history.entries.length > 0) {
      term.writeln('\x1b[90m─── fin historique ───\x1b[0m')
    }
    markReplayComplete()
  }, [historyLoaded, history, markReplayComplete])

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    if (!command.trim() || !agent.isOnline) return

    xtermRef.current?.writeln(`\x1b[32m$ ${command}\x1b[0m`)
    sendCommand(command)
    setCommand('')
  }

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
      <div className='border-border flex flex-1 flex-col overflow-hidden rounded-md border bg-[#09090b]'>
        <div
          className='text-muted-foreground border-border/50 flex items-center gap-2 border-b px-3 py-1.5 font-mono text-xs'
          dir='rtl'
        >
          <span className='truncate' dir='ltr'>
            {cwd ?? '…'}
          </span>
        </div>
        <div
          ref={terminalRef}
          className='min-h-[400px] flex-1 overflow-hidden bg-[#09090b]'
        />
      </div>
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
