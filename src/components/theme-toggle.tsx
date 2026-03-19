import { Laptop, Moon, Sun } from 'lucide-react'

import { useTheme } from './theme-provider'
import { Button } from './ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuTrigger
} from './ui/dropdown-menu'

export function ThemeToggle() {
  const { setTheme } = useTheme()

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button size='icon' variant='secondary'>
          <Sun className='size-4 dark:hidden' />
          <Moon className='size-4 not-dark:hidden' />
        </Button>
      </DropdownMenuTrigger>

      <DropdownMenuContent align='end' className='min-w-40'>
        <DropdownMenuGroup>
          <DropdownMenuItem onClick={() => setTheme('light')}>
            <Sun className='size-4' />
            <span>Light</span>
          </DropdownMenuItem>
          <DropdownMenuItem onClick={() => setTheme('dark')}>
            <Moon className='size-4' />
            <span>Dark</span>
          </DropdownMenuItem>
          <DropdownMenuItem onClick={() => setTheme('system')}>
            <Laptop className='size-4' />
            <span>System</span>
          </DropdownMenuItem>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
