import { render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import LoginHeader from '@/components/login/LoginHeader'

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

describe('LoginHeader component', () => {
  it('renders without crashing', () => {
    render(<LoginHeader />)
    expect(screen.getByText('welcomeBack')).toBeInTheDocument()
  })

  it('renders welcome back text', () => {
    render(<LoginHeader />)
    expect(screen.getByText('welcomeBack')).toBeInTheDocument()
  })

  it('renders sign in to dashboard text', () => {
    render(<LoginHeader />)
    expect(screen.getByText('signInToDashboard')).toBeInTheDocument()
  })

  it('renders Logo component with correct props', () => {
    render(<LoginHeader />)
    const logo = screen.getByTestId('logo')
    expect(logo).toBeInTheDocument()
    expect(logo).toHaveAttribute('data-fill', '#ff6900')
    expect(logo.className).toContain('h-20')
    expect(logo.className).toContain('w-20')
  })

  it('has correct styling for welcome text', () => {
    render(<LoginHeader />)
    const heading = screen.getByText('welcomeBack')
    expect(heading.tagName).toBe('H1')
    expect(heading.className).toContain('text-3xl')
    expect(heading.className).toContain('font-bold')
  })

  it('has centered text layout', () => {
    const { container } = render(<LoginHeader />)
    const mainDiv = container.firstChild as HTMLElement
    expect(mainDiv.className).toContain('text-center')
  })
})
