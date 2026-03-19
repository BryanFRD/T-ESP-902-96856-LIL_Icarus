import { CameraIcon, CropIcon, Undo2Icon } from 'lucide-react'
import React, { type SyntheticEvent, useCallback, useRef } from 'react'
import { type FileWithPath, useDropzone } from 'react-dropzone'
import { useTranslation } from 'react-i18next'
import ReactCrop, {
  type Crop,
  type PixelCrop,
  centerCrop,
  makeAspectCrop
} from 'react-image-crop'
import 'react-image-crop/dist/ReactCrop.css'

import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle
} from '@/components/ui/dialog'
import { cn } from '@/lib/utils'

export type FileWithPreview = FileWithPath & {
  preview: string
}

interface ImageCropperProps {
  value?: string
  fallback?: string
  onChange?: (file: File) => void
  className?: string
}

export function ImageCropper({
  value,
  fallback = 'AV',
  onChange,
  className
}: ImageCropperProps) {
  const { t } = useTranslation('common')

  const imgRef = useRef<HTMLImageElement | null>(null)

  const [selectedFile, setSelectedFile] =
    React.useState<FileWithPreview | null>(null)
  const [croppedPreview, setCroppedPreview] = React.useState<string>('')
  const [isDialogOpen, setIsDialogOpen] = React.useState(false)
  const [crop, setCrop] = React.useState<Crop>()
  const [completedPixelCrop, setCompletedPixelCrop] =
    React.useState<PixelCrop | null>(null)

  function onImageLoad(e: SyntheticEvent<HTMLImageElement>) {
    const { width, height } = e.currentTarget
    setCrop(
      centerCrop(
        makeAspectCrop({ unit: '%', width: 90 }, 1, width, height),
        width,
        height
      )
    )
  }

  function buildCanvas(
    image: HTMLImageElement,
    pixelCrop: PixelCrop
  ): HTMLCanvasElement {
    const canvas = document.createElement('canvas')
    const scaleX = image.naturalWidth / image.width
    const scaleY = image.naturalHeight / image.height

    canvas.width = pixelCrop.width * scaleX
    canvas.height = pixelCrop.height * scaleY

    const ctx = canvas.getContext('2d')
    if (ctx) {
      ctx.imageSmoothingEnabled = false
      ctx.drawImage(
        image,
        pixelCrop.x * scaleX,
        pixelCrop.y * scaleY,
        pixelCrop.width * scaleX,
        pixelCrop.height * scaleY,
        0,
        0,
        pixelCrop.width * scaleX,
        pixelCrop.height * scaleY
      )
    }

    return canvas
  }

  function onCropComplete(pixelCrop: PixelCrop) {
    setCompletedPixelCrop(pixelCrop)
    if (imgRef.current && pixelCrop.width && pixelCrop.height) {
      setCroppedPreview(
        buildCanvas(imgRef.current, pixelCrop).toDataURL('image/png', 1.0)
      )
    }
  }

  async function handleCrop() {
    if (!imgRef.current || !completedPixelCrop || !selectedFile) return

    const canvas = buildCanvas(imgRef.current, completedPixelCrop)
    const file = await new Promise<File>((resolve, reject) => {
      canvas.toBlob(
        blob => {
          if (!blob) return reject(new Error('Failed to create blob'))
          resolve(new File([blob], selectedFile.name, { type: 'image/png' }))
        },
        'image/png',
        1.0
      )
    })

    onChange?.(file)
    setIsDialogOpen(false)
  }

  function handleCancel() {
    setSelectedFile(null)
    setCroppedPreview('')
    setCompletedPixelCrop(null)
    setIsDialogOpen(false)
  }

  const onDrop = useCallback((acceptedFiles: FileWithPath[]) => {
    const file = acceptedFiles[0]
    if (!file) return

    setSelectedFile(
      Object.assign(file, {
        preview: URL.createObjectURL(file)
      }) as FileWithPreview
    )
    setIsDialogOpen(true)
  }, [])

  const { getRootProps, getInputProps, isDragActive, open } = useDropzone({
    onDrop,
    accept: { 'image/*': [] },
    maxFiles: 1,
    noClick: true
  })

  function handleClick() {
    if (selectedFile) {
      setIsDialogOpen(true)
    } else {
      open()
    }
  }

  const displaySrc = croppedPreview || selectedFile?.preview || value

  return (
    <>
      <div
        {...getRootProps()}
        onClick={handleClick}
        className={cn('group relative cursor-pointer outline-none', className)}
      >
        <input {...getInputProps()} />
        <Avatar
          className={cn(
            'group-focus-visible:ring-ring/50 size-20 ring-2 ring-offset-2 transition-all',
            isDragActive ? 'ring-primary scale-105' : 'ring-secondary'
          )}
        >
          <AvatarImage src={displaySrc} alt='avatar' />
          <AvatarFallback className='text-muted-foreground text-xs'>
            {fallback}
          </AvatarFallback>
        </Avatar>

        <div
          className={cn(
            'absolute inset-0 flex flex-col items-center justify-center gap-0.5 rounded-full transition-all',
            isDragActive
              ? 'bg-primary/50 scale-105'
              : 'group-hover:bg-muted-foreground/70'
          )}
        >
          <CameraIcon
            className={cn(
              'size-5 text-white transition-opacity',
              isDragActive ? 'opacity-100' : 'opacity-0 group-hover:opacity-100'
            )}
          />
          <span
            className={cn(
              'text-xs font-medium text-white transition-opacity',
              isDragActive ? 'opacity-100' : 'opacity-0 group-hover:opacity-100'
            )}
          >
            {isDragActive ? 'Drop' : 'Edit'}
          </span>
        </div>
      </div>

      <Dialog
        open={isDialogOpen}
        onOpenChange={open => !open && handleCancel()}
      >
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t('ui.imageCropper.title')}</DialogTitle>
          </DialogHeader>

          <div className='size-full'>
            <ReactCrop
              crop={crop}
              onChange={(_, percentCrop) => setCrop(percentCrop)}
              onComplete={c => onCropComplete(c)}
              aspect={1}
              circularCrop
              className='w-full'
            >
              <img
                ref={imgRef}
                src={selectedFile?.preview}
                alt={t('ui.imageCropper.alt')}
                onLoad={onImageLoad}
                className='w-full'
              />
            </ReactCrop>
          </div>

          <DialogFooter>
            <DialogClose asChild>
              <Button
                size='sm'
                type='button'
                variant='outline'
                onClick={handleCancel}
              >
                <Undo2Icon className='size-4' />
                {t('ui.imageCropper.actions.cancel')}
              </Button>
            </DialogClose>

            <Button
              size='sm'
              type='button'
              onClick={handleCrop}
              disabled={!completedPixelCrop}
            >
              <CropIcon className='size-4' />
              {t('ui.imageCropper.actions.apply')}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  )
}
