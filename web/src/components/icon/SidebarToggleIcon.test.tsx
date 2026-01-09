import { render } from '@testing-library/react'
import { describe, expect, it } from 'vitest'

import SidebarToggleIcon from '@/components/icon/SidebarToggleIcon'

describe('SidebarToggleIcon component', () => {
  it('renders without crashing', () => {
    const { container } = render(<SidebarToggleIcon isOpen={false} />)
    expect(container.querySelector('span')).toBeInTheDocument()
  })

  it('renders PanelLeft icon when isOpen is false', () => {
    const { container } = render(<SidebarToggleIcon isOpen={false} />)
    const svg = container.querySelector('svg')
    expect(svg).toBeInTheDocument()
  })

  it('renders PanelLeftClose icon when isOpen is true', () => {
    const { container } = render(<SidebarToggleIcon isOpen={true} />)
    const svg = container.querySelector('svg')
    expect(svg).toBeInTheDocument()
  })

  it('applies custom className when provided', () => {
    const { container } = render(
      <SidebarToggleIcon isOpen={false} className='custom-class' />
    )
    const span = container.querySelector('span')
    expect(span?.className).toContain('custom-class')
  })

  it('applies default size', () => {
    const { container } = render(<SidebarToggleIcon isOpen={false} />)
    const svg = container.querySelector('svg')
    expect(svg).toHaveAttribute('width', '20')
    expect(svg).toHaveAttribute('height', '20')
  })

  it('applies custom size when provided', () => {
    const { container } = render(<SidebarToggleIcon isOpen={false} size={24} />)
    const svg = container.querySelector('svg')
    expect(svg).toHaveAttribute('width', '24')
    expect(svg).toHaveAttribute('height', '24')
  })

  it('changes icon when isOpen prop changes', () => {
    const { container, rerender } = render(<SidebarToggleIcon isOpen={false} />)
    let svg = container.querySelector('svg')
    expect(svg).toBeInTheDocument()

    rerender(<SidebarToggleIcon isOpen={true} />)
    svg = container.querySelector('svg')
    expect(svg).toBeInTheDocument()
  })
})
