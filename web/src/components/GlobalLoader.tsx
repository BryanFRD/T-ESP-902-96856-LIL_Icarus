import { faker } from '@faker-js/faker'
import { useEffect, useState } from 'react'

import Logo from '@/components/icon/Logo.tsx'

const GlobalLoader = () => {
  const [phrase, setPhrase] = useState('')

  useEffect(() => {
    faker.seed(Date.now())
    setPhrase(faker.hacker.phrase())
  }, [])

  return (
    <div className='fixed inset-0 z-50 flex flex-col items-center justify-center bg-white dark:bg-gray-900'>
      <div className='relative'>
        <div className='h-16 w-16 animate-spin rounded-full border-4 border-gray-200 border-t-transparent dark:border-gray-800 dark:border-t-transparent'></div>

        <div className='absolute inset-0 h-16 w-16 animate-spin rounded-full'></div>

        <div className='absolute inset-0 flex items-center justify-center'>
          <Logo fill={'#FF6900'} className='h-8 w-8' />
        </div>
      </div>

      <p className='mt-4 text-gray-700 dark:text-gray-300'>{phrase}</p>
    </div>
  )
}

export default GlobalLoader
