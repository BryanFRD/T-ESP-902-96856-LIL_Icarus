import { render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import RegisterHeader from '@/components/register/RegisterHeader'

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

describe('RegisterHeader component', () => {
  it('renders without crashing', () => {
    render(<RegisterHeader />)
    expect(screen.getByText('createAccount')).toBeInTheDocument()
  })

  it('renders create account text', () => {
    render(<RegisterHeader />)
    expect(screen.getByText('createAccount')).toBeInTheDocument()
  })

  it('renders register to continue text', () => {
    render(<RegisterHeader />)
    expect(screen.getByText('registerToContinue')).toBeInTheDocument()
  })

  it('renders Logo component with correct props', () => {
    render(<RegisterHeader />)
    const logo = screen.getByTestId('logo')
    expect(logo).toBeInTheDocument()
    expect(logo).toHaveAttribute('data-fill', '#ff6900')
    expect(logo.className).toContain('h-20')
    expect(logo.className).toContain('w-20')
  })

  it('has correct styling for create account text', () => {
    render(<RegisterHeader />)
    const heading = screen.getByText('createAccount')
    expect(heading.tagName).toBe('H1')
    expect(heading.className).toContain('text-3xl')
    expect(heading.className).toContain('font-bold')
  })

  it('has centered text layout', () => {
    const { container } = render(<RegisterHeader />)
    const mainDiv = container.firstChild as HTMLElement
    expect(mainDiv.className).toContain('text-center')
  })
})
