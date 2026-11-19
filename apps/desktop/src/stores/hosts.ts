import { defineStore } from 'pinia'
import * as api from '@/api'
import type { DiscoveredHost, Host, HostInput, PairingResult } from '@/api/types'
import { useAppStore } from './app'

export const useHostsStore = defineStore('hosts', {
  state: () => ({
    hosts: [] as Host[],
    discovered: [] as DiscoveredHost[],
    activeId: null as string | null,
    scanning: false,
    loading: false,
  }),
  getters: {
    activeHost: (state) => state.hosts.find((host) => host.id === state.activeId) ?? null,
    localHost: (state) =>
      state.hosts.find(
        (host) => host.address === '127.0.0.1' || host.address === 'localhost',
      ) ?? null,
  },
  actions: {
    async refresh() {
      this.loading = true
      try {
        this.hosts = await api.listHosts()
        const app = useAppStore()
        if (app.config) app.config.hosts = this.hosts
      } catch (error) {
        useAppStore().notify(api.errorMessage(error), 'error')
      } finally {
        this.loading = false
      }
    },
    async discover(timeoutMs = 3000) {
      this.scanning = true
      try {
        this.discovered = await api.discoverHosts(timeoutMs)
      } catch (error) {
        useAppStore().notify(api.errorMessage(error), 'error')
      } finally {
        this.scanning = false
      }
    },
    async add(input: HostInput) {
      const host = await api.addHost(input)
      await this.refresh()
      return host
    },
    async update(input: HostInput) {
      const host = await api.updateHost(input)
      await this.refresh()
      return host
    },
    async remove(id: string) {
      await api.removeHost(id)
      await this.refresh()
    },
    async addDiscovered(discovered: DiscoveredHost) {
      const host = await api.addDiscoveredHost(discovered)
      await this.refresh()
      return host
    },
    async setActive(id: string | null) {
      this.activeId = id
      await api.setActiveHost(id)
    },
    async test(input: HostInput) {
      return api.testHost(input)
    },
    async connect(id: string) {
      const config = await api.connectHost(id)
      this.activeId = id
      await api.setActiveHost(id)
      return config
    },
    async pair(hostId: string, pin: string, name?: string): Promise<PairingResult> {
      const result = await api.pairWithPin(hostId, pin, name)
      await this.refresh()
      return result
    },
    async resetFingerprint(id: string) {
      await api.resetHostFingerprint(id)
      await this.refresh()
    },
  },
})
