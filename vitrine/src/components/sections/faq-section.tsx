'use client'

import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger
} from '@/components/ui/accordion'
import { Badge } from '@/components/ui/badge'
import { useScopedI18n } from '@/i18n/client'

export function FAQSection() {
  const t = useScopedI18n('pages.home.faq')

  const questions = [
    {
      question: t('questions.0.question'),
      answer: t('questions.0.answer')
    },
    {
      question: t('questions.1.question'),
      answer: t('questions.1.answer')
    },
    {
      question: t('questions.2.question'),
      answer: t('questions.2.answer')
    },
    {
      question: t('questions.3.question'),
      answer: t('questions.3.answer')
    },
    {
      question: t('questions.4.question'),
      answer: t('questions.4.answer')
    },
    {
      question: t('questions.5.question'),
      answer: t('questions.5.answer')
    }
  ]

  return (
    <section className='bg-muted/50 py-20' id='faq'>
      <div className='container mx-auto px-4'>
        <div className='mx-auto max-w-3xl text-center'>
          <Badge variant='outline' className='mb-4'>
            {t('badge')}
          </Badge>
          <h2 className='mb-4 text-3xl font-bold tracking-tight md:text-4xl'>
            {t('title')}
          </h2>
          <p className='text-muted-foreground mb-12 text-lg'>{t('subtitle')}</p>
        </div>

        <div className='mx-auto max-w-3xl'>
          <Accordion type='single' collapsible className='w-full'>
            {questions.map((item, idx) => (
              <AccordionItem key={idx} value={`item-${idx}`}>
                <AccordionTrigger className='text-left'>
                  {item.question}
                </AccordionTrigger>
                <AccordionContent className='text-muted-foreground'>
                  {item.answer}
                </AccordionContent>
              </AccordionItem>
            ))}
          </Accordion>
        </div>
      </div>
    </section>
  )
}
