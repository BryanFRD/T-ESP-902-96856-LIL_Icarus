import { fireEvent, render, screen } from '@testing-library/react'
import { Mail } from 'lucide-react'
import { describe, expect, it, vi } from 'vitest'

import SocialButton from '@/components/auth/login/SocialButton.tsx'

describe('SocialButton components', () => {
  it('renders label correctly', () => {
    render(
      <SocialButton
        icon={<Mail />}
        label='Sign in with Email'
        onClick={vi.fn()}
      />
    )

    expect(screen.getByText('Sign in with Email')).toBeInTheDocument()
  })

  it('renders icon correctly', () => {
    render(
      <SocialButton
        icon={<Mail data-testid='mail-icon' />}
        label='Sign in with Email'
        onClick={vi.fn()}
      />
    )

    expect(screen.getByTestId('mail-icon')).toBeInTheDocument()
  })

  it('calls onClick when button is clicked', () => {
    const onClick = vi.fn()
    render(
      <SocialButton
        icon={<Mail />}
        label='Sign in with Email'
        onClick={onClick}
      />
    )

    const button = screen.getByRole('button')
    fireEvent.click(button)

    expect(onClick).toHaveBeenCalledTimes(1)
  })

  it('applies correct styling classes', () => {
    render(
      <SocialButton
        icon={<Mail />}
        label='Sign in with Email'
        onClick={vi.fn()}
      />
    )

    const button = screen.getByRole('button')
    expect(button.className).toContain('flex')
    expect(button.className).toContain('items-center')
    expect(button.className).toContain('justify-center')
    expect(button.className).toContain('gap-2')
    expect(button.className).toContain('rounded-xl')
    expect(button.className).toContain('border')
  })

  it('renders with custom icon components', () => {
    const CustomIcon = () => <span data-testid='custom-icon'>🔑</span>
    render(
      <SocialButton icon={<CustomIcon />} label='Sign in' onClick={vi.fn()} />
    )

    expect(screen.getByTestId('custom-icon')).toBeInTheDocument()
  })

  it('renders button as a button element', () => {
    render(
      <SocialButton
        icon={<Mail />}
        label='Sign in with Email'
        onClick={vi.fn()}
      />
    )

    const button = screen.getByRole('button')
    expect(button.tagName).toBe('BUTTON')
  })

  it('has hover transition classes', () => {
    render(
      <SocialButton
        icon={<Mail />}
        label='Sign in with Email'
        onClick={vi.fn()}
      />
    )

    const button = screen.getByRole('button')
    expect(button.className).toContain('transition-colors')
    expect(button.className).toContain('hover:bg-gray-50')
  })

  it('renders both icon and label together', () => {
    render(
      <SocialButton
        icon={<Mail data-testid='mail-icon' />}
        label='Sign in with Email'
        onClick={vi.fn()}
      />
    )

    expect(screen.getByTestId('mail-icon')).toBeInTheDocument()
    expect(screen.getByText('Sign in with Email')).toBeInTheDocument()

    const button = screen.getByRole('button')
    expect(button).toContainElement(screen.getByTestId('mail-icon'))
  })
})
