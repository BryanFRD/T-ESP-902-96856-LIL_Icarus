import { render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import ThreatLevel from '@/components/ThreatLevel'

vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => key
  })
}))

describe('ThreatLevel component', () => {
  it('renders with low level', () => {
    render(<ThreatLevel level='low' />)
    expect(screen.getByText('threatLevel.low')).toBeInTheDocument()
  })

  it('renders with medium level', () => {
    render(<ThreatLevel level='medium' />)
    expect(screen.getByText('threatLevel.medium')).toBeInTheDocument()
  })

  it('renders with high level', () => {
    render(<ThreatLevel level='high' />)
    expect(screen.getByText('threatLevel.high')).toBeInTheDocument()
  })

  it('renders with critical level', () => {
    render(<ThreatLevel level='critical' />)
    expect(screen.getByText('threatLevel.critical')).toBeInTheDocument()
  })

  it('applies correct color class for low level', () => {
    const { container } = render(<ThreatLevel level='low' />)
    const indicator = container.querySelector('.h-2')
    expect(indicator?.className).toContain('bg-green-500')
  })

  it('applies correct color class for medium level', () => {
    const { container } = render(<ThreatLevel level='medium' />)
    const indicator = container.querySelector('.h-2')
    expect(indicator?.className).toContain('bg-yellow-500')
  })

  it('applies correct color class for high level', () => {
    const { container } = render(<ThreatLevel level='high' />)
    const indicator = container.querySelector('.h-2')
    expect(indicator?.className).toContain('bg-orange-500')
  })

  it('applies correct color class for critical level', () => {
    const { container } = render(<ThreatLevel level='critical' />)
    const indicator = container.querySelector('.h-2')
    expect(indicator?.className).toContain('bg-red-500')
  })

  it('has animate-pulse class on indicator', () => {
    const { container } = render(<ThreatLevel level='low' />)
    const indicator = container.querySelector('.h-2')
    expect(indicator?.className).toContain('animate-pulse')
  })
})
