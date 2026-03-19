import { useCallback, useState } from 'react'
import { type Crop, centerCrop, makeAspectCrop } from 'react-image-crop'

export const useImageCrop = (aspect = 1) => {
  const [crop, setCrop] = useState<Crop>()
  const [completedCrop, setCompletedCrop] = useState<Crop>()
  const [scale, setScale] = useState(1)
  const [rotate, setRotate] = useState(0)

  const onImageLoad = useCallback(
    (e: React.SyntheticEvent<HTMLImageElement>) => {
      const { width, height } = e.currentTarget
      setCrop(
        centerCrop(
          makeAspectCrop(
            {
              unit: '%',
              width: 90
            },
            aspect,
            width,
            height
          ),
          width,
          height
        )
      )
    },
    [aspect]
  )

  const getCroppedImage = useCallback(
    async (
      image: HTMLImageElement,
      crop: Crop,
      fileName: string
    ): Promise<File> => {
      const scaleX = image.naturalWidth / image.width
      const scaleY = image.naturalHeight / image.height
      const cropWidth = crop.width * scaleX
      const cropHeight = crop.height * scaleY

      const canvas = document.createElement('canvas')
      canvas.width = Math.floor(cropWidth)
      canvas.height = Math.floor(cropHeight)
      const ctx = canvas.getContext('2d')
      if (!ctx) throw new Error('Could not get 2D context')

      ctx.save()
      ctx.translate(-crop.x * scaleX, -crop.y * scaleY)
      ctx.translate(image.naturalWidth / 2, image.naturalHeight / 2)
      ctx.rotate((rotate * Math.PI) / 180)
      ctx.scale(scale, scale)
      ctx.translate(-image.naturalWidth / 2, -image.naturalHeight / 2)
      ctx.drawImage(image, 0, 0, image.naturalWidth, image.naturalHeight)
      ctx.restore()

      return new Promise(resolve => {
        canvas.toBlob(
          blob => {
            if (!blob) throw new Error('Failed to create blob')
            resolve(new File([blob], fileName, { type: 'image/jpeg' }))
          },
          'image/jpeg',
          0.9
        )
      })
    },
    [scale, rotate]
  )

  return {
    crop,
    setCrop,
    completedCrop,
    setCompletedCrop,
    scale,
    setScale,
    rotate,
    setRotate,
    onImageLoad,
    getCroppedImage
  }
}
