import { fireEvent, render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import LogoSidebarMobile from '@/components/layout/sidebar/LogoSidebarMobile.tsx'

vi.mock('i18next', () => ({
  t: (key: string) => key
}))

describe('LogoSidebarMobile components', () => {
  it('renders without crashing', () => {
    render(<LogoSidebarMobile onToggle={vi.fn()} isOpen={false} />)
    expect(screen.getByAltText('Icarus Logo')).toBeInTheDocument()
  })

  it('shows Menu icon when sidebar is closed', () => {
    const { container } = render(
      <LogoSidebarMobile onToggle={vi.fn()} isOpen={false} />
    )
    const svg = container.querySelector('svg')
    expect(svg).toBeInTheDocument()
  })

  it('shows X icon when sidebar is open', () => {
    const { container } = render(
      <LogoSidebarMobile onToggle={vi.fn()} isOpen={true} />
    )
    const svg = container.querySelector('svg')
    expect(svg).toBeInTheDocument()
  })

  it('calls onToggle when button is clicked', () => {
    const onToggle = vi.fn()
    render(<LogoSidebarMobile onToggle={onToggle} isOpen={false} />)
    const button = screen.getByRole('button')
    fireEvent.click(button)
    expect(onToggle).toHaveBeenCalledTimes(1)
  })

  it('has correct aria-label when open', () => {
    render(<LogoSidebarMobile onToggle={vi.fn()} isOpen={true} />)
    const button = screen.getByRole('button')
    expect(button).toHaveAttribute('aria-label', 'Fermer le menu')
  })

  it('has correct aria-label when closed', () => {
    render(<LogoSidebarMobile onToggle={vi.fn()} isOpen={false} />)
    const button = screen.getByRole('button')
    expect(button).toHaveAttribute('aria-label', 'Ouvrir le menu')
  })

  it('displays app name', () => {
    render(<LogoSidebarMobile onToggle={vi.fn()} isOpen={false} />)
    expect(screen.getByText('app.name')).toBeInTheDocument()
  })

  it('renders logo image', () => {
    render(<LogoSidebarMobile onToggle={vi.fn()} isOpen={false} />)
    const logo = screen.getByAltText('Icarus Logo')
    expect(logo).toBeInTheDocument()
    expect(logo.className).toContain('h-8')
    expect(logo.className).toContain('w-8')
  })
})
