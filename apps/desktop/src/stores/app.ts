import { defineStore } from 'pinia'
import * as api from '@/api'
import type {
  AppConfig,
  AppSettings,
  ComponentProgress,
  EnvironmentInfo,
  InstalledComponent,
  SessionSnapshot,
} from '@/api/types'

interface ToastState {
  show: boolean
  message: string
  hint: string
  color: string
}

export const useAppStore = defineStore('app', {
  state: () => ({
    loading: true,
    config: null as AppConfig | null,
    environment: null as EnvironmentInfo | null,
    components: [] as InstalledComponent[],
    session: null as SessionSnapshot | null,
    toast: { show: false, message: '', hint: '', color: 'info' } as ToastState,
  }),
  getters: {
    settings: (state): AppSettings | null => state.config?.settings ?? null,
    hosts: (state) => state.config?.hosts ?? [],
    theme: (state) => state.config?.theme ?? null,
    onboardingComplete: (state) => state.config?.onboardingComplete ?? false,
    isStreaming: (state) =>
      state.session?.state === 'streaming' || state.session?.state === 'launching',
    appVersion: (state) => state.environment?.appVersion ?? '0.1.3',
  },
  actions: {
    notify(message: string, color = 'info', hint = '') {
      this.toast = { show: true, message, hint, color }
    },
    async bootstrap() {
      this.loading = true
      try {
        const state = await api.getBootstrap()
        this.config = state.config
        this.components = state.components
        this.environment = state.environment
        this.session = state.session
      } catch (error) {
        this.notify(api.errorMessage(error), 'error')
      } finally {
        this.loading = false
      }
    },
    async refreshEnvironment() {
      try {
        this.environment = await api.getEnvironment()
        this.components = await api.listComponents()
      } catch (error) {
        this.notify(api.errorMessage(error), 'error')
      }
    },
    setConfig(config: AppConfig) {
      this.config = config
    },
    setSession(snapshot: SessionSnapshot) {
      this.session = snapshot
    },
    async saveSettings(settings: AppSettings) {
      const saved = await api.saveAppSettings(settings)
      if (this.config) this.config.settings = saved
      return saved
    },
    async completeOnboarding() {
      await api.completeOnboarding()
      if (this.config) this.config.onboardingComplete = true
    },
    async autoStartSunshineService() {
      if (!this.settings?.autoStartServices) return
      try {
        const status = await api.startSunshineService()
        if (status.found) await this.refreshEnvironment()
      } catch (error) {
        this.notify(api.errorMessage(error), 'warning')
      }
    },
    async installComponent(id: string, onProgress?: (progress: ComponentProgress) => void) {
      let installed
      if (onProgress) {
        const { listen } = await import('@tauri-apps/api/event')
        const unlisten = await listen<ComponentProgress>('component://progress', (event) => {
          if (event.payload.id === id) onProgress(event.payload)
        })
        try {
          installed = await api.installComponent(id)
        } finally {
          unlisten()
        }
      } else {
        installed = await api.installComponent(id)
      }
      this.components = [...this.components.filter((component) => component.id !== id), installed]
      return installed
    },
    async removeComponent(id: string) {
      await api.removeComponent(id)
      this.components = this.components.filter((component) => component.id !== id)
    },
  },
})
