import { render, screen } from '@testing-library/react'
import { I18nextProvider } from 'react-i18next'
import { describe, expect, it } from 'vitest'

import LoadingButton from '@/components/ui/LoadingButton'
import i18n from '@/i18n'

describe('LoadingButton component', () => {
  const renderWithI18n = (component: React.ReactElement) => {
    return render(<I18nextProvider i18n={i18n}>{component}</I18nextProvider>)
  }

  it('renders children when not loading', () => {
    renderWithI18n(<LoadingButton isLoading={false}>Submit</LoadingButton>)
    expect(screen.getByText('Submit')).toBeInTheDocument()
  })

  it('shows loading state with spinner when isLoading is true', () => {
    renderWithI18n(
      <LoadingButton isLoading={true} loadingText='Please wait'>
        Submit
      </LoadingButton>
    )

    expect(screen.getByText('Please wait')).toBeInTheDocument()
    const button = screen.getByRole('button')
    expect(button.querySelector('svg')).toBeTruthy()
  })

  it('uses default loading text from translation when loadingText is not provided', () => {
    renderWithI18n(<LoadingButton isLoading={true}>Submit</LoadingButton>)

    const button = screen.getByRole('button')
    expect(button).toBeInTheDocument()
  })

  it('is disabled when isLoading is true', () => {
    renderWithI18n(<LoadingButton isLoading={true}>Submit</LoadingButton>)

    const button = screen.getByRole('button')
    expect(button).toBeDisabled()
  })

  it('is disabled when disabled prop is true', () => {
    renderWithI18n(
      <LoadingButton isLoading={false} disabled={true}>
        Submit
      </LoadingButton>
    )

    const button = screen.getByRole('button')
    expect(button).toBeDisabled()
  })

  it('is clickable when not disabled or loading', () => {
    renderWithI18n(<LoadingButton isLoading={false}>Submit</LoadingButton>)

    const button = screen.getByText('Submit')
    expect(button).not.toBeDisabled()
  })

  it('applies custom className', () => {
    renderWithI18n(
      <LoadingButton isLoading={false} className='custom-class'>
        Submit
      </LoadingButton>
    )

    const button = screen.getByRole('button')
    expect(button.className).toContain('custom-class')
  })

  it('sets button type correctly', () => {
    renderWithI18n(
      <LoadingButton isLoading={false} type='submit'>
        Submit
      </LoadingButton>
    )

    const button = screen.getByRole('button')
    expect(button).toHaveAttribute('type', 'submit')
  })

  it('defaults to button type when type is not provided', () => {
    renderWithI18n(<LoadingButton isLoading={false}>Submit</LoadingButton>)

    const button = screen.getByRole('button')
    expect(button).toHaveAttribute('type', 'button')
  })
})
