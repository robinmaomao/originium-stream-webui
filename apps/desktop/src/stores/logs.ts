import { defineStore } from 'pinia'
import * as api from '@/api'
import type { LogEntry } from '@/api/types'
import { useAppStore } from './app'

export const useLogsStore = defineStore('logs', {
  state: () => ({
    appLogs: [] as LogEntry[],
    hostLogs: '' as string,
    level: 'all' as string,
    search: '',
    loading: false,
  }),
  getters: {
    filteredAppLogs: (state) => {
      const needle = state.search.trim().toLowerCase()
      return state.appLogs.filter((entry) => {
        const levelMatch = state.level === 'all' || entry.level.toLowerCase() === state.level
        const searchMatch =
          !needle ||
          entry.message.toLowerCase().includes(needle) ||
          entry.target.toLowerCase().includes(needle)
        return levelMatch && searchMatch
      })
    },
  },
  actions: {
    async refreshApp() {
      this.loading = true
      try {
        this.appLogs = await api.getAppLogs(1000)
      } catch (error) {
        useAppStore().notify(api.errorMessage(error), 'error')
      } finally {
        this.loading = false
      }
    },
    async loadHost(hostId: string) {
      try {
        this.hostLogs = await api.getHostLogs(hostId)
      } catch (error) {
        useAppStore().notify(api.errorMessage(error), 'error')
      }
    },
    async clear() {
      try {
        await api.clearAppLogs()
        this.appLogs = []
      } catch (error) {
        useAppStore().notify(api.errorMessage(error), 'error')
      }
    },
  },
})
