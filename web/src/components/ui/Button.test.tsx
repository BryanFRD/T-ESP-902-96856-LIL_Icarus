import { fireEvent, render, screen } from '@testing-library/react'
import { Loader2 } from 'lucide-react'
import { MemoryRouter } from 'react-router-dom'
import { describe, expect, it, vi } from 'vitest'

import Button from '@/components/ui/Button'

describe('Button component', () => {
  it('renders children correctly', () => {
    render(<Button>Click me</Button>)
    expect(screen.getByText('Click me')).toBeInTheDocument()
  })

  it('calls onClick when clicked', () => {
    const onClick = vi.fn()
    render(<Button onClick={onClick}>Click</Button>)
    fireEvent.click(screen.getByText('Click'))
    expect(onClick).toHaveBeenCalledTimes(1)
  })

  it('does not call onClick when disabled', () => {
    const onClick = vi.fn()
    render(
      <Button disabled onClick={onClick}>
        Disabled
      </Button>
    )
    fireEvent.click(screen.getByText('Disabled'))
    expect(onClick).not.toHaveBeenCalled()
  })

  it('shows loader when isLoading is true', () => {
    render(
      <Button isLoading loadingText='Loading...'>
        Submit
      </Button>
    )
    const button = screen.getByRole('button')

    expect(button).toHaveTextContent('Loading...')
    // l’icône Loader2 génère un svg => on le vérifie
    expect(button.querySelector('svg')).toBeTruthy()
  })

  it('renders left icon when provided', () => {
    const FakeIcon = Loader2

    render(<Button icon={{ type: FakeIcon, position: 'left' }}>Go</Button>)

    const button = screen.getByRole('button')

    // Vérifie qu'un svg est présent
    const icons = button.querySelectorAll('svg')
    expect(icons.length).toBe(1)
  })

  it('renders as Link when as="link"', () => {
    render(
      <MemoryRouter>
        <Button as='link' to='/test'>
          Navigate
        </Button>
      </MemoryRouter>
    )

    const link = screen.getByText('Navigate')
    expect(link.tagName).toBe('A')
    expect(link).toHaveAttribute('href', '/test')
  })

  it('applies correct classes for size/rounded', () => {
    render(
      <Button size='lg' rounded='full'>
        Test
      </Button>
    )

    const button = screen.getByText('Test')
    expect(button.className).toContain('rounded-full')
    expect(button.className).toContain('px-3')
    expect(button.className).toContain('py-3')
  })
})
