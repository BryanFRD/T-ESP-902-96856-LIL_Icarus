import { fireEvent, render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import SocialLogin from '@/components/login/SocialLogin'

vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => key
  })
}))

vi.mock('react-social-icons', () => ({
  SocialIcon: ({ network }: { network: string }) => (
    <div data-testid={`social-icon-${network}`} />
  )
}))

vi.mock('@/components/ui/SocialButton.tsx', () => ({
  default: ({
    icon,
    label,
    onClick
  }: {
    icon: React.ReactNode
    label: string
    onClick: () => void
  }) => (
    <button onClick={onClick} data-testid={`social-button-${label}`}>
      {icon}
      {label}
    </button>
  )
}))

describe('SocialLogin component', () => {
  it('renders without crashing', () => {
    render(<SocialLogin />)
    expect(screen.getByText('social.orContinueWith')).toBeInTheDocument()
  })

  it('renders separator text', () => {
    render(<SocialLogin />)
    expect(screen.getByText('social.orContinueWith')).toBeInTheDocument()
  })

  it('renders Google social button', () => {
    render(<SocialLogin />)
    expect(
      screen.getByTestId('social-button-social.google')
    ).toBeInTheDocument()
  })

  it('renders GitHub social button', () => {
    render(<SocialLogin />)
    expect(
      screen.getByTestId('social-button-social.github')
    ).toBeInTheDocument()
  })

  it('navigates to Google when Google button is clicked', () => {
    const originalLocation = globalThis.location.href
    render(<SocialLogin />)

    const googleButton = screen.getByTestId('social-button-social.google')
    fireEvent.click(googleButton)

    expect(globalThis.location.href).toBe('https://google.com/')
    globalThis.location.href = originalLocation
  })

  it('navigates to GitHub when GitHub button is clicked', () => {
    const originalLocation = globalThis.location.href
    render(<SocialLogin />)

    const githubButton = screen.getByTestId('social-button-social.github')
    fireEvent.click(githubButton)

    expect(globalThis.location.href).toBe('https://github.com/')
    globalThis.location.href = originalLocation
  })

  it('renders social icons for Google and GitHub', () => {
    render(<SocialLogin />)
    expect(screen.getByTestId('social-icon-google')).toBeInTheDocument()
    expect(screen.getByTestId('social-icon-github')).toBeInTheDocument()
  })
})
