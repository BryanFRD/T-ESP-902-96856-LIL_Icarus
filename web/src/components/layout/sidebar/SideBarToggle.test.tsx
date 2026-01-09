import { fireEvent, render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import SidebarToggle from '@/components/layout/sidebar/SideBarToggle'

vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => key
  })
}))

vi.mock('@/components/icon/SidebarToggleIcon', () => ({
  default: ({ isOpen, size }: { isOpen: boolean; size: number }) => (
    <div
      data-testid='sidebar-toggle-icon'
      data-is-open={isOpen}
      data-size={size}
    />
  )
}))

vi.mock('@/components/ui/Button', () => ({
  default: ({
    children,
    onClick,
    className,
    ...props
  }: {
    children: React.ReactNode
    onClick: () => void
    className?: string
  }) => (
    <button onClick={onClick} className={className} {...props}>
      {children}
    </button>
  )
}))

describe('SidebarToggle component', () => {
  it('renders without crashing', () => {
    render(<SidebarToggle isOpen={false} onToggle={vi.fn()} />)
    expect(screen.getByTestId('sidebar-toggle-icon')).toBeInTheDocument()
  })

  it('calls onToggle when clicked', () => {
    const onToggle = vi.fn()
    render(<SidebarToggle isOpen={false} onToggle={onToggle} />)
    const button = screen.getByRole('button')
    fireEvent.click(button)
    expect(onToggle).toHaveBeenCalledTimes(1)
  })

  it('shows close menu text when sidebar is open', () => {
    render(<SidebarToggle isOpen={true} onToggle={vi.fn()} />)
    expect(screen.getByText('sidebar.closeMenu')).toBeInTheDocument()
  })

  it('does not show close menu text when sidebar is closed', () => {
    render(<SidebarToggle isOpen={false} onToggle={vi.fn()} />)
    expect(screen.queryByText('sidebar.closeMenu')).not.toBeInTheDocument()
  })

  it('has correct aria-label when open', () => {
    render(<SidebarToggle isOpen={true} onToggle={vi.fn()} />)
    const button = screen.getByRole('button')
    expect(button).toHaveAttribute('aria-label', 'sidebar.closeMenu')
  })

  it('has correct aria-label when closed', () => {
    render(<SidebarToggle isOpen={false} onToggle={vi.fn()} />)
    const button = screen.getByRole('button')
    expect(button).toHaveAttribute('aria-label', 'sidebar.openMenu')
  })

  it('passes correct props to SidebarToggleIcon', () => {
    render(<SidebarToggle isOpen={true} onToggle={vi.fn()} />)
    const icon = screen.getByTestId('sidebar-toggle-icon')
    expect(icon).toHaveAttribute('data-is-open', 'true')
    expect(icon).toHaveAttribute('data-size', '20')
  })

  it('applies custom className', () => {
    render(
      <SidebarToggle
        isOpen={false}
        onToggle={vi.fn()}
        className='custom-class'
      />
    )
    const button = screen.getByRole('button')
    expect(button.className).toContain('custom-class')
  })
})
