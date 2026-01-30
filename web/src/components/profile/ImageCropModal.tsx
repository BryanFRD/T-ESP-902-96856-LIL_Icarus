import { RotateCw, ZoomIn, ZoomOut } from 'lucide-react'
import { useRef, useState } from 'react'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'
import ReactCrop from 'react-image-crop'
import 'react-image-crop/dist/ReactCrop.css'

import Button from '@/components/ui/button/Button.tsx'
import Modal from '@/components/ui/modal/Modal.tsx'
import { useImageCrop } from '@/hooks/common/useImageCrop.ts'
import { getErrorMessage } from '@/utils/apiUtils'

interface ImageCropModalProps {
  isOpen: boolean
  onClose: () => void
  imageSrc: string
  fileName: string
  onCropComplete: (croppedFile: File) => void
}

const ImageCropModal = ({
  isOpen,
  onClose,
  imageSrc,
  fileName,
  onCropComplete
}: ImageCropModalProps) => {
  const { t } = useTranslation()
  const imgRef = useRef<HTMLImageElement>(null)
  const [isProcessing, setIsProcessing] = useState(false)

  const {
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
  } = useImageCrop(1)

  const handleCropComplete = async () => {
    if (!imgRef.current || !completedCrop) return

    setIsProcessing(true)
    try {
      const croppedFile = await getCroppedImage(
        imgRef.current,
        completedCrop,
        fileName
      )
      onCropComplete(croppedFile)
      onClose()
    } catch (error) {
      toast.error(t(getErrorMessage(error)))
    } finally {
      setIsProcessing(false)
    }
  }

  const modalFooter = (
    <div className='flex justify-end space-x-3'>
      <Button variant='outline' onClick={onClose}>
        {t('common.cancel')}
      </Button>
      <Button
        onClick={handleCropComplete}
        isLoading={isProcessing}
        disabled={!completedCrop}
      >
        {t('image_crop.apply')}
      </Button>
    </div>
  )

  return (
    <Modal
      isOpen={isOpen}
      onClose={onClose}
      title={t('image_crop.title')}
      size='xl'
      footer={modalFooter}
      className='max-w-3xl'
    >
      <div className='mb-6 flex flex-wrap gap-4'>
        <div className='flex items-center space-x-2'>
          <ZoomOut className='h-4 w-4 text-gray-500' />
          <input
            type='range'
            min='0.5'
            max='3'
            step='0.1'
            value={scale}
            onChange={e => setScale(Number(e.target.value))}
            className='h-2 w-24 cursor-pointer appearance-none rounded-lg bg-gray-200 dark:bg-gray-700'
          />
          <ZoomIn className='h-4 w-4 text-gray-500' />
          <span className='min-w-[3rem] text-sm text-gray-600 dark:text-gray-400'>
            {Math.round(scale * 100)}%
          </span>
        </div>

        <Button
          variant='outline'
          size='sm'
          onClick={() => setRotate(prev => prev + 90)}
        >
          <RotateCw className='mr-2 h-4 w-4' />
          {t('image_crop.rotate')}
        </Button>
      </div>

      <div className='mb-4 rounded-lg bg-blue-50 p-3 dark:bg-blue-900/20'>
        <p className='text-sm text-blue-700 dark:text-blue-300'>
          {t('image_crop.instructions')}
        </p>
      </div>

      <div className='flex justify-center rounded-lg bg-gray-50 p-4 dark:bg-gray-800'>
        <ReactCrop
          crop={crop}
          onChange={(_, percentCrop) => setCrop(percentCrop)}
          onComplete={c => setCompletedCrop(c)}
          aspect={1}
          circularCrop
          className='max-h-96 max-w-full'
        >
          <img
            ref={imgRef}
            src={imageSrc}
            alt={t('image_crop.preview_alt')}
            style={{
              transform: `scale(${scale}) rotate(${rotate}deg)`,
              maxWidth: '100%',
              maxHeight: '384px',
              objectFit: 'contain'
            }}
            onLoad={onImageLoad}
            className='block'
          />
        </ReactCrop>
      </div>

      {completedCrop && (
        <div className='mt-6'>
          <h4 className='mb-2 text-sm font-medium text-gray-700 dark:text-gray-300'>
            {t('image_crop.preview')}
          </h4>
          <div className='flex justify-center'>
            <div className='h-20 w-20 overflow-hidden rounded-full border-2 border-gray-200 dark:border-gray-600'>
              <canvas
                ref={canvas => {
                  if (canvas && imgRef.current && completedCrop) {
                    const ctx = canvas.getContext('2d')!
                    canvas.width = 80
                    canvas.height = 80

                    const scaleX =
                      imgRef.current.naturalWidth / imgRef.current.width
                    const scaleY =
                      imgRef.current.naturalHeight / imgRef.current.height

                    ctx.drawImage(
                      imgRef.current,
                      completedCrop.x * scaleX,
                      completedCrop.y * scaleY,
                      completedCrop.width * scaleX,
                      completedCrop.height * scaleY,
                      0,
                      0,
                      80,
                      80
                    )
                  }
                }}
                className='h-full w-full object-cover'
              />
            </div>
          </div>
        </div>
      )}
    </Modal>
  )
}

export default ImageCropModal
