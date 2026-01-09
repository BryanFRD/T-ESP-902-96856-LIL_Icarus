import { fireEvent, render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import LanguageSelector from '@/components/LanguageSelector'

vi.mock('@/hooks/lang/useLanguageOptions', () => ({
  useLanguageOptions: () => [
    { code: 'en', label: 'English' },
    { code: 'fr', label: 'Français' },
    { code: 'es', label: 'Español' }
  ]
}))

describe('LanguageSelector component', () => {
  it('renders select element', () => {
    render(<LanguageSelector onChange={vi.fn()} />)
    expect(screen.getByRole('combobox')).toBeInTheDocument()
  })

  it('renders all language options', () => {
    render(<LanguageSelector onChange={vi.fn()} />)
    expect(screen.getByText('English')).toBeInTheDocument()
    expect(screen.getByText('Français')).toBeInTheDocument()
    expect(screen.getByText('Español')).toBeInTheDocument()
  })

  it('calls onChange with selected language code', () => {
    const onChange = vi.fn()
    render(<LanguageSelector onChange={onChange} />)

    const select = screen.getByRole('combobox')
    fireEvent.change(select, { target: { value: 'fr' } })

    expect(onChange).toHaveBeenCalledWith('fr')
  })

  it('renders options with correct values', () => {
    render(<LanguageSelector onChange={vi.fn()} />)

    const options = screen.getAllByRole('option')
    expect(options[0]).toHaveValue('en')
    expect(options[1]).toHaveValue('fr')
    expect(options[2]).toHaveValue('es')
  })
})
