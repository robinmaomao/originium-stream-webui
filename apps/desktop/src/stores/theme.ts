import { defineStore } from 'pinia'
import * as api from '@/api'
import type { AssetsInfo, ThemeConfig, ThemePreset } from '@/api/types'
import { useAppStore } from './app'
import { isLightTheme } from '@/utils/theme'
import { applyWindowChrome } from '@/utils/window'

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
      const light = isLightTheme(theme)
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
      root.style.setProperty('--originium-text', light ? '#1F2937' : '#E8E8F0')
      root.style.setProperty(
        '--originium-text-muted',
        light ? 'rgba(31, 41, 55, 0.68)' : 'rgba(232, 232, 240, 0.64)',
      )
      root.style.setProperty(
        '--originium-text-faint',
        light ? 'rgba(31, 41, 55, 0.46)' : 'rgba(232, 232, 240, 0.45)',
      )
      root.style.setProperty(
        '--originium-border',
        light ? 'rgba(15, 23, 42, 0.10)' : 'rgba(255, 255, 255, 0.07)',
      )
      root.style.setProperty(
        '--originium-hover',
        light ? 'rgba(15, 23, 42, 0.06)' : 'rgba(255, 255, 255, 0.07)',
      )
      root.style.setProperty(
        '--originium-titlebar-bg',
        light ? 'rgba(255, 255, 255, 0.62)' : 'rgba(10, 10, 16, 0.62)',
      )
      root.style.setProperty(
        '--originium-nav-bg',
        light ? 'rgba(255, 255, 255, 0.58)' : 'rgba(12, 12, 18, 0.58)',
      )
      root.style.setProperty(
        '--originium-glass-bg',
        light ? 'rgba(255, 255, 255, 0.72)' : 'rgba(22, 22, 30, 0.58)',
      )
      root.style.setProperty(
        '--originium-shell-top',
        light ? 'rgba(255, 255, 255, 0.64)' : 'rgba(10, 10, 16, 0.72)',
      )
      root.style.setProperty(
        '--originium-shell-bottom',
        light ? 'rgba(248, 247, 255, 0.78)' : 'rgba(10, 10, 16, 0.88)',
      )
      root.dataset.motion = theme.effects.motion ? 'on' : 'off'
      root.dataset.preset = theme.preset
      root.dataset.mode = light ? 'light' : 'dark'

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
      document.body.style.background = 'transparent'
      await applyWindowChrome(useAppStore().config?.settings.acrylic ?? true, theme)
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
