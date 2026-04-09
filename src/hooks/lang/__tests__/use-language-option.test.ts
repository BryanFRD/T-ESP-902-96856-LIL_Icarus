import { renderHook } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => {
      const map: Record<string, string> = {
        'common:languages.english': 'English',
        'common:languages.french': 'French'
      }
      return map[key] ?? key
    }
  })
}))

describe('useLanguageOption', () => {
  it('should return language options with correct codes and labels', async () => {
    const { useLanguageOption } = await import('../use-language-option.ts')
    const { result } = renderHook(() => useLanguageOption())

    expect(result.current).toHaveLength(2)
    expect(result.current[0]).toEqual({ code: 'en', label: 'English' })
    expect(result.current[1]).toEqual({ code: 'fr', label: 'French' })
  })

  it('should have the correct structure for each option', async () => {
    const { useLanguageOption } = await import('../use-language-option.ts')
    const { result } = renderHook(() => useLanguageOption())

    for (const option of result.current) {
      expect(option).toHaveProperty('code')
      expect(option).toHaveProperty('label')
      expect(typeof option.code).toBe('string')
      expect(typeof option.label).toBe('string')
    }
  })
})
