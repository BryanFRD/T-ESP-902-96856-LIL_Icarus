type DarkModeIconProps = {
  className?: string
  isDark: boolean
}

const DarkModeIcon = ({ className = '', isDark }: DarkModeIconProps) => {
  return (
    <svg
      className={`${className} transition-all duration-500 ease-in-out ${
        isDark ? 'rotate-0' : 'rotate-180'
      }`}
      viewBox='0 0 24 24'
      fill='none'
      stroke='currentColor'
      strokeWidth='2'
      strokeLinecap='round'
      strokeLinejoin='round'
    >
      <circle
        cx='12'
        cy='12'
        r='5'
        className={`transition-all duration-300 ease-in-out ${
          isDark ? 'scale-0 opacity-0' : 'scale-100 opacity-100'
        } origin-center`}
        fill='#FFD700'
        stroke='#FFD700'
      />
      <g
        className={`transition-opacity duration-300 ease-in-out ${
          isDark ? 'opacity-0' : 'opacity-100'
        }`}
        stroke='#FFD700'
      >
        <line x1='12' y1='1' x2='12' y2='3' />
        <line x1='12' y1='21' x2='12' y2='23' />
        <line x1='4.22' y1='4.22' x2='5.64' y2='5.64' />
        <line x1='18.36' y1='18.36' x2='19.78' y2='19.78' />
        <line x1='1' y1='12' x2='3' y2='12' />
        <line x1='21' y1='12' x2='23' y2='12' />
        <line x1='4.22' y1='19.78' x2='5.64' y2='18.36' />
        <line x1='18.36' y1='5.64' x2='19.78' y2='4.22' />
      </g>
      <path
        d='M21 12.79A9 9 0 1111.21 3 7 7 0 0021 12.79z'
        className={`transition-all duration-300 ease-in-out ${
          isDark ? 'scale-100 opacity-100' : 'scale-0 opacity-0'
        } origin-center`}
        fill='#CEDFF6'
        stroke='#CEDFF6'
      />
    </svg>
  )
}

export default DarkModeIcon
