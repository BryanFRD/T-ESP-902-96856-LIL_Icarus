import {
  type JSX,
  type ReactNode,
  createContext,
  useContext,
  useMemo
} from 'react'

import type {
  LoginPayload,
  RegisterPayload
} from '@/domain/commonTypes/authType.ts'
import type { User } from '@/domain/commonTypes/userTypes.ts'
import { useLogin } from '@/hooks/auths/useLogin.ts'
import { useLogout } from '@/hooks/auths/useLogout.ts'
import { useMe } from '@/hooks/auths/useMe.ts'
import { useRegister } from '@/hooks/auths/useRegister'

export type UserContextType = {
  user: User | null
  loading: boolean
  isAuthenticated: boolean
  register: (payload: RegisterPayload) => Promise<boolean>
  login: (payload: LoginPayload) => Promise<boolean>
  logout: () => Promise<boolean>
}

export const UserContext = createContext<UserContextType | undefined>(undefined)

export const UserContextProvider = ({
  children
}: {
  children: ReactNode
}): JSX.Element => {
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
