import { render, screen } from '@testing-library/react'
import { Shield } from 'lucide-react'
import { describe, expect, it } from 'vitest'

import FeatureItem from '@/components/login/FeatureItem'

describe('FeatureItem component', () => {
  it('renders without crashing', () => {
    render(
      <FeatureItem
        icon={Shield}
        title='Security'
        description='Secure authentication'
      />
    )
    expect(screen.getByText('Security')).toBeInTheDocument()
  })

  it('renders title correctly', () => {
    render(
      <FeatureItem
        icon={Shield}
        title='Test Title'
        description='Test description'
      />
    )
    expect(screen.getByText('Test Title')).toBeInTheDocument()
  })

  it('renders description correctly', () => {
    render(
      <FeatureItem
        icon={Shield}
        title='Test Title'
        description='Test description'
      />
    )
    expect(screen.getByText('Test description')).toBeInTheDocument()
  })

  it('renders icon component', () => {
    const { container } = render(
      <FeatureItem
        icon={Shield}
        title='Security'
        description='Secure authentication'
      />
    )
    const svg = container.querySelector('svg')
    expect(svg).toBeInTheDocument()
  })

  it('has correct structure with icon and text', () => {
    const { container } = render(
      <FeatureItem
        icon={Shield}
        title='Security'
        description='Secure authentication'
      />
    )
    const mainDiv = container.firstChild as HTMLElement
    expect(mainDiv.className).toContain('flex')
    expect(mainDiv.className).toContain('items-center')
    expect(mainDiv.className).toContain('gap-4')
  })
})
