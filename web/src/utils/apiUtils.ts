export class ApiError extends Error {
  errorKey: string
  error: string

  constructor(errorKey: string, error: string) {
    super(error)
    this.name = 'ApiError'
    this.errorKey = errorKey
    this.error = error

    console.debug(`[ApiError] ${errorKey}: ${error}`)
  }

  toJSON() {
    return {
      name: this.name,
      errorKey: this.errorKey,
      error: this.error,
      message: this.message,
      stack: this.stack
    }
  }
}

export function getErrorMessage(
  err: unknown,
  fallback = 'error.fallback'
): string {
  if (typeof err === 'string') {
    return err
  }

  if (err instanceof ApiError) {
    return err.errorKey
  }

  return fallback
}
