import { zodResolver } from '@hookform/resolvers/zod'
import { ArrowLeft, ArrowRight, Lock, Mail, User } from 'lucide-react'
import { useState } from 'react'
import { Controller, useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { z } from 'zod'

import {
  Stepper,
  StepperContent,
  StepperIndicator,
  StepperItem,
  StepperNav,
  StepperPanel,
  StepperSeparator,
  StepperTitle,
  StepperTrigger
} from '@/components/ui/Stepper'
import { Button } from '@/components/ui/button'
import {
  Field,
  FieldError,
  FieldGroup,
  FieldLabel
} from '@/components/ui/field'
import { Input } from '@/components/ui/input'
import { register } from '@/service/auth-service.ts'

const formSchema = z
  .object({
    firstName: z.string().min(1, 'First name is required'),
    lastName: z.string().min(1, 'Last name is required'),
    username: z.string().min(3, 'Username must be at least 3 characters'),
    email: z.string().email('Invalid email address'),
    password: z.string().min(6, 'Password must be at least 6 characters'),
    confirmPassword: z
      .string()
      .min(6, 'Confirm password must be at least 6 characters')
  })
  .refine(data => data.password === data.confirmPassword, {
    message: 'Passwords do not match'
  })

type FormSchema = z.infer<typeof formSchema>

export function RegisterStepperForm() {
  const { t } = useTranslation('auth')

  const [currentStep, setCurrentStep] = useState<number>(1)

  const steps = [
    {
      title: t('register.form.stepper.personal'),
      icon: <User className='size-4' />
    },
    {
      title: t('register.form.stepper.account'),
      icon: <Mail className='size-4' />
    },
    {
      title: t('register.form.stepper.security'),
      icon: <Lock className='size-4' />
    }
  ]

  const form = useForm<FormSchema>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      firstName: '',
      lastName: '',
      username: '',
      email: ''
    },
    mode: 'onChange'
  })

  const stepFields: Record<number, (keyof FormSchema)[]> = {
    1: ['firstName', 'lastName'],
    2: ['username', 'email'],
    3: ['password', 'confirmPassword']
  }

  const handleNext = async () => {
    const fields = stepFields[currentStep]
    const isValid = await form.trigger(fields)
    if (!isValid) return

    if (currentStep === steps.length) {
      form.handleSubmit(onSubmit)()
    } else {
      setCurrentStep(step => step + 1)
    }
  }

  const onSubmit = async (data: FormSchema) => {
    try {
      await register(data)
    } catch {
      toast.error(t('register.form.toast.error'))
    } finally {
      toast.success(t('register.form.toast.success'))
    }
  }

  return (
    <Stepper
      value={currentStep}
      onValueChange={setCurrentStep}
      className='w-full space-y-8'
    >
      <StepperNav className='gap-x-3'>
        {steps.map((step, index) => (
          <StepperItem key={index} step={index + 1} className='relative'>
            <StepperTrigger
              className='flex grow flex-col items-start justify-center gap-2.5'
              asChild
            >
              <StepperIndicator className='data-[state=inactive]:border-border data-[state=inactive]:text-muted-foreground data-[state=completed]:bg-primary data-[state=completed]:text-primary-foreground size-8 border-2 data-[state=inactive]:bg-transparent'>
                {step.icon}
              </StepperIndicator>

              <div className='flex flex-col items-start gap-y-1'>
                <div className='text-muted-foreground text-xs font-semibold uppercase'>
                  Step {index + 1}
                </div>
                <StepperTitle className='group-data-[state=inactive]/step:text-muted-foreground text-base font-semibold'>
                  {step.title}
                </StepperTitle>
              </div>
            </StepperTrigger>

            {steps.length > index + 1 && (
              <StepperSeparator className='group-data-[state=completed]/step:bg-primary absolute inset-x-0 start-9 top-4 m-0 group-data-[orientation=horizontal]/stepper-nav:w-[calc(100%-2rem)] group-data-[orientation=horizontal]/stepper-nav:flex-none' />
            )}
          </StepperItem>
        ))}
      </StepperNav>

      <form
        id='form-auth-register'
        onSubmit={form.handleSubmit(onSubmit)}
        className='space-y-8'
      >
        <StepperPanel>
          {/* Personal */}
          <StepperContent key={0} value={1}>
            <FieldGroup className='gap-5'>
              <Controller
                control={form.control}
                name='firstName'
                render={({ field, fieldState }) => (
                  <Field data-invalid={fieldState.invalid}>
                    <FieldLabel htmlFor={field.name}>
                      {t('register.form.fields.firstName.label')}
                    </FieldLabel>
                    <Input
                      {...field}
                      id={field.name}
                      aria-invalid={fieldState.invalid}
                      placeholder={t(
                        'register.form.fields.firstName.placeholder'
                      )}
                    />
                    {fieldState.invalid && (
                      <FieldError errors={[fieldState.error]} />
                    )}
                  </Field>
                )}
              />

              <Controller
                control={form.control}
                name='lastName'
                render={({ field, fieldState }) => (
                  <Field data-invalid={fieldState.invalid}>
                    <FieldLabel htmlFor={field.name}>
                      {t('register.form.fields.lastName.label')}
                    </FieldLabel>
                    <Input
                      {...field}
                      id={field.name}
                      aria-invalid={fieldState.invalid}
                      placeholder={t(
                        'register.form.fields.lastName.placeholder'
                      )}
                    />
                    {fieldState.invalid && (
                      <FieldError errors={[fieldState.error]} />
                    )}
                  </Field>
                )}
              />
            </FieldGroup>
          </StepperContent>

          {/* Account */}
          <StepperContent key={1} value={2}>
            <FieldGroup className='gap-5'>
              <Controller
                control={form.control}
                name='username'
                render={({ field, fieldState }) => (
                  <Field data-invalid={fieldState.invalid}>
                    <FieldLabel htmlFor={field.name}>
                      {t('register.form.fields.username.label')}
                    </FieldLabel>
                    <Input
                      {...field}
                      id={field.name}
                      aria-invalid={fieldState.invalid}
                      placeholder={t(
                        'register.form.fields.username.placeholder'
                      )}
                    />
                    {fieldState.invalid && (
                      <FieldError errors={[fieldState.error]} />
                    )}
                  </Field>
                )}
              />

              <Controller
                control={form.control}
                name='email'
                render={({ field, fieldState }) => (
                  <Field data-invalid={fieldState.invalid}>
                    <FieldLabel htmlFor={field.name}>
                      {t('register.form.fields.email.label')}
                    </FieldLabel>
                    <Input
                      {...field}
                      id={field.name}
                      aria-invalid={fieldState.invalid}
                      placeholder={t('register.form.fields.email.placeholder')}
                    />
                    {fieldState.invalid && (
                      <FieldError errors={[fieldState.error]} />
                    )}
                  </Field>
                )}
              />
            </FieldGroup>
          </StepperContent>

          {/* Security */}
          <StepperContent key={2} value={3}>
            <FieldGroup className='gap-5'>
              <Controller
                control={form.control}
                name='password'
                render={({ field, fieldState }) => (
                  <Field data-invalid={fieldState.invalid}>
                    <FieldLabel htmlFor={field.name}>
                      {t('register.form.fields.password.label')}
                    </FieldLabel>
                    <Input
                      {...field}
                      id={field.name}
                      type='password'
                      aria-invalid={fieldState.invalid}
                      placeholder={t(
                        'register.form.fields.password.placeholder'
                      )}
                    />
                    {fieldState.invalid && (
                      <FieldError errors={[fieldState.error]} />
                    )}
                  </Field>
                )}
              />

              <Controller
                control={form.control}
                name='confirmPassword'
                render={({ field, fieldState }) => (
                  <Field data-invalid={fieldState.invalid}>
                    <FieldLabel htmlFor={field.name}>
                      {t('register.form.fields.confirmPassword.label')}
                    </FieldLabel>
                    <Input
                      {...field}
                      id={field.name}
                      type='password'
                      aria-invalid={fieldState.invalid}
                      placeholder={t(
                        'register.form.fields.confirmPassword.placeholder'
                      )}
                    />
                    {fieldState.invalid && (
                      <FieldError errors={[fieldState.error]} />
                    )}
                  </Field>
                )}
              />
            </FieldGroup>
          </StepperContent>
        </StepperPanel>
      </form>

      <div className='flex items-center justify-between'>
        <Button
          type='button'
          variant='outline'
          disabled={currentStep === 1}
          onClick={() => setCurrentStep(step => step - 1)}
        >
          <ArrowLeft className='size-4' />
          <span>{t('register.form.previous')}</span>
        </Button>

        <Button onClick={handleNext}>
          {currentStep === steps.length ? (
            <span>{t('register.form.submit')}</span>
          ) : (
            <>
              <span>{t('register.form.next')}</span>
              <ArrowRight className='size-4' />
            </>
          )}
        </Button>
      </div>
    </Stepper>
  )
}
