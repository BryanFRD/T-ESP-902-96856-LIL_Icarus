import type { FeatureItemProps } from '@/types/global.ts'

export function FeatureItem({
  icon: Icon,
  title,
  description
}: FeatureItemProps) {
  return (
    <div className='flex items-center gap-x-4'>
      <div className='bg-muted text-muted-foreground flex size-10 items-center justify-center rounded-lg'>
        <Icon className='size-5' />
      </div>

      <div>
        <h3 className='font-semibold'>{title}</h3>
        <p className='text-muted-foreground text-sm'>{description}</p>
      </div>
    </div>
  )
}
