import { useMutation } from '@tanstack/react-query'

import type {
  ResetPasswordPayload,
  SendResetPasswordEmailPayload
} from '@/domain/types/Authentication.ts'
import { resetPassword, sendPasswordResetEmail } from '@/service/authService.ts'
import { getErrorMessage } from '@/utils/apiUtils.ts'

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
