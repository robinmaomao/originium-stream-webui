<template>
  <div>
    <div class="d-flex ga-3 align-center mb-5 flex-wrap">
      <div>
        <div class="page-title">{{ t('logs.title') }}</div>
        <div class="page-subtitle mono">{{ logPath ?? t('logs.appLogs') }}</div>
      </div>
      <v-spacer />
      <v-btn variant="tonal" prepend-icon="mdi-download" @click="exportDiagnostics">
        {{ t('settings.diagnostics') }}
      </v-btn>
      <v-btn variant="tonal" prepend-icon="mdi-folder-open" @click="openLogDir">
        {{ t('settings.openLogDir') }}
      </v-btn>
    </div>

    <v-tabs v-model="tab" class="mb-4">
      <v-tab value="app" prepend-icon="mdi-application">{{ t('logs.appLogs') }}</v-tab>
      <v-tab value="host" prepend-icon="mdi-server">{{ t('logs.hostLogs') }}</v-tab>
    </v-tabs>

    <div v-show="tab === 'app'">
      <LogViewer :entries="logs.appLogs" :loading="logs.loading" @refresh="logs.refreshApp" @clear="logs.clear" />
    </div>

    <div v-show="tab === 'host'">
      <div class="d-flex ga-3 mb-4 flex-wrap">
        <v-select
          v-model="selectedHost"
          :items="hostOptions"
          hide-details
          density="compact"
          style="max-width: 320px"
          @update:model-value="loadHostLogs"
        />
      </div>
      <LogViewer :text="logs.hostLogs" :loading="hostLoading" @refresh="loadHostLogs" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { openPath } from '@tauri-apps/plugin-opener'
import * as api from '@/api'
import LogViewer from '@/components/LogViewer.vue'
import { useAppStore } from '@/stores/app'
import { useHostsStore } from '@/stores/hosts'
import { useLogsStore } from '@/stores/logs'

const { t } = useI18n()
const app = useAppStore()
const hosts = useHostsStore()
const logs = useLogsStore()

const tab = ref('app')
const selectedHost = ref<string>('')
const logPath = ref<string | null>(null)
const hostLoading = ref(false)
let timer: number | null = null

const hostOptions = computed(() => hosts.hosts.map((host) => ({ title: host.name, value: host.id })))

onMounted(async () => {
  await hosts.refresh()
  await logs.refreshApp()
  logPath.value = await api.getLogPath()
  selectedHost.value = hosts.hosts[0]?.id ?? ''
  timer = window.setInterval(() => {
    if (tab.value === 'app') void logs.refreshApp()
  }, 3000)
})

onUnmounted(() => {
  if (timer !== null) window.clearInterval(timer)
})

async function loadHostLogs() {
  if (!selectedHost.value) return
  hostLoading.value = true
  try {
    await logs.loadHost(selectedHost.value)
  } finally {
    hostLoading.value = false
  }
}

async function exportDiagnostics() {
  try {
    const path = await api.exportDiagnostics()
    app.notify(path, 'success')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

async function openLogDir() {
  try {
    const path = await api.getLogPath()
    await openPath(path ? path.replace(/[\\/][^\\/]+$/, '') : '.')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}
</script>
