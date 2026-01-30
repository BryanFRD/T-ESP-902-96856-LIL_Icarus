import type { SocialButtonProps } from '@/types/global.ts'

const SocialButton = ({ icon, label, onClick }: SocialButtonProps) => {
  return (
    <button
      onClick={onClick}
      className='flex items-center justify-center gap-2 rounded-xl border border-gray-200 px-4 py-3 transition-colors hover:bg-gray-50'
    >
      {icon}
      {label}
    </button>
  )
}

export default SocialButton
