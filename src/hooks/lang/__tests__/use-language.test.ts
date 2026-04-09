import { act, renderHook } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

const mockChangeLanguage = vi.fn()

vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    i18n: {
      language: 'en',
      changeLanguage: mockChangeLanguage
    }
  })
}))

describe('useLanguage', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should return the current language', async () => {
    const { default: useLanguage } = await import('../use-language.ts')
    const { result } = renderHook(() => useLanguage())

    expect(result.current.currentLanguage).toBe('en')
  })

  it('should call i18n.changeLanguage when changing language', async () => {
    const { default: useLanguage } = await import('../use-language.ts')
    const { result } = renderHook(() => useLanguage())

    act(() => {
      result.current.changeLanguage('fr')
    })

    expect(mockChangeLanguage).toHaveBeenCalledWith('fr')
  })
})
