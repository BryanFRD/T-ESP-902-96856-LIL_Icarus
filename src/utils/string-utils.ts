export const isNull = <T>(
  value: T | null | undefined
): value is null | undefined => {
  return value === null || value === undefined
}

export const isNotNull = <T>(value: T | null | undefined): value is T => {
  return !isNull(value)
}

export const isEmptyString = (str: string | null | undefined): boolean => {
  return !str || str.trim() === ''
}

export const validToken = (token: string | null | undefined): boolean => {
  if (!token) return false
  const trimmed = token.trim()
  if (trimmed === '') return false
  const pattern = /^[A-Za-z0-9-_]+(?:\.[A-Za-z0-9-_]+){0,2}$/
  return pattern.test(trimmed)
}
