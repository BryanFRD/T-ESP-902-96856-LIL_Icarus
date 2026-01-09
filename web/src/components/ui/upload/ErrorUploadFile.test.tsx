import { render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import { ErrorUploadFile } from '@/components/ui/upload/ErrorUploadFile'

vi.mock('i18next', () => ({
  t: (key: string) => key
}))

describe('ErrorUploadFile component', () => {
  it('renders nothing when errors array is empty', () => {
    const { container } = render(<ErrorUploadFile errors={[]} />)
    expect(container.firstChild).toBeNull()
  })

  it('renders error messages when errors are provided', () => {
    render(<ErrorUploadFile errors={['Error 1', 'Error 2']} />)
    expect(screen.getByText('• Error 1')).toBeInTheDocument()
    expect(screen.getByText('• Error 2')).toBeInTheDocument()
  })

  it('renders validation errors header', () => {
    render(<ErrorUploadFile errors={['Error 1']} />)
    expect(
      screen.getByText('file_upload.validation_errors')
    ).toBeInTheDocument()
  })

  it('renders AlertCircle icon', () => {
    const { container } = render(<ErrorUploadFile errors={['Error 1']} />)
    const svg = container.querySelector('svg')
    expect(svg).toBeInTheDocument()
  })

  it('applies compact styles when compact prop is true', () => {
    const { container } = render(
      <ErrorUploadFile errors={['Error 1']} compact={true} />
    )
    const mainDiv = container.firstChild as HTMLElement
    expect(mainDiv.className).toContain('mt-3')
    expect(mainDiv.className).toContain('p-3')
  })

  it('applies default styles when compact prop is false', () => {
    const { container } = render(
      <ErrorUploadFile errors={['Error 1']} compact={false} />
    )
    const mainDiv = container.firstChild as HTMLElement
    expect(mainDiv.className).toContain('mt-4')
    expect(mainDiv.className).toContain('p-4')
  })

  it('renders multiple errors in a list', () => {
    render(<ErrorUploadFile errors={['Error 1', 'Error 2', 'Error 3']} />)
    expect(screen.getByText('• Error 1')).toBeInTheDocument()
    expect(screen.getByText('• Error 2')).toBeInTheDocument()
    expect(screen.getByText('• Error 3')).toBeInTheDocument()
  })

  it('has correct error styling classes', () => {
    const { container } = render(<ErrorUploadFile errors={['Error 1']} />)
    const mainDiv = container.firstChild as HTMLElement
    expect(mainDiv.className).toContain('border-red-200')
    expect(mainDiv.className).toContain('bg-red-50')
  })
})
