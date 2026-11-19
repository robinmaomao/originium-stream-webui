import * as api from '@/api'
import type { ThemeConfig } from '@/api/types'
import { isLightTheme } from './theme'

let appliedAcrylic: boolean | null = null
let appliedDark: boolean | null = null

export async function applyWindowChrome(acrylic: boolean, theme: ThemeConfig | null) {
  const light = theme ? isLightTheme(theme) : false
  const root = document.documentElement
  root.dataset.mode = light ? 'light' : 'dark'
  root.dataset.acrylic = acrylic ? 'on' : 'off'

  if (appliedAcrylic === acrylic && appliedDark === !light) return

  try {
    await api.setWindowEffects(acrylic, !light)
    appliedAcrylic = acrylic
    appliedDark = !light
  } catch {
    // CSS glass remains as a fallback on unsupported Windows versions.
  }
}
