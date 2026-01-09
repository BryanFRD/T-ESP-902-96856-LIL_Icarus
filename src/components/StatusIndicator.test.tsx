import { render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import StatusIndicator from '@/components/StatusIndicator'
import { ServerStatusEnum } from '@/domain/enums/serverStatusEnum'

vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string, options?: { status?: string }) =>
      options?.status ? `Status: ${options.status}` : key
  })
}))

vi.mock('@/components/ThreatLevel.tsx', () => ({
  default: ({ level }: { level: string }) => (
    <div data-testid='threat-level'>{level}</div>
  )
}))

describe('StatusIndicator component', () => {
  it('renders nothing when status is undefined', () => {
    const { container } = render(<StatusIndicator status={undefined} />)
    expect(container.firstChild).toBeNull()
  })

  it('renders status when provided', () => {
    render(<StatusIndicator status={ServerStatusEnum.Up} />)
    expect(screen.getByText('Status: Up')).toBeInTheDocument()
  })

  it('renders ThreatLevel component with medium level', () => {
    render(<StatusIndicator status={ServerStatusEnum.Up} />)
    const threatLevel = screen.getByTestId('threat-level')
    expect(threatLevel).toBeInTheDocument()
    expect(threatLevel).toHaveTextContent('medium')
  })

  it('renders Zap icon', () => {
    const { container } = render(
      <StatusIndicator status={ServerStatusEnum.Up} />
    )
    const svg = container.querySelector('svg')
    expect(svg).toBeInTheDocument()
  })

  it('has correct styling classes', () => {
    const { container } = render(
      <StatusIndicator status={ServerStatusEnum.Up} />
    )
    const mainDiv = container.firstChild as HTMLElement
    expect(mainDiv.className).toContain('flex')
    expect(mainDiv.className).toContain('items-center')
    expect(mainDiv.className).toContain('rounded-xl')
  })
})
