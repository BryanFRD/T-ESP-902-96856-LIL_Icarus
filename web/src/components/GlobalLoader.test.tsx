import { render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import GlobalLoader from '@/components/GlobalLoader'

vi.mock('@faker-js/faker', () => ({
  faker: {
    seed: vi.fn(),
    hacker: {
      phrase: vi.fn(() => 'Compiling the circuit')
    }
  }
}))

vi.mock('@/components/icon/Logo.tsx', () => ({
  default: ({ fill, className }: { fill?: string; className?: string }) => (
    <div data-testid='logo-icon' data-fill={fill} className={className} />
  )
}))

describe('GlobalLoader component', () => {
  it('renders the loader with spinning animation', () => {
    render(<GlobalLoader />)

    const container = screen.getByText('Compiling the circuit').parentElement
    expect(container).toBeInTheDocument()
    expect(container?.className).toContain('fixed')
    expect(container?.className).toContain('flex')
  })

  it('displays a random hacker phrase from faker', () => {
    render(<GlobalLoader />)

    expect(screen.getByText('Compiling the circuit')).toBeInTheDocument()
  })

  it('renders the Logo component with correct props', () => {
    render(<GlobalLoader />)

    const logo = screen.getByTestId('logo-icon')
    expect(logo).toBeInTheDocument()
    expect(logo).toHaveAttribute('data-fill', '#FF6900')
    expect(logo.className).toContain('h-8')
    expect(logo.className).toContain('w-8')
  })

  it('has the correct dark mode classes', () => {
    render(<GlobalLoader />)

    const container = screen.getByText('Compiling the circuit').parentElement
    expect(container?.className).toContain('bg-white')
    expect(container?.className).toContain('dark:bg-gray-900')
  })
})
