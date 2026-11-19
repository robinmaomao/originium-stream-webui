import 'vuetify/styles'
import '@mdi/font/css/materialdesignicons.css'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import { aliases, mdi } from 'vuetify/iconsets/mdi'

const baseTheme = {
  dark: true,
  colors: {
    primary: '#7C4DFF',
    secondary: '#00E5FF',
    surface: '#121218',
    background: '#0B0B0F',
    error: '#FF5370',
    success: '#4ADE80',
    info: '#38BDF8',
    warning: '#FBBF24',
    'on-surface': '#E8E8F0',
    'on-background': '#E8E8F0',
  },
}

export const vuetify = createVuetify({
  components,
  directives,
  icons: {
    defaultSet: 'mdi',
    aliases,
    sets: { mdi },
  },
  theme: {
    defaultTheme: 'originium',
    themes: {
      originium: baseTheme,
    },
  },
  defaults: {
    VBtn: { rounded: 'lg', variant: 'flat' },
    VCard: { rounded: 'xl' },
    VTextField: { variant: 'outlined', density: 'comfortable', rounded: 'lg' },
    VSelect: { variant: 'outlined', density: 'comfortable', rounded: 'lg' },
    VTextarea: { variant: 'outlined', density: 'comfortable', rounded: 'lg' },
    VSwitch: { color: 'primary', inset: true },
    VChip: { rounded: 'lg' },
    VDialog: { rounded: 'xl' },
    VTabs: { color: 'primary' },
  },
})
