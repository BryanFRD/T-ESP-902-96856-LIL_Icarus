import { fireEvent, render, screen } from '@testing-library/react'
import { Mail } from 'lucide-react'
import { describe, expect, it, vi } from 'vitest'

import InputField from '@/components/oldUi/InputField.tsx'

describe('InputField components', () => {
  const mockRegister = vi.fn(name => ({
    name,
    onChange: vi.fn(),
    onBlur: vi.fn(),
    ref: vi.fn()
  }))

  it('renders label and input correctly', () => {
    render(
      <InputField
        label='Email'
        type='text'
        icon={Mail}
        placeholder='Enter your email'
        register={mockRegister}
        name='email'
      />
    )

    expect(screen.getByText('Email')).toBeInTheDocument()
    expect(screen.getByPlaceholderText('Enter your email')).toBeInTheDocument()
  })

  it('displays error message when error prop is provided', () => {
    render(
      <InputField
        label='Email'
        type='text'
        icon={Mail}
        placeholder='Enter your email'
        register={mockRegister}
        name='email'
        error='Email is required'
      />
    )

    expect(screen.getByText('Email is required')).toBeInTheDocument()
  })

  it('shows password toggle button for password types', () => {
    render(
      <InputField
        label='Password'
        type='password'
        icon={Mail}
        placeholder='Enter password'
        register={mockRegister}
        name='password'
      />
    )

    const toggleButton = screen.getByRole('button')
    expect(toggleButton).toBeInTheDocument()
  })

  it('toggles password visibility when toggle button is clicked', () => {
    render(
      <InputField
        label='Password'
        type='password'
        icon={Mail}
        placeholder='Enter password'
        register={mockRegister}
        name='password'
      />
    )

    const input = screen.getByPlaceholderText(
      'Enter password'
    ) as HTMLInputElement
    expect(input.type).toBe('password')

    const toggleButton = screen.getByRole('button')
    fireEvent.click(toggleButton)

    expect(input.type).toBe('text')

    fireEvent.click(toggleButton)
    expect(input.type).toBe('password')
  })

  it('does not show toggle button for non-password types', () => {
    render(
      <InputField
        label='Email'
        type='text'
        icon={Mail}
        placeholder='Enter your email'
        register={mockRegister}
        name='email'
      />
    )

    expect(screen.queryByRole('button')).not.toBeInTheDocument()
  })

  it('applies error styles when error is present', () => {
    render(
      <InputField
        label='Email'
        type='text'
        icon={Mail}
        placeholder='Enter your email'
        register={mockRegister}
        name='email'
        error='Email is required'
      />
    )

    const input = screen.getByPlaceholderText('Enter your email')
    expect(input.className).toContain('border-red-300')
  })

  it('sets autoComplete attribute correctly', () => {
    render(
      <InputField
        label='Email'
        type='text'
        icon={Mail}
        placeholder='Enter your email'
        register={mockRegister}
        name='email'
        autoComplete='email'
      />
    )

    const input = screen.getByPlaceholderText('Enter your email')
    expect(input).toHaveAttribute('autocomplete', 'email')
  })
})
