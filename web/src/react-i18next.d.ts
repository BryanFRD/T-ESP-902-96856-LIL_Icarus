// react-i18next.d.ts
import 'react-i18next'

// 1. On récupère le types des traductions à partir du JSON importé
import en from './locales/en.json'

type Resources = typeof en

declare module 'react-i18next' {
  // 2. On surcharge l'interface DefaultResources pour que 't' soit typé avec les clés de Resources
  interface Resources {
    translation: Resources
  }
}
