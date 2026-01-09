import axios, { AxiosError } from 'axios'
import type { AxiosResponse, InternalAxiosRequestConfig } from 'axios'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { ApiError } from '@/utils/apiUtils'
import UnauthorizedHandler from '@/utils/unauthorizedHandler'

import { API_VERSIONS, getAxiosClient } from './axiosClient'

if (!('__CONFIG__' in globalThis)) {
  Object.defineProperty(globalThis, '__CONFIG__', {
    value: { ICARUS_API_URL: 'http://localhost:3000' },
    writable: true,
    configurable: true
  })
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
  it('should create an axios instance with correct config', () => {
    const client = getAxiosClient(API_VERSIONS.V1, 'http://localhost:3000')
    expect(client.defaults.baseURL).toBe('http://localhost:3000/v1')
    expect(client.defaults.headers['Content-Type']).toBe('application/json')
    expect(client.defaults.headers['Accept']).toBe('application/json')
    expect(client.defaults.withCredentials).toBe(true)
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
