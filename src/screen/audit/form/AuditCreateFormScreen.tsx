import { useTranslation } from 'react-i18next'

import AuditCreateForm from '@/components/audit/AuditCreateForm.tsx'
import BackButton from '@/components/ui/button/BackButton.tsx'

const AuditCreateFormScreen = () => {
  const { t } = useTranslation()

  return (
    <div className={'p-8'}>
      <div className={'mb-8 flex items-center gap-4'}>
        <BackButton />
        <h1 className='text-2xl font-bold'>
          {t('audit:createAuditFormTitle')}
        </h1>
      </div>
      <AuditCreateForm />
    </div>
  )
}

export default AuditCreateFormScreen
