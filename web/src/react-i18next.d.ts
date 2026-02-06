// react-i18next.d.ts
import 'react-i18next'

// Import all namespace types
import app from './i18n/locales/en/app.json'
import audit from './i18n/locales/en/audit.json'
import auth from './i18n/locales/en/auth.json'
import common from './i18n/locales/en/common.json'
import messages from './i18n/locales/en/messages.json'
import team from './i18n/locales/en/team.json'
import ui from './i18n/locales/en/ui.json'

declare module 'react-i18next' {
  interface CustomTypeOptions {
    defaultNS: 'common'
    resources: {
      common: typeof common
      auth: typeof auth
      team: typeof team
      audit: typeof audit
      ui: typeof ui
      messages: typeof messages
      app: typeof app
    }
  }
}
