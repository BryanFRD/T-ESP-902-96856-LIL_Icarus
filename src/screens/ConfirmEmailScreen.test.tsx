import { render, screen, waitFor } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import ConfirmEmailScreen from '@/screens/ConfirmEmailScreen'

const mockNavigate = vi.fn()
const mockMutateAsync = vi.fn()

vi.mock('react-router-dom', async () => {
  const actual = await vi.importActual('react-router-dom')
  return {
    ...actual,
    useNavigate: () => mockNavigate,
    useLocation: () => ({
      search: '?token=valid-token-123'
    })
  }
})

vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => key
  })
}))

vi.mock('@/hooks/auths/useConfirmEmail.ts', () => ({
  useConfirmEmail: () => ({
    mutateAsync: mockMutateAsync
  })
}))

vi.mock('@/utils/stringUtils.ts', () => ({
  isNull: (value: string | null) => value === null,
  validToken: (token: string) => token === 'valid-token-123'
}))

vi.mock('@/routes/routesName.ts', () => ({
  ROUTES: { LOGIN: 'LOGIN' },
  getRoutePath: () => '/login'
}))

describe('ConfirmEmailScreen', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('renders loading state initially', () => {
    mockMutateAsync.mockImplementation(
      () => new Promise(() => {}) // Never resolves
    )
    render(<ConfirmEmailScreen />)
    expect(screen.getByText('confirmEmail.verifying')).toBeInTheDocument()
    expect(screen.getByText('confirmEmail.pleaseWait')).toBeInTheDocument()
  })

  it('renders success state after successful confirmation', async () => {
    mockMutateAsync.mockResolvedValue({})
    render(<ConfirmEmailScreen />)

    await waitFor(() => {
      expect(screen.getByText('confirmEmail.success')).toBeInTheDocument()
    })

    expect(screen.getByText('confirmEmail.successMessage')).toBeInTheDocument()
  })

  it('renders error state when confirmation fails', async () => {
    mockMutateAsync.mockRejectedValue(new Error('Confirmation failed'))
    render(<ConfirmEmailScreen />)

    await waitFor(() => {
      expect(screen.getByText('confirmEmail.error')).toBeInTheDocument()
    })
  })

  it('navigates to login after delay on success', async () => {
    mockMutateAsync.mockResolvedValue({})
    render(<ConfirmEmailScreen />)

    await waitFor(() => {
      expect(screen.getByText('confirmEmail.success')).toBeInTheDocument()
    })

    await waitFor(
      () => {
        expect(mockNavigate).toHaveBeenCalledWith('/login')
      },
      { timeout: 4000 }
    )
  })

  it('navigates to login after delay on error', async () => {
    mockMutateAsync.mockRejectedValue(new Error('Confirmation failed'))
    render(<ConfirmEmailScreen />)

    await waitFor(() => {
      expect(screen.getByText('confirmEmail.error')).toBeInTheDocument()
    })

    await waitFor(
      () => {
        expect(mockNavigate).toHaveBeenCalledWith('/login')
      },
      { timeout: 4000 }
    )
  })

  it('renders loading icon in loading state', () => {
    mockMutateAsync.mockImplementation(
      () => new Promise(() => {}) // Never resolves
    )
    const { container } = render(<ConfirmEmailScreen />)
    const loader = container.querySelector('.animate-spin')
    expect(loader).toBeInTheDocument()
  })

  it('renders success icon in success state', async () => {
    mockMutateAsync.mockResolvedValue({})
    const { container } = render(<ConfirmEmailScreen />)

    await waitFor(() => {
      expect(screen.getByText('confirmEmail.success')).toBeInTheDocument()
    })

    const checkCircle = container.querySelector('.text-green-600')
    expect(checkCircle).toBeInTheDocument()
  })

  it('renders error icon in error state', async () => {
    mockMutateAsync.mockRejectedValue(new Error('Confirmation failed'))
    const { container } = render(<ConfirmEmailScreen />)

    await waitFor(() => {
      expect(screen.getByText('confirmEmail.error')).toBeInTheDocument()
    })

    const xCircle = container.querySelector('.text-red-600')
    expect(xCircle).toBeInTheDocument()
  })
})
