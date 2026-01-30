import { render } from '@testing-library/react'
import { describe, expect, it } from 'vitest'

import Logo from '@/components/icon/Logo.tsx'

describe('Logo components', () => {
  it('renders without crashing', () => {
    const { container } = render(<Logo />)
    expect(container.querySelector('svg')).toBeInTheDocument()
  })

  it('applies default width and height', () => {
    const { container } = render(<Logo />)
    const svg = container.querySelector('svg')
    expect(svg).toHaveAttribute('width', '1024pt')
    expect(svg).toHaveAttribute('height', '1024pt')
  })

  it('applies custom width and height when provided', () => {
    const { container } = render(<Logo width='512' height='512' />)
    const svg = container.querySelector('svg')
    expect(svg).toHaveAttribute('width', '512')
    expect(svg).toHaveAttribute('height', '512')
  })

  it('applies default fill color', () => {
    const { container } = render(<Logo />)
    const g = container.querySelector('g')
    expect(g).toHaveAttribute('fill', 'currentColor')
  })

  it('applies custom fill color when provided', () => {
    const { container } = render(<Logo fill='#FF6900' />)
    const g = container.querySelector('g')
    expect(g).toHaveAttribute('fill', '#FF6900')
  })

  it('applies custom className when provided', () => {
    const { container } = render(<Logo className='custom-logo-class' />)
    const svg = container.querySelector('svg')
    expect(svg?.className).toContain('custom-logo-class')
  })

  it('has correct viewBox', () => {
    const { container } = render(<Logo />)
    const svg = container.querySelector('svg')
    expect(svg).toHaveAttribute('viewBox', '0 0 1024.000000 1024.000000')
  })

  it('renders all path elements', () => {
    const { container } = render(<Logo />)
    const paths = container.querySelectorAll('path')
    expect(paths.length).toBe(3)
  })
})
