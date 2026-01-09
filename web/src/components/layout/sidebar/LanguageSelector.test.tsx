import { fireEvent, render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import LanguageSelector from '@/components/layout/sidebar/LanguageSelector'

const mockChangeLanguage = vi.fn()

vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => key
  })
}))

vi.mock('@/hooks/lang/useLanguage', () => ({
  default: () => ({
    currentLanguage: 'en',
    changeLanguage: mockChangeLanguage
  })
}))

vi.mock('@/hooks/lang/useLanguageOptions', () => ({
  useLanguageOptions: () => [
    { code: 'en', label: 'English' },
    { code: 'fr', label: 'Français' }
  ]
}))

vi.mock('react-country-flag', () => ({
  default: ({ countryCode }: { countryCode: string }) => (
    <div data-testid='country-flag' data-country={countryCode} />
  )
}))

describe('LanguageSelector component', () => {
  it('renders without crashing', () => {
    render(<LanguageSelector />)
    expect(screen.getByText('language')).toBeInTheDocument()
  })

  it('shows language options when clicked', () => {
    render(<LanguageSelector />)
    const button = screen.getByText('language')
    fireEvent.click(button)
    expect(screen.getByText('English')).toBeInTheDocument()
    expect(screen.getByText('Français')).toBeInTheDocument()
  })

  it('calls changeLanguage when language is selected', () => {
    render(<LanguageSelector />)
    const mainButton = screen.getByText('language')
    fireEvent.click(mainButton)

    const frenchButton = screen.getByText('Français')
    fireEvent.click(frenchButton)

    expect(mockChangeLanguage).toHaveBeenCalledWith('fr')
  })

  it('renders language options', () => {
    render(<LanguageSelector />)
    expect(screen.getByText('English')).toBeInTheDocument()
    expect(screen.getByText('Français')).toBeInTheDocument()
  })

  it('renders country flags', () => {
    render(<LanguageSelector />)
    const flags = screen.getAllByTestId('country-flag')
    expect(flags.length).toBeGreaterThan(0)
  })
})
