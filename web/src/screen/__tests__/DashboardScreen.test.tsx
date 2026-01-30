import { render } from '@testing-library/react'
import { describe, expect, it } from 'vitest'

import DashboardScreen from '@/screen/DashboardScreen.tsx'

describe('DashboardScreen', () => {
  it('renders without crashing', () => {
    const { container } = render(<DashboardScreen />)
    expect(container.firstChild).toBeInTheDocument()
  })

  it('renders an empty div', () => {
    const { container } = render(<DashboardScreen />)
    const div = container.firstChild as HTMLElement
    expect(div.tagName).toBe('DIV')
    expect(div.children.length).toBe(0)
  })
})
