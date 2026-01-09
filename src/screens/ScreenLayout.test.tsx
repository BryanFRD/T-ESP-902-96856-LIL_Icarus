import { render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import ScreenLayout from '@/screens/ScreenLayout'

vi.mock('react-router-dom', () => ({
  Outlet: () => <div data-testid='outlet'>Outlet Content</div>
}))

vi.mock('@/components/layout/sidebar/Sidebar', () => ({
  default: ({
    isOpen,
    menuItems
  }: {
    isOpen: boolean
    menuItems: unknown[]
  }) => (
    <div data-testid='sidebar' data-is-open={isOpen}>
      Sidebar - Items: {menuItems.length}
    </div>
  )
}))

vi.mock('@/constant.ts', () => ({
  MENU_ITEMS: [
    { label: 'Dashboard', href: '/dashboard' },
    { label: 'Profile', href: '/profile' }
  ]
}))

vi.mock('@/hooks/useToggle.tsx', () => ({
  default: (initialValue: boolean) => ({
    toggle: vi.fn(),
    value: initialValue,
    close: vi.fn(),
    open: vi.fn()
  })
}))

describe('ScreenLayout', () => {
  it('renders without crashing', () => {
    render(<ScreenLayout />)
    expect(screen.getByTestId('sidebar')).toBeInTheDocument()
  })

  it('renders Sidebar component', () => {
    render(<ScreenLayout />)
    expect(screen.getByTestId('sidebar')).toBeInTheDocument()
  })

  it('renders Outlet component for nested routes', () => {
    render(<ScreenLayout />)
    expect(screen.getByTestId('outlet')).toBeInTheDocument()
  })

  it('passes menu items to Sidebar', () => {
    render(<ScreenLayout />)
    const sidebar = screen.getByTestId('sidebar')
    expect(sidebar).toHaveTextContent('Items: 2')
  })

  it('initializes sidebar as closed', () => {
    render(<ScreenLayout />)
    const sidebar = screen.getByTestId('sidebar')
    expect(sidebar).toHaveAttribute('data-is-open', 'false')
  })

  it('has correct layout structure', () => {
    const { container } = render(<ScreenLayout />)
    const mainContainer = container.firstChild as HTMLElement
    expect(mainContainer.className).toContain('flex')
    expect(mainContainer.className).toContain('h-screen')
  })

  it('renders main content area', () => {
    const { container } = render(<ScreenLayout />)
    const main = container.querySelector('main')
    expect(main).toBeInTheDocument()
    expect(main?.className).toContain('flex-1')
  })
})
