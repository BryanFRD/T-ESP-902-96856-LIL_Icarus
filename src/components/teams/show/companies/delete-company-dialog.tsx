import { Building2Icon } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogMedia,
  AlertDialogTitle
} from '@/components/ui/alert-dialog'
import type { Company } from '@/domain/types/team.ts'
import { useDeleteCompany } from '@/hooks/team/use-delete-company.ts'

interface Props {
  open: boolean
  onOpenChange: (open: boolean) => void
  teamId: string
  company: Company
}

export function DeleteCompanyDialog({
  open,
  onOpenChange,
  teamId,
  company
}: Props) {
  const { t } = useTranslation('team')
  const { mutate, isPending } = useDeleteCompany()

  function handleConfirm() {
    mutate(
      { teamId, companyId: company.id },
      {
        onSuccess: () => {
          toast.success(t('show.companies.deleteDialog.toast.success'))
          onOpenChange(false)
        },
        onError: () => {
          toast.error(t('show.companies.deleteDialog.toast.error'))
        }
      }
    )
  }

  return (
    <AlertDialog open={open} onOpenChange={onOpenChange}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogMedia className='size-12'>
            <Building2Icon className='size-6' />
          </AlertDialogMedia>

          <AlertDialogTitle>
            {t('show.companies.deleteDialog.title')}
          </AlertDialogTitle>

          <AlertDialogDescription>
            {t('show.companies.deleteDialog.description', {
              name: company.name
            })}
          </AlertDialogDescription>
        </AlertDialogHeader>

        <AlertDialogFooter>
          <AlertDialogCancel>
            {t('show.companies.deleteDialog.cancel')}
          </AlertDialogCancel>

          <AlertDialogAction
            variant='destructive'
            disabled={isPending}
            onClick={handleConfirm}
          >
            {t('show.companies.deleteDialog.confirm')}
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  )
}
