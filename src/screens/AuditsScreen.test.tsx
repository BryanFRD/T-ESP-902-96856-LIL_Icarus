import { render } from '@testing-library/react'
import { describe, expect, it } from 'vitest'

import AuditScreen from '@/screens/AuditsScreen'

describe('AuditScreen', () => {
  it('renders without crashing', () => {
    const { container } = render(<AuditScreen />)
    expect(container.firstChild).toBeInTheDocument()
  })

  it('renders an empty div', () => {
    const { container } = render(<AuditScreen />)
    const div = container.firstChild as HTMLElement
    expect(div.tagName).toBe('DIV')
    expect(div.children.length).toBe(0)
  })
})
