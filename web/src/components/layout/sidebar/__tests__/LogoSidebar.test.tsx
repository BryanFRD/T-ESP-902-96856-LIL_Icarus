import { fireEvent, render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import LogoSidebar from '@/components/layout/sidebar/LogoSidebar.tsx'

vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => key
  })
}))

vi.mock('@/components/icon/Logo.tsx', () => ({
  default: ({ fill, className }: { fill?: string; className?: string }) => (
    <div data-testid='logo' data-fill={fill} className={className} />
  )
}))

describe('LogoSidebar components', () => {
  it('renders without crashing', () => {
    render(<LogoSidebar onClose={vi.fn()} isDesktopCollapsed={false} />)
    expect(screen.getByTestId('logo')).toBeInTheDocument()
  })

  it('renders app name when not collapsed', () => {
    render(<LogoSidebar onClose={vi.fn()} isDesktopCollapsed={false} />)
    expect(screen.getByText('app.name')).toBeInTheDocument()
  })

  it('renders app description when not collapsed', () => {
    render(<LogoSidebar onClose={vi.fn()} isDesktopCollapsed={false} />)
    expect(screen.getByText('app.description')).toBeInTheDocument()
  })

  it('renders Logo with correct props', () => {
    render(<LogoSidebar onClose={vi.fn()} isDesktopCollapsed={false} />)
    const logo = screen.getByTestId('logo')
    expect(logo).toHaveAttribute('data-fill', '#ff6900')
    expect(logo.className).toContain('h-10')
    expect(logo.className).toContain('w-10')
  })

  it('calls onClose when close button is clicked', () => {
    const onClose = vi.fn()
    render(<LogoSidebar onClose={onClose} isDesktopCollapsed={false} />)
    const closeButton = screen.getByLabelText('Close Sidebar')
    fireEvent.click(closeButton)
    expect(onClose).toHaveBeenCalledTimes(1)
  })

  it('applies collapsed styles when isDesktopCollapsed is true', () => {
    const { container } = render(
      <LogoSidebar onClose={vi.fn()} isDesktopCollapsed={true} />
    )
    const wrapper = container.querySelector('.ml-4\\.5')
    expect(wrapper).toBeInTheDocument()
  })

  it('applies expanded styles when isDesktopCollapsed is false', () => {
    const { container } = render(
      <LogoSidebar onClose={vi.fn()} isDesktopCollapsed={false} />
    )
    const wrapper = container.querySelector('.ml-0')
    expect(wrapper).toBeInTheDocument()
  })

  it('renders close button with X icon', () => {
    render(<LogoSidebar onClose={vi.fn()} isDesktopCollapsed={false} />)
    const closeButton = screen.getByLabelText('Close Sidebar')
    expect(closeButton).toBeInTheDocument()
    expect(closeButton.querySelector('svg')).toBeInTheDocument()
  })
})
