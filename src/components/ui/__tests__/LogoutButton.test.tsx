import { fireEvent, render, screen, waitFor } from '@testing-library/react'
import { I18nextProvider } from 'react-i18next'
import { MemoryRouter } from 'react-router-dom'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import LogoutButton from '@/components/layout/sidebar/LogoutButton.tsx'
import { UserContext } from '@/context/UserContext.tsx'
import i18n from '@/i18n'

const mockNavigate = vi.fn()
vi.mock('react-router-dom', async () => {
  const actual = await vi.importActual('react-router-dom')
  return {
    ...actual,
    useNavigate: () => mockNavigate
  }
})

vi.mock('react-hot-toast', () => ({
  default: {
    promise: vi.fn((promise, options) => {
      return promise.then(
        (result: unknown) => {
          if (options && typeof options.success === 'function') {
            options.success(result)
          }
          return result
        },
        (error: unknown) => {
          if (options && typeof options.error === 'function') {
            options.error(error)
          }
          throw error
        }
      )
    })
  }
}))

describe('LogoutButton components', () => {
  const mockLogout = vi.fn()

  const mockUserContextValue = {
    logout: mockLogout,
    user: null,
    setUser: vi.fn(),
    isLoading: false,
    updateUser: vi.fn(),
    fetchUser: vi.fn(),
    isUserLoading: false,
    loading: false,
    isAuthenticated: false,
    register: vi.fn(),
    login: vi.fn()
  }

  const renderWithProviders = (component: React.ReactElement) => {
    return render(
      <MemoryRouter>
        <I18nextProvider i18n={i18n}>
          <UserContext.Provider value={mockUserContextValue}>
            {component}
          </UserContext.Provider>
        </I18nextProvider>
      </MemoryRouter>
    )
  }

  beforeEach(() => {
    vi.clearAllMocks()
    mockLogout.mockResolvedValue(undefined)
  })

  it('renders logout button with icon and text', () => {
    renderWithProviders(<LogoutButton />)

    const button = screen.getByRole('button')
    expect(button).toBeInTheDocument()

    const svg = button.querySelector('svg')
    expect(svg).toBeInTheDocument()
  })

  it('calls logout function when clicked', async () => {
    renderWithProviders(<LogoutButton />)

    const button = screen.getByRole('button')
    fireEvent.click(button)

    await waitFor(() => {
      expect(mockLogout).toHaveBeenCalledTimes(1)
    })
  })

  it('navigates to home page after successful logout', async () => {
    renderWithProviders(<LogoutButton />)

    const button = screen.getByRole('button')
    fireEvent.click(button)

    await waitFor(() => {
      expect(mockLogout).toHaveBeenCalled()
      expect(mockNavigate).toHaveBeenCalledWith('/')
    })
  })

  it('applies correct button variant and styling', () => {
    renderWithProviders(<LogoutButton />)

    const button = screen.getByRole('button')
    expect(button.className).toContain('gap-4')
  })
})
