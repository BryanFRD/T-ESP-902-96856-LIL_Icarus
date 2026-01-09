import { buttonSizes, buttonVariants } from '@/types/components/button.type'

import Button from '../components/ui/Button'

export default {
  title: 'UI/Button',
  component: Button,
  tags: ['autodocs']
}

export const AllVariantsAndSizes = () => (
  <div className='flex flex-col gap-8 p-4'>
    {buttonVariants.map(variant => (
      <div key={variant}>
        <h3 className='mb-2 text-lg font-semibold capitalize'>{variant}</h3>
        <div className='flex flex-col flex-wrap items-end justify-start gap-4 md:flex-row'>
          {buttonSizes.map(size => (
            <Button key={size} variant={variant} size={size}>
              {variant} {size}
            </Button>
          ))}
        </div>
      </div>
    ))}
  </div>
)
