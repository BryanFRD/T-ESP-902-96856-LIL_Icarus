export type LoginPayload = {
  email: string
  password: string
}

export type RegisterPayload = {
  username: string
  firstName: string
  lastName: string
  email: string
  password: string
}

export type ResetPasswordPayload = {
  token: string
  newPassword: string
}

export type SendResetPasswordEmailPayload = {
  resetEmail: string
}
