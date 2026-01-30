import { fireEvent, render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import ThemeToggle from '@/components/layout/sidebar/ThemeToggle.tsx'

const mockToggleTheme = vi.fn()

vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => key
  })
}))

vi.mock('@/context/ThemeContext', () => ({
  useTheme: () => ({
    theme: 'light',
    toggleTheme: mockToggleTheme
  })
}))

vi.mock('@/components/icon/DarkModeIcon', () => ({
  default: ({ isDark, className }: { isDark: boolean; className?: string }) => (
    <div
      data-testid='dark-mode-icon'
      data-is-dark={isDark}
      className={className}
    />
  )
}))

describe('ThemeToggle components', () => {
  it('renders button with theme label', () => {
    render(<ThemeToggle />)
    expect(screen.getByText('theme')).toBeInTheDocument()
  })

  it('renders DarkModeIcon components', () => {
    render(<ThemeToggle />)
    expect(screen.getByTestId('dark-mode-icon')).toBeInTheDocument()
  })

  it('calls toggleTheme when clicked', () => {
    render(<ThemeToggle />)
    const button = screen.getByRole('button')
    fireEvent.click(button)
    expect(mockToggleTheme).toHaveBeenCalledTimes(1)
  })

  it('passes correct isDark prop to DarkModeIcon when theme is light', () => {
    render(<ThemeToggle />)
    const icon = screen.getByTestId('dark-mode-icon')
    expect(icon).toHaveAttribute('data-is-dark', 'false')
  })

  it('has correct styling classes', () => {
    render(<ThemeToggle />)
    const button = screen.getByRole('button')
    expect(button.className).toContain('flex')
    expect(button.className).toContain('items-center')
    expect(button.className).toContain('justify-between')
  })
})
