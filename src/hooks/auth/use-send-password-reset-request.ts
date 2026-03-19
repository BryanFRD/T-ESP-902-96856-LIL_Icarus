import { useMutation } from '@tanstack/react-query'

import type {
  ResetPasswordPayload,
  SendResetPasswordEmailPayload
} from '@/domain/types/authentication.ts'
import {
  resetPassword,
  sendPasswordResetEmail
} from '@/service/auth-service.ts'
import { getErrorMessage } from '@/utils/api-utils.ts'

export const useSendPasswordResetRequest = () => {
  return useMutation<void, Error, SendResetPasswordEmailPayload>({
    mutationFn: sendPasswordResetEmail,
    onError: error => getErrorMessage(error, 'error.sendResetPassword.failed')
  })
}

export const useResetPassword = () => {
  return useMutation<void, Error, ResetPasswordPayload>({
    mutationFn: resetPassword,
    onError: error => getErrorMessage(error, 'error.resetPassword.failed')
  })
}
