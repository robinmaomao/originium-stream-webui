import { defineStore } from 'pinia'
import * as api from '@/api'
import type { AssetsInfo, ThemeConfig, ThemePreset } from '@/api/types'
import { useAppStore } from './app'

export const useThemeStore = defineStore('theme', {
  state: () => ({
    theme: null as ThemeConfig | null,
    presets: [] as ThemePreset[],
    assets: { fonts: [], backgrounds: [] } as AssetsInfo,
    backgroundUrl: '',
  }),
  getters: {
    fontOptions: (state) => state.assets.fonts,
    backgroundOptions: (state) => state.assets.backgrounds,
  },
  actions: {
    async load() {
      const app = useAppStore()
      this.theme = await api.getTheme()
      this.presets = await api.listThemePresets()
      this.assets = await api.listAssets()
      if (app.config) app.config.theme = this.theme
      await this.apply(this.theme)
    },
    async apply(theme: ThemeConfig | null) {
      if (!theme) return
      this.theme = theme
      const root = document.documentElement
      root.style.setProperty('--originium-primary', theme.colors.primary)
      root.style.setProperty('--originium-secondary', theme.colors.secondary)
      root.style.setProperty('--originium-surface', theme.colors.surface)
      root.style.setProperty('--originium-background', theme.colors.background)
      root.style.setProperty('--originium-error', theme.colors.error)
      root.style.setProperty('--originium-success', theme.colors.success)
      root.style.setProperty('--originium-info', theme.colors.info)
      root.style.setProperty('--originium-warning', theme.colors.warning)
      root.style.setProperty('--originium-font', theme.typography.fontFamily)
      root.style.setProperty('--originium-scale', String(theme.typography.scale))
      root.style.setProperty('--originium-radius', `${theme.effects.rounded}px`)
      root.style.setProperty('--originium-bg-opacity', String(theme.background.opacity))
      root.style.setProperty('--originium-bg-blur', `${theme.background.blur}px`)
      root.dataset.motion = theme.effects.motion ? 'on' : 'off'
      root.dataset.preset = theme.preset

      if (theme.background.image) {
        try {
          this.backgroundUrl = await api.getAssetDataUrl(theme.background.image)
          root.style.setProperty('--originium-bg-image', `url("${this.backgroundUrl}")`)
        } catch {
          this.backgroundUrl = ''
          root.style.removeProperty('--originium-bg-image')
        }
      } else {
        this.backgroundUrl = ''
        root.style.removeProperty('--originium-bg-image')
      }

      await this.registerFonts()
      document.body.style.background = theme.colors.background
    },
    async registerFonts() {
      let style = document.getElementById('originium-custom-fonts') as HTMLStyleElement | null
      if (!style) {
        style = document.createElement('style')
        style.id = 'originium-custom-fonts'
        document.head.appendChild(style)
      }
      if (this.assets.fonts.length === 0) {
        style.textContent = ''
        return
      }
      const rules = await Promise.all(
        this.assets.fonts.map(async (file) => {
          try {
            const url = await api.getAssetDataUrl(`fonts/${file}`)
            const family = file.replace(/\.[^.]+$/, '')
            const format = file.endsWith('.woff2')
              ? 'woff2'
              : file.endsWith('.woff')
                ? 'woff'
                : file.endsWith('.otf')
                  ? 'opentype'
                  : 'truetype'
            return `@font-face { font-family: '${family}'; src: url('${url}') format('${format}'); font-display: swap; }`
          } catch {
            return ''
          }
        }),
      )
      style.textContent = rules.filter(Boolean).join('\n')
    },
    async save(theme: ThemeConfig) {
      const saved = await api.saveTheme(theme)
      this.theme = saved
      const app = useAppStore()
      if (app.config) app.config.theme = saved
      await this.apply(saved)
      return saved
    },
    async applyPreset(id: string) {
      const preset = this.presets.find((item) => item.id === id)
      if (!preset) return
      const theme: ThemeConfig = {
        ...preset.theme,
        background: { ...preset.theme.background, image: this.theme?.background.image ?? null },
      }
      await this.save(theme)
    },
    async importTheme(json: string) {
      const theme = await api.importTheme(json)
      this.theme = theme
      const app = useAppStore()
      if (app.config) app.config.theme = theme
      await this.apply(theme)
      return theme
    },
    async exportTheme() {
      return api.exportTheme()
    },
    async importFont(path: string) {
      const relative = await api.importAsset('font', path)
      this.assets = await api.listAssets()
      return relative
    },
    async importBackground(path: string) {
      const relative = await api.importAsset('background', path)
      this.assets = await api.listAssets()
      return relative
    },
    async refreshAssets() {
      this.assets = await api.listAssets()
    },
  },
})
