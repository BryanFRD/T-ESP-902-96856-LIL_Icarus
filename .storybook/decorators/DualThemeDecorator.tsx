import type { Decorator } from '@storybook/react-vite'
import { useTheme } from 'storybook/internal/theming'

const DualThemeDecorator: Decorator = (Story, context) => {
  useTheme()

  if (context.viewMode === 'docs') {
    return <Story />
  }

  return (
    <div className='flex gap-8'>
      <div className='flex-1'>
        <h2 className='mb-4 text-lg font-semibold'>Light Theme</h2>
        <div className='rounded bg-white p-4 shadow'>
          <Story />
        </div>
      </div>
      <div className='flex-1'>
        <h2 className='mb-4 text-lg font-semibold'>Dark Theme</h2>
        <div className='dark rounded bg-gray-800 p-4 text-white shadow'>
          <Story />
        </div>
      </div>
    </div>
  )
}

export default DualThemeDecorator
