import { createContext, useContext, useMemo } from 'react'

import type {
  LoginPayload,
  RegisterPayload
} from '@/domain/types/authentication.ts'
import type { User } from '@/domain/types/user.ts'
import { useLogin } from '@/hooks/auth/use-login.ts'
import { useLogout } from '@/hooks/auth/use-logout.ts'
import { useMe } from '@/hooks/auth/use-me.ts'
import { useRegister } from '@/hooks/auth/use-register.ts'

export type UserContextType = {
  user: User | null
  loading: boolean
  isAuthenticated: boolean
  register: (payload: RegisterPayload) => Promise<boolean>
  login: (payload: LoginPayload) => Promise<boolean>
  logout: () => Promise<boolean>
}

export const UserContext = createContext<UserContextType | undefined>(undefined)

export const UserContextProvider = ({ children }: React.PropsWithChildren) => {
  const { data: user, isLoading } = useMe()
  const { mutateAsync: mutateLogin } = useLogin()
  const { mutateAsync: mutateRegister } = useRegister()
  const { mutateAsync: mutateLogout } = useLogout()

  const login = async (payload: LoginPayload) => {
    await mutateLogin(payload)
    return true
  }

  const register = async (payload: RegisterPayload) => {
    await mutateRegister(payload)
    return true
  }

  const logout = async () => {
    return await mutateLogout()
  }

  const contextValue = useMemo(
    () => ({
      user: user ?? null,
      loading: isLoading,
      isAuthenticated: !!user,
      register,
      login,
      logout
    }),
    [user, isLoading, register, login, logout]
  )

  return (
    <UserContext.Provider value={contextValue}>{children}</UserContext.Provider>
  )
}

export const useUserContext = (): UserContextType => {
  const context = useContext(UserContext)
  if (!context) {
    if (__CONFIG__.CURRENT_MODE !== 'production') {
      //! Hot reloading issue workaround
      return {
        user: null,
        loading: false,
        isAuthenticated: false,
        register: async () => false,
        login: async () => false,
        logout: async () => false
      }
    }

    throw new Error('useUserContext must be used within a UserContextProvider')
  }
  return context
}
