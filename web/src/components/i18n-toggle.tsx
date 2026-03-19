import { CircleFlagLanguage } from 'react-circle-flags'
import { useTranslation } from 'react-i18next'

import { Button } from './ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuTrigger
} from './ui/dropdown-menu'

export function I18nToggle() {
  const { i18n } = useTranslation()

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button size='icon' variant='secondary'>
          <CircleFlagLanguage languageCode={i18n.language} className='size-5' />
        </Button>
      </DropdownMenuTrigger>

      <DropdownMenuContent align='end' className='min-w-40'>
        <DropdownMenuGroup>
          <DropdownMenuItem onClick={() => i18n.changeLanguage('en')}>
            <CircleFlagLanguage languageCode='en' className='size-4' />
            <span>English</span>
          </DropdownMenuItem>
          <DropdownMenuItem onClick={() => i18n.changeLanguage('fr')}>
            <CircleFlagLanguage languageCode='fr' className='size-4' />
            <span>French</span>
          </DropdownMenuItem>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
