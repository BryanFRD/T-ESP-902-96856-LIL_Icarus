import { render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import ProfileScreen from '@/screens/ProfileScreen'

const mockUser = {
  id: '1',
  username: 'testuser',
  email: 'test@example.com',
  firstName: 'Test',
  lastName: 'User',
  role: 'user',
  createdAt: '2024-01-01T00:00:00Z',
  updatedAt: '2024-01-15T12:00:00Z'
}

vi.mock('i18next', () => ({
  t: (key: string) => key
}))

vi.mock('@/context/UserContext', () => ({
  useUserContext: () => ({
    user: mockUser
  })
}))

vi.mock('@/hooks/upload/useAvatar', () => ({
  useAvatar: () => ({
    handleAvatarSelect: vi.fn(),
    cropModalOpen: false,
    selectedImage: null,
    handleCropComplete: vi.fn(),
    handleCropCancel: vi.fn(),
    currentAvatarUrl: 'https://example.com/avatar.jpg',
    fileUploadKey: 0
  })
}))

vi.mock('@/utils/dateFormatter', () => ({
  useDateFormatter: () => ({
    dateLongTimeNative: (date: string) => `Long Time: ${date}`,
    dateLong: (date: string) => `Long: ${date}`
  })
}))

vi.mock('@/components/ImageCropModal', () => ({
  default: () => null
}))

vi.mock('@/components/profile/ProfileForm', () => ({
  default: () => <div data-testid='profile-form'>Profile Form</div>
}))

vi.mock('@/components/ui/upload/FileUpload', () => ({
  default: () => <div data-testid='file-upload'>File Upload</div>
}))

describe('ProfileScreen', () => {
  it('renders without crashing', () => {
    render(<ProfileScreen />)
    expect(screen.getByTestId('profile-form')).toBeInTheDocument()
  })

  it('renders ProfileForm component', () => {
    render(<ProfileScreen />)
    expect(screen.getByTestId('profile-form')).toBeInTheDocument()
  })

  it('renders FileUpload component', () => {
    render(<ProfileScreen />)
    expect(screen.getByTestId('file-upload')).toBeInTheDocument()
  })

  it('displays profile details section', () => {
    render(<ProfileScreen />)
    expect(screen.getByText('profileScreen.profileDetails')).toBeInTheDocument()
  })

  it('displays created on date', () => {
    render(<ProfileScreen />)
    expect(screen.getByText('profileScreen.createdOn')).toBeInTheDocument()
    expect(screen.getByText('Long: 2024-01-01T00:00:00Z')).toBeInTheDocument()
  })

  it('displays last updated date', () => {
    render(<ProfileScreen />)
    expect(screen.getByText('profileScreen.lastUpdated')).toBeInTheDocument()
    expect(
      screen.getByText('Long Time: 2024-01-15T12:00:00Z')
    ).toBeInTheDocument()
  })

  it('has correct grid layout', () => {
    const { container } = render(<ProfileScreen />)
    const gridContainer = container.querySelector('.grid')
    expect(gridContainer).toBeInTheDocument()
    expect(gridContainer?.className).toContain('lg:grid-cols-3')
  })
})
