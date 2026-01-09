import { zodResolver } from '@hookform/resolvers/zod'
import { ArrowLeft, ArrowRight, Lock, Mail, User } from 'lucide-react'
import { useState } from 'react'
import { useForm } from 'react-hook-form'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'
import { Link, useNavigate } from 'react-router-dom'

import Button from '@/components/ui/Button'
import InputField from '@/components/ui/InputField'
import Stepper from '@/components/ui/Stepper'
import { useUserContext } from '@/context/UserContext'
import {
  type RegisterSchemaType,
  getRegisterSchema
} from '@/domain/validation/registerSchema'
import { ROUTES, getRoutePath } from '@/routes/routesName.ts'
import type { ApiError } from '@/utils/apiUtils'

import RegisterHeader from './RegisterHeader'

type Step = 1 | 2 | 3

const RegisterStepForm = () => {
  const { register: registerUser } = useUserContext()
  const navigate = useNavigate()
  const { t } = useTranslation()
  const [isLoading, setIsLoading] = useState(false)
  const [currentStep, setCurrentStep] = useState<Step>(1)

  const {
    register,
    handleSubmit,
    trigger,
    formState: { errors }
  } = useForm<RegisterSchemaType>({
    resolver: zodResolver(getRegisterSchema(t)),
    mode: 'onBlur'
  })

  const steps = [
    {
      number: 1,
      title: t('register.step.personal'),
      icon: User
    },
    {
      number: 2,
      title: t('register.step.account'),
      icon: Mail
    },
    {
      number: 3,
      title: t('register.step.security'),
      icon: Lock
    }
  ]

  const handleNext = async () => {
    let fieldsToValidate: (keyof RegisterSchemaType)[] = []

    if (currentStep === 1) {
      fieldsToValidate = ['firstName', 'lastName']
    } else if (currentStep === 2) {
      fieldsToValidate = ['username', 'email']
    }

    const isValid = await trigger(fieldsToValidate)

    if (isValid) {
      setCurrentStep(prev => Math.min(prev + 1, 3) as Step)
    }
  }

  const handlePrevious = () => {
    setCurrentStep(prev => Math.max(prev - 1, 1) as Step)
  }

  const onSubmit = async (data: RegisterSchemaType) => {
    setIsLoading(true)
    try {
      const success = await toast.promise(
        registerUser({
          username: data.username,
          email: data.email,
          firstName: data.firstName,
          lastName: data.lastName,
          password: data.password
        }),
        {
          loading: t('loading.register'),
          success: () => t('success.register'),
          error: (err: ApiError) => t(err.errorKey)
        }
      )
      if (success) {
        navigate(getRoutePath(ROUTES.LOGIN))
      }
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <div className='mx-auto w-full max-w-2xl px-4 py-10'>
      <RegisterHeader />

      <div className='mb-10'>
        <Stepper steps={steps} currentStep={currentStep} />
      </div>

      <form onSubmit={handleSubmit(onSubmit)} className='space-y-6'>
        {currentStep === 1 && (
          <div className='animate-in fade-in slide-in-from-right-4 space-y-6 duration-300'>
            <InputField
              label={t('inputField.firstName')}
              type='text'
              name='firstName'
              icon={User}
              placeholder={t('inputField.firstNamePlaceholder')}
              register={register}
              error={errors.firstName?.message}
              autoComplete='given-name'
            />

            <InputField
              label={t('inputField.lastName')}
              type='text'
              name='lastName'
              icon={User}
              placeholder={t('inputField.lastNamePlaceholder')}
              register={register}
              error={errors.lastName?.message}
              autoComplete='family-name'
            />
          </div>
        )}

        {currentStep === 2 && (
          <div className='animate-in fade-in slide-in-from-right-4 space-y-6 duration-300'>
            <InputField
              label={t('inputField.username')}
              type='text'
              name='username'
              icon={User}
              placeholder={t('inputField.usernamePlaceholder')}
              register={register}
              error={errors.username?.message}
              autoComplete='username'
            />

            <InputField
              label={t('inputField.email')}
              type='email'
              name='email'
              icon={Mail}
              placeholder={t('inputField.emailPlaceholder')}
              register={register}
              error={errors.email?.message}
              autoComplete='email'
            />
          </div>
        )}

        {currentStep === 3 && (
          <div className='animate-in fade-in slide-in-from-right-4 space-y-6 duration-300'>
            <InputField
              label={t('inputField.password')}
              type='password'
              name='password'
              icon={Lock}
              placeholder={t('inputField.passwordPlaceholder')}
              register={register}
              error={errors.password?.message}
              autoComplete='new-password'
            />

            <InputField
              label={t('inputField.confirmPassword')}
              type='password'
              name='confirmPassword'
              icon={Lock}
              placeholder={t('inputField.confirmPasswordPlaceholder')}
              register={register}
              error={errors.confirmPassword?.message}
              autoComplete='new-password'
            />
          </div>
        )}

        <div className='mt-8 flex items-center justify-between pt-4'>
          {currentStep > 1 ? (
            <Button
              icon={{ type: ArrowLeft, position: 'left' }}
              type='button'
              variant='secondary'
              onClick={handlePrevious}
              size='sm'
              className='px-6'
            >
              {t('actions.previous')}
            </Button>
          ) : (
            <div />
          )}

          {currentStep < 3 ? (
            <Button
              icon={{ type: ArrowRight, position: 'right' }}
              type='button'
              variant='primary'
              onClick={handleNext}
              size='sm'
              className='px-6'
            >
              {t('actions.next')}
            </Button>
          ) : (
            <Button
              type='submit'
              variant='primary'
              isLoading={isLoading}
              loadingText={t('inputField.signingUp')}
              size='sm'
              className='px-6'
            >
              {t('inputField.signUp')}
            </Button>
          )}
        </div>
      </form>

      <div className='mt-8 text-center'>
        <p className='flex items-center justify-center gap-1 text-gray-600 transition-all duration-300 dark:text-gray-400'>
          {t('inputField.haveAccount')}
          <Link
            to='/'
            className='font-semibold text-orange-600 transition-colors hover:text-orange-700 dark:text-orange-400 dark:hover:text-orange-300'
          >
            {t('inputField.signInHere')}
          </Link>
        </p>
      </div>
    </div>
  )
}

export default RegisterStepForm
