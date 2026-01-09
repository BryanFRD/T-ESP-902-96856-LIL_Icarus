import axios, { type AxiosError } from 'axios'

import type { AxiosApiResponse } from '@/types/global'
import { ApiError } from '@/utils/apiUtils.ts'
import UnauthorizedHandler from '@/utils/unauthorizedHandler'

export const API_VERSIONS = {
  V1: 'v1'
} as const

export type ApiVersion = (typeof API_VERSIONS)[keyof typeof API_VERSIONS]

export function getAxiosClient(
  version: ApiVersion = API_VERSIONS.V1,
  baseUrl?: string
) {
  const instance = axios.create({
    baseURL: `${baseUrl ?? __CONFIG__.ICARUS_API_URL}/${version}`,
    headers: {
      'Content-Type': 'application/json',
      Accept: 'application/json'
    },
    withCredentials: true
  })

  instance.interceptors.response.use(
    response => response,
    (error: AxiosError) => {
      const status = error?.response?.status

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
