import { fireEvent, render, screen } from '@testing-library/react'
import { I18nextProvider } from 'react-i18next'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import PreferencesToggle from '@/components/ui/PreferencesToggle.tsx'
import { ThemeProvider } from '@/context/ThemeContext.tsx'
import i18n from '@/i18n'

const mockChangeLanguage = vi.fn()

vi.mock('@/hooks/lang/useLanguage', () => ({
  default: () => ({
    currentLanguage: 'en',
    changeLanguage: mockChangeLanguage
  })
}))

vi.mock('framer-motion', () => ({
  AnimatePresence: ({ children }: { children: React.ReactNode }) => (
    <>{children}</>
  ),
  motion: {
    span: ({ children, ...props }: React.HTMLAttributes<HTMLSpanElement>) => (
      <span {...props}>{children}</span>
    )
  }
}))

vi.mock('react-country-flag', () => ({
  default: ({ countryCode }: { countryCode: string }) => (
    <span data-testid={`flag-${countryCode}`}>{countryCode}</span>
  )
}))

vi.mock('../icon/DarkModeIcon', () => ({
  default: ({ isDark }: { isDark: boolean }) => (
    <span data-testid={`dark-mode-icon-${isDark}`}>🌙</span>
  )
}))

describe('PreferencesToggle components', () => {
  const renderWithProviders = (component: React.ReactElement) => {
    return render(
      <I18nextProvider i18n={i18n}>
        <ThemeProvider>{component}</ThemeProvider>
      </I18nextProvider>
    )
  }

  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('renders language and theme toggle buttons', () => {
    renderWithProviders(<PreferencesToggle />)

    const buttons = screen.getAllByRole('button')
    expect(buttons).toHaveLength(2)
  })

  it('displays US flag for English language', () => {
    renderWithProviders(<PreferencesToggle />)

    const usFlag = screen.getByTestId('flag-US')
    expect(usFlag).toBeInTheDocument()
  })

  it('calls changeLanguage when language button is clicked', () => {
    renderWithProviders(<PreferencesToggle />)

    const buttons = screen.getAllByRole('button')
    const languageButton = buttons[0]

    fireEvent.click(languageButton)

    expect(mockChangeLanguage).toHaveBeenCalledWith('fr')
  })

  it('applies correct hover styles to buttons', () => {
    renderWithProviders(<PreferencesToggle />)

    const buttons = screen.getAllByRole('button')
    buttons.forEach(button => {
      expect(button.className).toContain('hover:bg-gray-200')
      expect(button.className).toContain('dark:hover:bg-gray-800')
    })
  })

  it('theme button is clickable', () => {
    renderWithProviders(<PreferencesToggle />)

    const buttons = screen.getAllByRole('button')
    const themeButton = buttons[1]

    fireEvent.click(themeButton)

    expect(themeButton).toBeInTheDocument()
  })
})
