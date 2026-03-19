import { useState } from 'react'

const useToggle = (initialValue = false) => {
  const [value, setValue] = useState(initialValue)

  const toggle = () => setValue(prev => !prev)
  const open = () => setValue(true)
  const close = () => setValue(false)
  const set = (newValue: boolean) => setValue(newValue)

  return { value, toggle, open, close, set }
}

export default useToggle
