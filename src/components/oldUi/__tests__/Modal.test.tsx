import { fireEvent, render, screen } from '@testing-library/react'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

import Modal from '@/components/oldUi/modal/Modal.tsx'

describe('Modal components', () => {
  beforeEach(() => {
    document.body.style.overflow = 'auto'
  })

  afterEach(() => {
    document.body.style.overflow = 'auto'
  })

  it('renders modal when isOpen is true', () => {
    render(
      <Modal isOpen={true} title='Test Modal'>
        <p>Modal content</p>
      </Modal>
    )

    expect(screen.getByText('Test Modal')).toBeInTheDocument()
    expect(screen.getByText('Modal content')).toBeInTheDocument()
  })

  it('does not render modal when isOpen is false', () => {
    render(
      <Modal isOpen={false} title='Test Modal'>
        <p>Modal content</p>
      </Modal>
    )

    expect(screen.queryByText('Test Modal')).not.toBeInTheDocument()
    expect(screen.queryByText('Modal content')).not.toBeInTheDocument()
  })

  it('calls onClose when close button is clicked', () => {
    const onClose = vi.fn()
    render(
      <Modal isOpen={true} title='Test Modal' onClose={onClose}>
        <p>Modal content</p>
      </Modal>
    )

    const closeButton = screen.getByRole('button', { hidden: true })
    fireEvent.click(closeButton)

    expect(onClose).toHaveBeenCalledTimes(1)
  })

  it('calls onClose when Escape key is pressed', () => {
    const onClose = vi.fn()
    render(
      <Modal isOpen={true} title='Test Modal' onClose={onClose}>
        <p>Modal content</p>
      </Modal>
    )

    fireEvent.keyDown(document, { key: 'Escape' })

    expect(onClose).toHaveBeenCalledTimes(1)
  })

  it('does not call onClose when Escape is pressed and disableEscapeKey is true', () => {
    const onClose = vi.fn()
    render(
      <Modal
        isOpen={true}
        title='Test Modal'
        onClose={onClose}
        disableEscapeKey={true}
      >
        <p>Modal content</p>
      </Modal>
    )

    fireEvent.keyDown(document, { key: 'Escape' })

    expect(onClose).not.toHaveBeenCalled()
  })

  it('renders footer when provided', () => {
    render(
      <Modal
        isOpen={true}
        title='Test Modal'
        footer={
          <div>
            <button>Cancel</button>
            <button>Confirm</button>
          </div>
        }
      >
        <p>Modal content</p>
      </Modal>
    )

    expect(screen.getByText('Cancel')).toBeInTheDocument()
    expect(screen.getByText('Confirm')).toBeInTheDocument()
  })

  it('applies correct size class', () => {
    const { container } = render(
      <Modal isOpen={true} title='Test Modal' size='lg'>
        <p>Modal content</p>
      </Modal>
    )

    const modalContent = container.querySelector('.max-w-lg')
    expect(modalContent).toBeInTheDocument()
  })

  it('applies custom className', () => {
    const { container } = render(
      <Modal isOpen={true} title='Test Modal' className='custom-modal-class'>
        <p>Modal content</p>
      </Modal>
    )

    const modalContent = container.querySelector('.custom-modal-class')
    expect(modalContent).toBeInTheDocument()
  })

  it('sets body overflow to hidden when modal is open', () => {
    render(
      <Modal isOpen={true} title='Test Modal'>
        <p>Modal content</p>
      </Modal>
    )

    expect(document.body.style.overflow).toBe('hidden')
  })

  it('restores body overflow when modal is closed', () => {
    const { rerender } = render(
      <Modal isOpen={true} title='Test Modal'>
        <p>Modal content</p>
      </Modal>
    )

    expect(document.body.style.overflow).toBe('hidden')

    rerender(
      <Modal isOpen={false} title='Test Modal'>
        <p>Modal content</p>
      </Modal>
    )

    expect(document.body.style.overflow).toBe('auto')
  })

  it('has correct ARIA attributes', () => {
    render(
      <Modal isOpen={true} title='Test Modal'>
        <p>Modal content</p>
      </Modal>
    )

    const dialog = screen.getByRole('dialog', { hidden: true })
    expect(dialog).toHaveAttribute('aria-modal', 'true')
    expect(dialog).toHaveAttribute('aria-labelledby', 'modal-title')
  })

  it('renders close button in modal header', () => {
    render(
      <Modal isOpen={true} title='Test Modal'>
        <p>Modal content</p>
      </Modal>
    )

    const closeButton = screen.getByRole('button', { hidden: true })
    expect(closeButton).toBeInTheDocument()
    expect(closeButton.tagName).toBe('BUTTON')
  })

  it('renders backdrop when modal is open', () => {
    const { container } = render(
      <Modal isOpen={true} title='Test Modal'>
        <p>Modal content</p>
      </Modal>
    )

    const backdrop = container.querySelector('.fixed.inset-0')
    expect(backdrop).toBeInTheDocument()
  })
})
