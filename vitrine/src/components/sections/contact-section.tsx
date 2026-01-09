'use client'

import { Mail, Send } from 'lucide-react'
import { useState } from 'react'

import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { useScopedI18n } from '@/i18n/client'

export function ContactSection() {
  const t = useScopedI18n('pages.home.contact')
  const [formData, setFormData] = useState({
    name: '',
    email: '',
    message: ''
  })

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    // Handle form submission here
    // eslint-disable-next-line no-console
    console.log('Form submitted:', formData)
  }

  return (
    <section className='py-20' id='contact'>
      <div className='container mx-auto px-4'>
        <div className='mx-auto max-w-3xl text-center'>
          <Badge className='mb-4'>
            <Mail className='mr-2 h-3 w-3' />
            {t('badge')}
          </Badge>
          <h2 className='mb-4 text-3xl font-bold tracking-tight md:text-4xl'>
            {t('title')}
          </h2>
          <p className='text-muted-foreground mb-12 text-lg'>{t('subtitle')}</p>
        </div>

        <div className='mx-auto max-w-2xl'>
          <form onSubmit={handleSubmit} className='space-y-6'>
            <div>
              <label htmlFor='name' className='mb-2 block text-sm font-medium'>
                {t('form.name')}
              </label>
              <Input
                id='name'
                placeholder={t('form.namePlaceholder')}
                value={formData.name}
                onChange={e =>
                  setFormData({ ...formData, name: e.target.value })
                }
                required
              />
            </div>

            <div>
              <label htmlFor='email' className='mb-2 block text-sm font-medium'>
                {t('form.email')}
              </label>
              <Input
                id='email'
                type='email'
                placeholder={t('form.emailPlaceholder')}
                value={formData.email}
                onChange={e =>
                  setFormData({ ...formData, email: e.target.value })
                }
                required
              />
            </div>

            <div>
              <label
                htmlFor='message'
                className='mb-2 block text-sm font-medium'
              >
                {t('form.message')}
              </label>
              <Textarea
                id='message'
                placeholder={t('form.messagePlaceholder')}
                rows={6}
                value={formData.message}
                onChange={e =>
                  setFormData({ ...formData, message: e.target.value })
                }
                required
              />
            </div>

            <Button type='submit' size='lg' className='w-full'>
              {t('form.submit')}
              <Send className='ml-2 h-4 w-4' />
            </Button>
          </form>
        </div>
      </div>
    </section>
  )
}
