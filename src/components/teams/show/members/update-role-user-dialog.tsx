import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'

import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle
} from '@/components/ui/dialog'
import {
  Item,
  ItemActions,
  ItemContent,
  ItemDescription,
  ItemGroup,
  ItemTitle
} from '@/components/ui/item'
import { RadioGroup, RadioGroupItem } from '@/components/ui/radio-group'
import { MemberRole } from '@/domain/enum/member-role.ts'
import type { TeamMember } from '@/domain/types/team.ts'
import { useUpdateMemberRole } from '@/hooks/team/use-update-member-role.ts'

const ASSIGNABLE_ROLES = [
  MemberRole.MODERATOR,
  MemberRole.MEMBER,
  MemberRole.CLIENT
] as const

type AssignableRole = (typeof ASSIGNABLE_ROLES)[number]

interface Props {
  open: boolean
  onOpenChange: (open: boolean) => void
  teamId: string
  member: TeamMember
}

export function UpdateRoleUserDialog({
  open,
  onOpenChange,
  teamId,
  member
}: Props) {
  const { t } = useTranslation('team')
  const { mutate, isPending } = useUpdateMemberRole()

  const initialRole = ASSIGNABLE_ROLES.includes(
    member.teamRole as AssignableRole
  )
    ? (member.teamRole as AssignableRole)
    : MemberRole.MEMBER

  const [selectedRole, setSelectedRole] = useState<AssignableRole>(initialRole)

  function handleSubmit() {
    mutate(
      { teamId, memberId: member.id, payload: { role: selectedRole } },
      {
        onSuccess: () => {
          toast.success(t('show.members.updateRole.toast.success'))
          onOpenChange(false)
        },
        onError: () => {
          toast.error(t('show.members.updateRole.toast.error'))
        }
      }
    )
  }

  const displayName =
    member.firstName && member.lastName
      ? `${member.firstName} ${member.lastName}`
      : member.username || member.email

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t('show.members.updateRole.title')}</DialogTitle>
          <DialogDescription>
            {t('show.members.updateRole.description', { name: displayName })}
          </DialogDescription>
        </DialogHeader>

        <RadioGroup
          value={selectedRole}
          onValueChange={value => setSelectedRole(value as AssignableRole)}
        >
          <ItemGroup>
            {ASSIGNABLE_ROLES.map(role => (
              <Item
                key={role}
                variant='outline'
                className='cursor-pointer'
                asChild
              >
                <label>
                  <ItemContent>
                    <ItemTitle>
                      {t(
                        `memberRoles.${role.toLowerCase() as Lowercase<AssignableRole>}`
                      )}
                    </ItemTitle>
                    <ItemDescription>
                      {t(
                        `memberRoles.descriptions.${role.toLowerCase() as Lowercase<AssignableRole>}`
                      )}
                    </ItemDescription>
                  </ItemContent>
                  <ItemActions>
                    <RadioGroupItem value={role} />
                  </ItemActions>
                </label>
              </Item>
            ))}
          </ItemGroup>
        </RadioGroup>

        <DialogFooter>
          <Button
            onClick={handleSubmit}
            disabled={isPending || selectedRole === initialRole}
          >
            {t('show.members.updateRole.submit')}
          </Button>

          <Button onClick={() => onOpenChange(false)} variant='secondary'>
            {t('show.members.updateRole.close')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
