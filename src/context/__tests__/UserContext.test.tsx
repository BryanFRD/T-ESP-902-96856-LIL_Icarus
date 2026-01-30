import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { renderHook, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { describe, expect, it, vi } from 'vitest'

import { UserRole } from '@/domain/enum/RoleEnum.ts'

import { UserContextProvider, useUserContext } from '../UserContext.tsx'

// Mock the hooks
vi.mock('@/hooks/auth/useMe', () => ({
  useMe: vi.fn(() => ({
    data: null,
    isLoading: false
  }))
}))

vi.mock('@/hooks/auth/useLogin', () => ({
  useLogin: vi.fn(() => ({
    mutateAsync: vi.fn()
  }))
}))

vi.mock('@/hooks/auth/useRegister', () => ({
  useRegister: vi.fn(() => ({
    mutateAsync: vi.fn()
  }))
}))

vi.mock('@/hooks/auth/useLogout', () => ({
  useLogout: vi.fn(() => ({
    mutateAsync: vi.fn()
  }))
}))

const createWrapper = () => {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: { retry: false },
      mutations: { retry: false }
    }
  })

  const Wrapper = ({ children }: { children: ReactNode }) => (
    <QueryClientProvider client={queryClient}>
      <UserContextProvider>{children}</UserContextProvider>
    </QueryClientProvider>
  )
  Wrapper.displayName = 'TestWrapper'

  return Wrapper
}

describe('UserContext', () => {
  describe('useUserContext', () => {
    it('should provide user context with null user when not authenticated', () => {
      const { result } = renderHook(() => useUserContext(), {
        wrapper: createWrapper()
      })

      expect(result.current.user).toBeNull()
      expect(result.current.loading).toBe(false)
      expect(result.current.isAuthenticated).toBe(false)
      expect(result.current.login).toBeDefined()
      expect(result.current.register).toBeDefined()
      expect(result.current.logout).toBeDefined()
    })

    it('should provide user context with authenticated user', async () => {
      const mockUser = {
        id: '123',
        username: 'testuser',
        firstName: 'Test',
        lastName: 'User',
        email: 'test@example.com',
        role: UserRole.CLIENT,
        createdAt: '2024-01-01T00:00:00Z',
        updatedAt: '2024-01-01T00:00:00Z'
      }

      const { useMe } = await import('@/hooks/auth/useMe.ts')
      vi.mocked(useMe).mockReturnValue({
        data: mockUser,
        isLoading: false
      } as unknown as ReturnType<typeof useMe>)

      const { result } = renderHook(() => useUserContext(), {
        wrapper: createWrapper()
      })

      await waitFor(() => {
        expect(result.current.user).toEqual(mockUser)
        expect(result.current.isAuthenticated).toBe(true)
      })
    })

    it('should show loading state when fetching user', async () => {
      const { useMe } = await import('@/hooks/auth/useMe.ts')
      vi.mocked(useMe).mockReturnValue({
        data: undefined,
        isLoading: true
      } as unknown as ReturnType<typeof useMe>)

      const { result } = renderHook(() => useUserContext(), {
        wrapper: createWrapper()
      })

      expect(result.current.loading).toBe(true)
    })
  })

  describe('context functions', () => {
    it('should have login function', () => {
      const { result } = renderHook(() => useUserContext(), {
        wrapper: createWrapper()
      })

      expect(result.current.login).toBeDefined()
      expect(typeof result.current.login).toBe('function')
    })

    it('should have register function', () => {
      const { result } = renderHook(() => useUserContext(), {
        wrapper: createWrapper()
      })

      expect(result.current.register).toBeDefined()
      expect(typeof result.current.register).toBe('function')
    })

    it('should have logout function', () => {
      const { result } = renderHook(() => useUserContext(), {
        wrapper: createWrapper()
      })

      expect(result.current.logout).toBeDefined()
      expect(typeof result.current.logout).toBe('function')
    })
  })

  describe('error handling', () => {
    it('should return default values when used outside provider in development mode', () => {
      // Mock development mode
      const originalMode = __CONFIG__.CURRENT_MODE
      Object.defineProperty(__CONFIG__, 'CURRENT_MODE', {
        value: 'development',
        writable: true,
        configurable: true
      })

      const { result } = renderHook(() => useUserContext())

      expect(result.current.user).toBeNull()
      expect(result.current.loading).toBe(false)
      expect(result.current.isAuthenticated).toBe(false)
      expect(typeof result.current.login).toBe('function')
      expect(typeof result.current.register).toBe('function')
      expect(typeof result.current.logout).toBe('function')

      // Restore original mode
      Object.defineProperty(__CONFIG__, 'CURRENT_MODE', {
        value: originalMode,
        writable: true,
        configurable: true
      })
    })
  })
})
