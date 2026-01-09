import type { Meta, StoryObj } from '@storybook/react'
import { useState } from 'react'

import FileUpload from '@/components/ui/upload/FileUpload'

const meta: Meta<typeof FileUpload> = {
  title: 'UI/FileUpload',
  component: FileUpload
}
export default meta

type VariantType = 'avatar' | 'any' | 'chat' | 'document' | 'gallery'

const VARIANTS: { label: string; variant: VariantType }[] = [
  { label: 'Avatar', variant: 'avatar' },
  { label: 'Any', variant: 'any' },
  { label: 'Chat', variant: 'chat' },
  { label: 'Document', variant: 'document' },
  { label: 'Gallery', variant: 'gallery' }
]

const FileUploadSection = ({
  label,
  variant
}: {
  label: string
  variant: VariantType
}) => {
  const [files, setFiles] = useState<File[]>([])
  return (
    <section>
      <h3 className='mb-2 text-lg font-bold'>{label}</h3>
      <FileUpload variant={variant} onFileSelect={setFiles} />
      <div className='mt-2 text-xs text-gray-500'>
        Fichiers sélectionnés :{' '}
        {files.length > 0 ? files.map(f => f.name).join(', ') : 'Aucun'}
      </div>
    </section>
  )
}

const AllVariantsComponent = () => (
  <div className='mx-auto max-w-2xl space-y-10 py-8'>
    {VARIANTS.map(({ label, variant }) => (
      <FileUploadSection key={variant} label={label} variant={variant} />
    ))}
  </div>
)

export const AllVariants: StoryObj = {
  render: () => <AllVariantsComponent />
}
