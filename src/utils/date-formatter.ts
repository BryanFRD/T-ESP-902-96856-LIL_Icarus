import { useTranslation } from 'react-i18next'

const TIME_SEPARATORS: Record<string, string> = {
  fr: 'à',
  en: 'at'
}

const getTimeSeparator = (lang: string): string => {
  const langCode = lang.split('-')[0]
  return TIME_SEPARATORS[langCode] || TIME_SEPARATORS.en
}

export const formatter = (lang: string) => ({
  /*
   * Formats date in a short format
   * Example: "01/01/2023"
   */
  date: (date: Date | string) => {
    return new Intl.DateTimeFormat(lang, {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric'
    }).format(new Date(date))
  },

  /*
   * Formats date in a long format
   * Example: "January 1, 2023"
   */
  dateLong: (date: Date | string) => {
    return new Intl.DateTimeFormat(lang, {
      day: 'numeric',
      month: 'long',
      year: 'numeric'
    }).format(new Date(date))
  },

  /*
   * Formats time in a short format
   * Example: "12:00"
   */
  time: (date: Date | string) => {
    return new Intl.DateTimeFormat(lang, {
      hour: '2-digit',
      minute: '2-digit'
    }).format(new Date(date))
  },

  /*
   * Formats date and time in a short format
   * Example: "01/01/2023 12:00"
   */
  dateTime: (date: Date | string) => {
    const d = new Date(date)
    const dateStr = new Intl.DateTimeFormat(lang, {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric'
    }).format(d)

    const timeStr = new Intl.DateTimeFormat(lang, {
      hour: '2-digit',
      minute: '2-digit'
    }).format(d)

    const separator = getTimeSeparator(lang)
    return `${dateStr} ${separator} ${timeStr}`
  },

  /*
   * Formats date in a short format
   * Example: "01/01/2023"
   */
  dateTimeNative: (date: Date | string) => {
    return new Intl.DateTimeFormat(lang, {
      dateStyle: 'short',
      timeStyle: 'short'
    }).format(new Date(date))
  },

  /*
   * Formats date and time in a long format
   * Example: "January 1, 2023 at 12:00 PM"
   */
  dateLongTimeNative: (date: Date | string) => {
    const d = new Date(date)
    const dateStr = new Intl.DateTimeFormat(lang, {
      day: 'numeric',
      month: 'long',
      year: 'numeric'
    }).format(d)
    const timeStr = new Intl.DateTimeFormat(lang, {
      hour: '2-digit',
      minute: '2-digit'
    }).format(d)
    const separator = getTimeSeparator(lang)
    return `${dateStr} ${separator} ${timeStr}`
  }
})

export const useDateFormatter = () => {
  const { i18n } = useTranslation()
  return formatter(i18n.language)
}
