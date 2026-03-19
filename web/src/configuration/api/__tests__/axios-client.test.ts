import axios, { AxiosError, AxiosHeaders } from 'axios'
import type { AxiosResponse, InternalAxiosRequestConfig } from 'axios'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { ApiError } from '@/utils/api-utils.ts'
import UnauthorizedHandler from '@/utils/unauthorized-handler.ts'

import { API_VERSIONS, __testUtils__, getAxiosClient } from '../axios-client.ts'

if (!('__CONFIG__' in globalThis)) {
  Object.defineProperty(globalThis, '__CONFIG__', {
    value: { ICARUS_API_URL: 'http://localhost:3000' },
    writable: true,
    configurable: true
  })
}

function getRequestInterceptor(client: ReturnType<typeof getAxiosClient>) {
  return (
    client.interceptors.request as unknown as {
      handlers: {
        fulfilled: (
          config: InternalAxiosRequestConfig
        ) => InternalAxiosRequestConfig | Promise<InternalAxiosRequestConfig>
      }[]
    }
  ).handlers[0]
}

function getResponseInterceptor(client: ReturnType<typeof getAxiosClient>) {
  return (
    client.interceptors.response as unknown as {
      handlers: {
        fulfilled: (response: AxiosResponse) => AxiosResponse
        rejected: (error: AxiosError) => Promise<unknown>
      }[]
    }
  ).handlers[0]
}

describe('getAxiosClient', () => {
  beforeEach(() => {
    __testUtils__.clearCsrfToken()
  })

  it('should create an axios instance with correct config', () => {
    const client = getAxiosClient(API_VERSIONS.V1, 'http://localhost:3000')
    expect(client.defaults.baseURL).toBe('http://localhost:3000/v1')
    expect(client.defaults.headers['Content-Type']).toBe('application/json')
    expect(client.defaults.headers['Accept']).toBe('application/json')
    expect(client.defaults.withCredentials).toBe(true)
  })

  describe('request interceptor - CSRF token', () => {
    it('should add CSRF token from sessionStorage to request headers for POST', async () => {
      const client = getAxiosClient(API_VERSIONS.V1, 'http://localhost:3000')
      const interceptor = getRequestInterceptor(client)

      __testUtils__.setCsrfToken('test-csrf-token-123')

      const config: InternalAxiosRequestConfig = {
        headers: new AxiosHeaders(),
        method: 'POST'
      } as InternalAxiosRequestConfig

      const result = await interceptor.fulfilled(config)

      expect(result.headers.get('x-csrf-token')).toBe('test-csrf-token-123')
    })

    it('should add CSRF token for PUT method', async () => {
      const client = getAxiosClient(API_VERSIONS.V1, 'http://localhost:3000')
      const interceptor = getRequestInterceptor(client)

      __testUtils__.setCsrfToken('test-token')

      const config: InternalAxiosRequestConfig = {
        headers: new AxiosHeaders(),
        method: 'PUT'
      } as InternalAxiosRequestConfig

      const result = await interceptor.fulfilled(config)

      expect(result.headers.get('x-csrf-token')).toBe('test-token')
    })

    it('should add CSRF token for PATCH method', async () => {
      const client = getAxiosClient(API_VERSIONS.V1, 'http://localhost:3000')
      const interceptor = getRequestInterceptor(client)

      __testUtils__.setCsrfToken('test-token')

      const config: InternalAxiosRequestConfig = {
        headers: new AxiosHeaders(),
        method: 'PATCH'
      } as InternalAxiosRequestConfig

      const result = await interceptor.fulfilled(config)

      expect(result.headers.get('x-csrf-token')).toBe('test-token')
    })

    it('should add CSRF token for DELETE method', async () => {
      const client = getAxiosClient(API_VERSIONS.V1, 'http://localhost:3000')
      const interceptor = getRequestInterceptor(client)

      __testUtils__.setCsrfToken('test-token')

      const config: InternalAxiosRequestConfig = {
        headers: new AxiosHeaders(),
        method: 'DELETE'
      } as InternalAxiosRequestConfig

      const result = await interceptor.fulfilled(config)

      expect(result.headers.get('x-csrf-token')).toBe('test-token')
    })

    it('should NOT add CSRF token for GET method', async () => {
      const client = getAxiosClient(API_VERSIONS.V1, 'http://localhost:3000')
      const interceptor = getRequestInterceptor(client)

      __testUtils__.setCsrfToken('test-token')

      const config: InternalAxiosRequestConfig = {
        headers: new AxiosHeaders(),
        method: 'GET'
      } as InternalAxiosRequestConfig

      const result = await interceptor.fulfilled(config)

      expect(result.headers.get('x-csrf-token')).toBeUndefined()
    })

    it('should handle lowercase method names', async () => {
      const client = getAxiosClient(API_VERSIONS.V1, 'http://localhost:3000')
      const interceptor = getRequestInterceptor(client)

      __testUtils__.setCsrfToken('test-token')

      const config: InternalAxiosRequestConfig = {
        headers: new AxiosHeaders(),
        method: 'post'
      } as InternalAxiosRequestConfig

      const result = await interceptor.fulfilled(config)

      expect(result.headers.get('x-csrf-token')).toBe('test-token')
    })

    it('should fetch token from /csrf endpoint when not in storage', async () => {
      const client = getAxiosClient(API_VERSIONS.V1, 'http://localhost:3000')

      vi.spyOn(client, 'get').mockResolvedValue({
        data: { token: 'new-token-from-api' },
        status: 200,
        statusText: 'OK',
        headers: {},
        config: {} as InternalAxiosRequestConfig
      })

      const interceptor = getRequestInterceptor(client)

      const config: InternalAxiosRequestConfig = {
        headers: new AxiosHeaders(),
        method: 'POST',
        url: '/some-endpoint'
      } as InternalAxiosRequestConfig

      const result = await interceptor.fulfilled(config)

      expect(client.get).toHaveBeenCalledWith('/csrf')
      expect(result.headers.get('x-csrf-token')).toBe('new-token-from-api')
      expect(__testUtils__.getCsrfToken()).toBe('new-token-from-api')
    })

    it('should not fetch token when URL is /csrf', async () => {
      const client = getAxiosClient(API_VERSIONS.V1, 'http://localhost:3000')
      const getSpy = vi.spyOn(client, 'get')

      const interceptor = getRequestInterceptor(client)

      const config: InternalAxiosRequestConfig = {
        headers: new AxiosHeaders(),
        method: 'POST',
        url: '/csrf'
      } as InternalAxiosRequestConfig

      await interceptor.fulfilled(config)

      expect(getSpy).not.toHaveBeenCalled()
    })
  })

  it('should pass through successful responses', () => {
    const client = getAxiosClient(API_VERSIONS.V1, 'http://localhost:3000')
    const interceptor = getResponseInterceptor(client)
    const response: AxiosResponse = {
      data: 'ok',
      status: 200,
      statusText: '',
      headers: {},
      config: { headers: {} } as InternalAxiosRequestConfig
    }
    const result = interceptor.fulfilled(response)
    expect(result).toBe(response)
  })

  describe('error handling', () => {
    let error: AxiosError
    let client: ReturnType<typeof getAxiosClient>
    let interceptor: ReturnType<typeof getResponseInterceptor>

    beforeEach(() => {
      client = getAxiosClient(API_VERSIONS.V1, 'http://localhost:3000')
      interceptor = getResponseInterceptor(client)
      error = {
        response: { status: axios.HttpStatusCode.Unauthorized },
        message: 'Unauthorized'
      } as AxiosError
      vi.spyOn(UnauthorizedHandler, 'isCurrentlyRedirecting').mockReturnValue(
        false
      )
      vi.spyOn(UnauthorizedHandler, 'handle').mockImplementation(() => {})
    })

    it('should clear sessionStorage on CSRF error (403)', async () => {
      __testUtils__.setCsrfToken('old-token')

      const csrfError = {
        response: {
          status: 403,
          data: { error: 'Invalid or missing CSRF token' }
        },
        message: 'Forbidden'
      } as AxiosError

      await expect(interceptor.rejected(csrfError)).rejects.toThrow(ApiError)
      expect(__testUtils__.getCsrfToken()).toBeNull()
    })

    it('should handle Unauthorized error and call UnauthorizedHandler', async () => {
      await expect(interceptor.rejected(error)).rejects.toThrow(ApiError)
      expect(UnauthorizedHandler.handle).toHaveBeenCalled()
    })

    it('should not handle Unauthorized if already redirecting', async () => {
      vi.spyOn(UnauthorizedHandler, 'isCurrentlyRedirecting').mockReturnValue(
        true
      )
      await expect(interceptor.rejected(error)).rejects.toThrow(ApiError)
      expect(UnauthorizedHandler.handle).not.toHaveBeenCalled()
    })

    it('should handle custom errorKey and error in response', async () => {
      const customError = {
        response: {
          status: 400,
          data: { errorKey: 'custom.key', error: 'Custom error' }
        },
        message: 'Bad Request'
      } as AxiosError
      await expect(interceptor.rejected(customError)).rejects.toThrow(ApiError)
    })

    it('should handle unknown error', async () => {
      const unknownError = {
        response: { status: 500 },
        message: 'Server error'
      } as AxiosError
      await expect(interceptor.rejected(unknownError)).rejects.toThrow(ApiError)
    })
  })
})
