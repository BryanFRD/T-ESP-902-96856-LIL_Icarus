import axios, { type AxiosError } from 'axios'

import type { AxiosApiResponse } from '@/types/global.ts'
import { ApiError } from '@/utils/apiUtils.ts'
import UnauthorizedHandler from '@/utils/unauthorizedHandler.ts'

export const API_VERSIONS = {
  V1: 'v1'
} as const

export type ApiVersion = (typeof API_VERSIONS)[keyof typeof API_VERSIONS]

const CSRF_HEADER_NAME = 'x-csrf-token'

let csrfTokenCache: string | null = null

function getCsrfToken(): string | null {
  return csrfTokenCache
}

function storeCsrfToken(token: string): void {
  csrfTokenCache = token
}

function clearCsrfToken(): void {
  console.log('[CSRF] Clearing token from memory')
  csrfTokenCache = null
}

async function ensureCsrfToken(
  instance: ReturnType<typeof axios.create>
): Promise<string | null> {
  try {
    const response = await instance.get('/csrf')
    const token = response.data?.token
    if (token) {
      storeCsrfToken(token)
      return token
    } else {
      console.warn('[CSRF] No token in API response')
    }
  } catch (error) {
    console.warn('[CSRF] Failed to fetch CSRF token', error)
  }
  return null
}

export function getAxiosClient(
  version: ApiVersion = API_VERSIONS.V1,
  baseUrl?: string
) {
  const finalBaseUrl = baseUrl ?? __CONFIG__.ICARUS_API_URL

  const instance = axios.create({
    baseURL: `${finalBaseUrl}/${version}`,
    headers: {
      'Content-Type': 'application/json',
      Accept: 'application/json'
    },
    withCredentials: true
  })

  instance.interceptors.request.use(async config => {
    if (
      config.method &&
      ['post', 'put', 'patch', 'delete'].includes(config.method.toLowerCase())
    ) {
      let token = getCsrfToken()

      if (!token && !config.url?.includes('/csrf')) {
        token = await ensureCsrfToken(instance)
      }

      if (token) {
        config.headers.set(CSRF_HEADER_NAME, token)
      }
    }

    return config
  })

  instance.interceptors.response.use(
    response => response,
    (error: AxiosError) => {
      const status = error?.response?.status

      if (status === axios.HttpStatusCode.Forbidden) {
        const errorMessage = (error.response?.data as { error?: string })?.error
        if (errorMessage?.includes('CSRF') || errorMessage?.includes('csrf')) {
          console.warn('[CSRF] CSRF validation failed, clearing token')
          clearCsrfToken()
        }
      }

      if (status === axios.HttpStatusCode.Unauthorized) {
        if (UnauthorizedHandler.isCurrentlyRedirecting()) {
          return Promise.reject(
            new ApiError('error.redirect', 'Already redirecting to login')
          )
        }

        UnauthorizedHandler.handle()
        return Promise.reject(
          new ApiError('error.unauthorized', 'Unauthorized')
        )
      }

      const data = error?.response?.data as AxiosApiResponse

      if (
        data &&
        typeof data === 'object' &&
        'errorKey' in data &&
        'error' in data
      ) {
        const { errorKey, error } = data
        if (errorKey && error) {
          return Promise.reject(new ApiError(errorKey, error))
        }
      }

      return Promise.reject(new ApiError('error.unknown', error.message))
    }
  )

  return instance
}

export const __testUtils__ = {
  getCsrfToken: () => csrfTokenCache,
  setCsrfToken: (token: string | null) => {
    csrfTokenCache = token
  },
  clearCsrfToken: () => {
    csrfTokenCache = null
  }
}
