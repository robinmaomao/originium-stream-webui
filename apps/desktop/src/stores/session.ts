import { defineStore } from 'pinia'
import * as api from '@/api'
import type { SessionSnapshot, StreamProfile } from '@/api/types'

let pollHandle: number | null = null

export const useSessionStore = defineStore('session', {
  state: () => ({
    snapshot: null as SessionSnapshot | null,
    launching: false,
  }),
  getters: {
    active: (store) => store.snapshot?.state === 'streaming' || store.snapshot?.state === 'launching',
    stateLabel: (store) => {
      switch (store.snapshot?.state) {
        case 'launching':
          return 'launching'
        case 'streaming':
          return 'streaming'
        case 'stopping':
          return 'stopping'
        case 'error':
          return 'error'
        default:
          return 'idle'
      }
    },
  },
  actions: {
    async refresh() {
      try {
        this.snapshot = await api.getSession()
      } catch {
        // polling errors are non-fatal
      }
    },
    async launch(hostId: string, appName: string, profile?: StreamProfile) {
      this.launching = true
      try {
        this.snapshot = await api.launchSession(hostId, appName, profile)
        return this.snapshot
      } finally {
        this.launching = false
      }
    },
    async stop() {
      this.snapshot = await api.stopSession()
      return this.snapshot
    },
    startPolling() {
      if (pollHandle !== null) return
      pollHandle = window.setInterval(() => {
        void this.refresh()
      }, 2500)
    },
    stopPolling() {
      if (pollHandle !== null) {
        window.clearInterval(pollHandle)
        pollHandle = null
      }
    },
  },
})
