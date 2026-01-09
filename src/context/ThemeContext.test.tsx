import { act, renderHook } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { ThemeProvider, useTheme } from './ThemeContext'

describe('ThemeContext', () => {
  beforeEach(() => {
    // Clear localStorage before each test
    localStorage.clear()
    // Clear document classes
    document.documentElement.classList.remove('dark', 'light')
    // Reset matchMedia mock
    Object.defineProperty(globalThis, 'matchMedia', {
      writable: true,
      value: vi.fn().mockImplementation(query => ({
        matches: false,
        media: query,
        onchange: null,
        addListener: vi.fn(),
        removeListener: vi.fn(),
        addEventListener: vi.fn(),
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn()
      }))
    })
  })

  describe('useTheme hook', () => {
    it('should provide theme context when used within ThemeProvider', () => {
      const { result } = renderHook(() => useTheme(), {
        wrapper: ThemeProvider
      })

      expect(result.current.theme).toBeDefined()
      expect(result.current.toggleTheme).toBeDefined()
      expect(result.current.setTheme).toBeDefined()
    })
  })

  describe('theme initialization', () => {
    it('should initialize with light theme by default', () => {
      const { result } = renderHook(() => useTheme(), {
        wrapper: ThemeProvider
      })

      expect(result.current.theme).toBe('light')
      expect(document.documentElement.classList.contains('light')).toBe(true)
    })

    it('should initialize with dark theme when prefers-color-scheme is dark', () => {
      Object.defineProperty(globalThis, 'matchMedia', {
        writable: true,
        value: vi.fn().mockImplementation(query => ({
          matches: query === '(prefers-color-scheme: dark)',
          media: query,
          onchange: null,
          addListener: vi.fn(),
          removeListener: vi.fn(),
          addEventListener: vi.fn(),
          removeEventListener: vi.fn(),
          dispatchEvent: vi.fn()
        }))
      })

      const { result } = renderHook(() => useTheme(), {
        wrapper: ThemeProvider
      })

      expect(result.current.theme).toBe('dark')
    })

    it('should initialize with saved theme from localStorage', () => {
      localStorage.setItem('theme', 'dark')

      const { result } = renderHook(() => useTheme(), {
        wrapper: ThemeProvider
      })

      expect(result.current.theme).toBe('dark')
      expect(document.documentElement.classList.contains('dark')).toBe(true)
    })

    it('should prioritize localStorage over system preference', () => {
      localStorage.setItem('theme', 'light')
      Object.defineProperty(globalThis, 'matchMedia', {
        writable: true,
        value: vi.fn().mockImplementation(query => ({
          matches: query === '(prefers-color-scheme: dark)',
          media: query,
          onchange: null,
          addListener: vi.fn(),
          removeListener: vi.fn(),
          addEventListener: vi.fn(),
          removeEventListener: vi.fn(),
          dispatchEvent: vi.fn()
        }))
      })

      const { result } = renderHook(() => useTheme(), {
        wrapper: ThemeProvider
      })

      expect(result.current.theme).toBe('light')
    })
  })

  describe('toggleTheme', () => {
    it('should toggle from light to dark', () => {
      const { result } = renderHook(() => useTheme(), {
        wrapper: ThemeProvider
      })

      expect(result.current.theme).toBe('light')

      act(() => {
        result.current.toggleTheme()
      })

      expect(result.current.theme).toBe('dark')
      expect(document.documentElement.classList.contains('dark')).toBe(true)
      expect(document.documentElement.classList.contains('light')).toBe(false)
      expect(localStorage.getItem('theme')).toBe('dark')
    })

    it('should toggle from dark to light', () => {
      localStorage.setItem('theme', 'dark')

      const { result } = renderHook(() => useTheme(), {
        wrapper: ThemeProvider
      })

      expect(result.current.theme).toBe('dark')

      act(() => {
        result.current.toggleTheme()
      })

      expect(result.current.theme).toBe('light')
      expect(document.documentElement.classList.contains('light')).toBe(true)
      expect(document.documentElement.classList.contains('dark')).toBe(false)
      expect(localStorage.getItem('theme')).toBe('light')
    })

    it('should toggle multiple times', () => {
      const { result } = renderHook(() => useTheme(), {
        wrapper: ThemeProvider
      })

      act(() => {
        result.current.toggleTheme()
      })
      expect(result.current.theme).toBe('dark')

      act(() => {
        result.current.toggleTheme()
      })
      expect(result.current.theme).toBe('light')

      act(() => {
        result.current.toggleTheme()
      })
      expect(result.current.theme).toBe('dark')
    })
  })

  describe('setTheme', () => {
    it('should set theme to dark', () => {
      const { result } = renderHook(() => useTheme(), {
        wrapper: ThemeProvider
      })

      act(() => {
        result.current.setTheme('dark')
      })

      expect(result.current.theme).toBe('dark')
      expect(document.documentElement.classList.contains('dark')).toBe(true)
      expect(localStorage.getItem('theme')).toBe('dark')
    })

    it('should set theme to light', () => {
      localStorage.setItem('theme', 'dark')

      const { result } = renderHook(() => useTheme(), {
        wrapper: ThemeProvider
      })

      act(() => {
        result.current.setTheme('light')
      })

      expect(result.current.theme).toBe('light')
      expect(document.documentElement.classList.contains('light')).toBe(true)
      expect(localStorage.getItem('theme')).toBe('light')
    })
  })

  describe('DOM and localStorage sync', () => {
    it('should update document classes when theme changes', () => {
      const { result } = renderHook(() => useTheme(), {
        wrapper: ThemeProvider
      })

      expect(document.documentElement.classList.contains('light')).toBe(true)

      act(() => {
        result.current.setTheme('dark')
      })

      expect(document.documentElement.classList.contains('dark')).toBe(true)
      expect(document.documentElement.classList.contains('light')).toBe(false)
    })

    it('should persist theme to localStorage when changed', () => {
      const { result } = renderHook(() => useTheme(), {
        wrapper: ThemeProvider
      })

      act(() => {
        result.current.setTheme('dark')
      })

      expect(localStorage.getItem('theme')).toBe('dark')

      act(() => {
        result.current.setTheme('light')
      })

      expect(localStorage.getItem('theme')).toBe('light')
    })
  })
})
