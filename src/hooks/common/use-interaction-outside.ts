import { useCallback, useEffect, useRef } from 'react'

import type {
  UseInteractionOutsideOptions,
  UseInteractionOutsideReturn
} from '@/types/global.ts'

function useInteractionOutside<T extends HTMLElement = HTMLElement>(
  callback: () => void,
  options: UseInteractionOutsideOptions = {}
): UseInteractionOutsideReturn<T> {
  const {
    enabled = true,
    eventType = 'click',
    ignoreEscapeKey = false
  } = options

  const ref = useRef<T>(null!)
  const callbackRef = useRef(callback)

  useEffect(() => {
    callbackRef.current = callback
  })

  const handleInteractOutside = useCallback((event: Event) => {
    if (ref.current && !ref.current.contains(event.target as Node)) {
      callbackRef.current()
    }
  }, [])

  const handleEscapeKey = useCallback((event: KeyboardEvent) => {
    if (event.key === 'Escape') {
      callbackRef.current()
    }
  }, [])

  useEffect(() => {
    if (!enabled) return

    const timeoutId = setTimeout(() => {
      document.addEventListener(eventType, handleInteractOutside)

      if (!ignoreEscapeKey) {
        document.addEventListener('keydown', handleEscapeKey)
      }
    }, 0)

    return () => {
      clearTimeout(timeoutId)
      document.removeEventListener(eventType, handleInteractOutside)
      if (!ignoreEscapeKey) {
        document.removeEventListener('keydown', handleEscapeKey)
      }
    }
  }, [
    enabled,
    eventType,
    ignoreEscapeKey,
    handleInteractOutside,
    handleEscapeKey
  ])

  return { ref }
}

export default useInteractionOutside
