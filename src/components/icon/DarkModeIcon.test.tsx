import { render } from '@testing-library/react'
import { describe, expect, it } from 'vitest'

import DarkModeIcon from '@/components/icon/DarkModeIcon'

describe('DarkModeIcon component', () => {
  it('renders without crashing', () => {
    const { container } = render(<DarkModeIcon isDark={false} />)
    expect(container.querySelector('svg')).toBeInTheDocument()
  })

  it('applies correct rotation when isDark is false (light mode)', () => {
    const { container } = render(<DarkModeIcon isDark={false} />)
    const svg = container.querySelector('svg')
    expect(svg?.className).toContain('rotate-180')
  })

  it('applies correct rotation when isDark is true (dark mode)', () => {
    const { container } = render(<DarkModeIcon isDark={true} />)
    const svg = container.querySelector('svg')
    expect(svg?.className).toContain('rotate-0')
  })

  it('applies custom className when provided', () => {
    const { container } = render(
      <DarkModeIcon isDark={false} className='custom-class' />
    )
    const svg = container.querySelector('svg')
    expect(svg?.className).toContain('custom-class')
  })

  it('renders sun icon elements in light mode', () => {
    const { container } = render(<DarkModeIcon isDark={false} />)
    const circle = container.querySelector('circle')
    expect(circle).toHaveAttribute('fill', '#FFD700')
  })

  it('renders moon icon path in dark mode', () => {
    const { container } = render(<DarkModeIcon isDark={true} />)
    const path = container.querySelector('path')
    expect(path).toBeInTheDocument()
  })

  it('has correct transition classes', () => {
    const { container } = render(<DarkModeIcon isDark={false} />)
    const svg = container.querySelector('svg')
    expect(svg?.className).toContain('transition-all')
    expect(svg?.className).toContain('duration-500')
  })
})
