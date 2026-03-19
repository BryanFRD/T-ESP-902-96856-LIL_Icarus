import { render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import RefreshLoader from '@/components/loader/refresh-loader.tsx'

vi.mock('@faker-js/faker', () => ({
  faker: {
    seed: vi.fn(),
    hacker: {
      phrase: vi.fn(() => 'Compiling the circuit')
    }
  }
}))

vi.mock('@/components/icon/logo.tsx', () => ({
  default: ({ fill, className }: { fill?: string; className?: string }) => (
    <div data-testid='logo-icon' data-fill={fill} className={className} />
  )
}))

describe('RefreshLoader components', () => {
  it('renders the loader with spinning animation', () => {
    render(<RefreshLoader />)

    const container = screen.getByText('Compiling the circuit').parentElement
    expect(container).toBeInTheDocument()
    expect(container?.className).toContain('fixed')
    expect(container?.className).toContain('flex')
  })

  it('displays a random hacker phrase from faker', () => {
    render(<RefreshLoader />)

    expect(screen.getByText('Compiling the circuit')).toBeInTheDocument()
  })

  it('renders the Logo components with correct props', () => {
    render(<RefreshLoader />)

    const logo = screen.getByTestId('logo-icon')
    expect(logo).toBeInTheDocument()
    expect(logo).toHaveAttribute('data-fill', '#FF6900')
    expect(logo.className).toContain('h-8')
    expect(logo.className).toContain('w-8')
  })

  it('has the correct dark mode classes', () => {
    render(<RefreshLoader />)

    const container = screen.getByText('Compiling the circuit').parentElement
    expect(container?.className).toContain('bg-white')
    expect(container?.className).toContain('dark:bg-gray-900')
  })
})
