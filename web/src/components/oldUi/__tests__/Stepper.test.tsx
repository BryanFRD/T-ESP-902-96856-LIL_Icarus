import { render, screen } from '@testing-library/react'
import { Check } from 'lucide-react'
import { describe, expect, it } from 'vitest'

import Stepper from '../Stepper.tsx'

const steps = [
  { number: 1, title: 'Step 1', icon: Check },
  { number: 2, title: 'Step 2', icon: Check },
  { number: 3, title: 'Step 3', icon: Check }
]

describe('Stepper', () => {
  it('renders all step labels', () => {
    render(<Stepper steps={steps} currentStep={2} />)
    expect(screen.getByText('Step 1')).toBeInTheDocument()
    expect(screen.getByText('Step 2')).toBeInTheDocument()
    expect(screen.getByText('Step 3')).toBeInTheDocument()
  })

  it('applies correct status classes for current step', () => {
    render(<Stepper steps={steps} currentStep={2} />)
    const currentLabel = screen.getByText('Step 2')
    expect(currentLabel.className).toContain('text-orange-600')
    expect(currentLabel.className).toContain('font-bold')
  })

  it('shows completed icon for completed steps', () => {
    render(<Stepper steps={steps} currentStep={3} />)
    // Step 1 and 2 should be completed
    const completedLabels = [
      screen.getByText('Step 1'),
      screen.getByText('Step 2')
    ]
    completedLabels.forEach(label => {
      expect(label.className).toContain('text-gray-700')
    })
  })

  it('renders the correct number of step circles', () => {
    const { container } = render(<Stepper steps={steps} currentStep={1} />)
    expect(container.querySelectorAll('svg').length).toBe(3)
  })
})
