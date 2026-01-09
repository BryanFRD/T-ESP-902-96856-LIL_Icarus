import { useLanguageOptions } from '@/hooks/lang/useLanguageOptions'

const LanguageSelector = ({
  onChange
}: {
  onChange: (code: string) => void
}) => {
  const options = useLanguageOptions()

  return (
    <select onChange={e => onChange(e.target.value)}>
      {options.map(opt => (
        <option key={opt.code} value={opt.code}>
          {opt.label}
        </option>
      ))}
    </select>
  )
}

export default LanguageSelector
