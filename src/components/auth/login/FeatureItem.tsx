import type { FeatureItemProps } from '@/types/global.ts'

const FeatureItem = ({ icon: Icon, title, description }: FeatureItemProps) => {
  return (
    <div className='flex items-center gap-4'>
      {/* Icon container */}
      <div className='rounded-lg bg-black/10 p-2 backdrop-blur-sm transition-all duration-300 dark:bg-white/20'>
        <Icon className='h-5 w-5 text-gray-900 transition-all duration-300 dark:text-white' />
      </div>

      {/* Text content */}
      <div>
        <h3 className='font-semibold text-gray-900 transition-all duration-300 dark:text-white'>
          {title}
        </h3>
        <p className='text-sm text-blue-900 transition-all duration-300 dark:text-blue-100'>
          {description}
        </p>
      </div>
    </div>
  )
}

export default FeatureItem
