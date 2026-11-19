<template>
  <div v-if="host">
    <div class="d-flex ga-3 align-center mb-5 flex-wrap">
      <v-btn icon variant="text" @click="router.back()">
        <v-icon>mdi-arrow-left</v-icon>
      </v-btn>
      <div class="min-width-0">
        <div class="page-title text-truncate">{{ host.name }}</div>
        <div class="page-subtitle mono">{{ host.address }}:{{ host.port }}</div>
      </div>
      <v-chip :color="host.paired ? 'success' : 'warning'" variant="tonal" size="small" class="ml-2">
        {{ host.paired ? t('common.pair') : t('hosts.pairPin') }}
      </v-chip>
      <v-spacer />
      <v-btn variant="tonal" prepend-icon="mdi-lan-connect" :loading="connecting" @click="connect">
        {{ t('hosts.test') }}
      </v-btn>
      <v-btn variant="tonal" prepend-icon="mdi-key-chain" @click="pairOpen = true">
        {{ t('common.pair') }}
      </v-btn>
      <v-btn color="error" variant="tonal" prepend-icon="mdi-stop-circle-outline" @click="closeApp">
        {{ t('hostDetail.closeApp') }}
      </v-btn>
    </div>

    <v-tabs v-model="tab" class="mb-4">
      <v-tab value="apps" prepend-icon="mdi-apps">{{ t('hostDetail.apps') }}</v-tab>
      <v-tab value="clients" prepend-icon="mdi-devices">{{ t('hostDetail.clients') }}</v-tab>
      <v-tab value="settings" prepend-icon="mdi-tune">{{ t('hostDetail.settings') }}</v-tab>
      <v-tab value="logs" prepend-icon="mdi-text-box-outline">{{ t('hostDetail.logs') }}</v-tab>
      <v-tab value="permissions" prepend-icon="mdi-shield-check">{{ t('hostDetail.permissions') }}</v-tab>
    </v-tabs>

    <div v-show="tab === 'apps'">
      <AppGrid :host-id="host.id" @launch="launch" />
    </div>

    <div v-show="tab === 'clients'">
      <div class="d-flex ga-3 align-center mb-4">
        <v-btn variant="tonal" prepend-icon="mdi-refresh" :loading="clientsLoading" @click="loadClients">
          {{ t('common.refresh') }}
        </v-btn>
        <v-spacer />
        <v-btn color="error" variant="text" @click="unpairAll">{{ t('hostDetail.unpair') }}</v-btn>
      </div>
      <v-card v-if="clients.length > 0" class="glass" elevation="0">
        <v-table density="comfortable" class="bg-transparent">
          <thead>
            <tr>
              <th>UUID</th>
              <th>{{ t('hosts.name') }}</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(client, index) in clients" :key="index">
              <td class="mono text-caption">{{ client.uuid ?? client.id ?? '—' }}</td>
              <td>{{ client.name ?? '—' }}</td>
              <td class="text-right">
                <v-btn size="small" variant="text" color="error" @click="unpair(client)">
                  {{ t('hostDetail.unpair') }}
                </v-btn>
              </td>
            </tr>
          </tbody>
        </v-table>
      </v-card>
      <v-card v-else class="glass pa-8 text-center" elevation="0">
        <div class="muted">{{ t('hostDetail.noClients') }}</div>
      </v-card>
    </div>

    <div v-show="tab === 'settings'">
      <div v-if="config">
        <ConfigEditor :config="config" :saving="configSaving" @save="saveConfig" />
      </div>
      <v-card v-else class="glass pa-8 text-center" elevation="0">
        <v-progress-circular indeterminate color="primary" />
        <div class="muted mt-3">{{ t('common.loading') }}</div>
      </v-card>
    </div>

    <div v-show="tab === 'logs'">
      <LogViewer :text="hostLogs" :loading="logsLoading" @refresh="loadLogs" />
    </div>

    <div v-show="tab === 'permissions'">
      <v-row>
        <v-col cols="12" md="6">
          <v-card class="glass pa-5 h-100" elevation="0">
            <div class="text-h6 mb-3">{{ t('hostDetail.permissions') }}</div>
            <v-list density="comfortable" class="bg-transparent">
              <v-list-item v-for="permission in permissions" :key="String(permission.id)">
                <v-list-item-title>{{ permission.id }}</v-list-item-title>
                <v-list-item-subtitle>
                  {{ permission.required ? 'Required' : 'Optional' }} ·
                  {{ permission.status }}
                </v-list-item-subtitle>
                <template #append>
                  <v-btn
                    v-if="permission.requestable"
                    size="small"
                    variant="tonal"
                    color="primary"
                    @click="requestPermission(String(permission.id))"
                  >
                    {{ t('common.confirm') }}
                  </v-btn>
                </template>
              </v-list-item>
            </v-list>
            <div v-if="permissions.length === 0" class="muted">{{ t('common.none') }}</div>
          </v-card>
        </v-col>
        <v-col cols="12" md="6">
          <v-card class="glass pa-5 h-100" elevation="0">
            <div class="text-h6 mb-3">Virtual Input</div>
            <pre class="mono text-caption json-box">{{ JSON.stringify(virtualInput, null, 2) }}</pre>
            <v-btn class="mt-3" variant="tonal" prepend-icon="mdi-monitor-screenshot" @click="resetDisplay">
              Reset display persistence
            </v-btn>
          </v-card>
        </v-col>
      </v-row>
    </div>

    <v-dialog v-model="restartDialog" max-width="420">
      <v-card class="glass pa-2">
        <v-card-title>{{ t('common.restart') }}</v-card-title>
        <v-card-text>配置已保存。需要重启 Sunshine 才能生效，是否现在重启？</v-card-text>
        <v-card-actions class="px-4 pb-3">
          <v-spacer />
          <v-btn variant="text" @click="restartDialog = false">{{ t('common.cancel') }}</v-btn>
          <v-btn color="primary" @click="restart">{{ t('common.restart') }}</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <PairDialog v-model="pairOpen" :host="host" @paired="onPaired" />
  </div>
  <v-card v-else class="glass pa-10 text-center" elevation="0">
    <v-icon size="48" color="warning">mdi-help-circle-outline</v-icon>
    <div class="text-h6 mt-3">{{ t('common.notFound') }}</div>
  </v-card>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import * as api from '@/api'
import AppGrid from '@/components/AppGrid.vue'
import ConfigEditor from '@/components/ConfigEditor.vue'
import LogViewer from '@/components/LogViewer.vue'
import PairDialog from '@/components/PairDialog.vue'
import { useAppStore } from '@/stores/app'
import { useHostsStore } from '@/stores/hosts'
import { useSessionStore } from '@/stores/session'

const props = defineProps<{ id: string }>()
const { t } = useI18n()
const app = useAppStore()
const hosts = useHostsStore()
const session = useSessionStore()
const router = useRouter()

const tab = ref('apps')
const pairOpen = ref(false)
const connecting = ref(false)
const clients = ref<Record<string, unknown>[]>([])
const clientsLoading = ref(false)
const config = ref<Record<string, unknown> | null>(null)
const configSaving = ref(false)
const restartDialog = ref(false)
const hostLogs = ref('')
const logsLoading = ref(false)
const permissions = ref<Record<string, unknown>[]>([])
const virtualInput = ref<Record<string, unknown>>({})

const host = computed(() => hosts.hosts.find((item) => item.id === props.id) ?? null)

onMounted(async () => {
  await hosts.refresh()
  void loadConfig()
})

watch(tab, (value) => {
  if (value === 'clients' && clients.value.length === 0) void loadClients()
  if (value === 'logs' && !hostLogs.value) void loadLogs()
  if (value === 'permissions' && permissions.value.length === 0) void loadPermissions()
})

async function connect() {
  connecting.value = true
  try {
    await hosts.connect(props.id)
    app.notify(t('hosts.testSuccess'), 'success')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  } finally {
    connecting.value = false
  }
}

async function launch(appName: string) {
  try {
    const profile = await api.getStreamProfile()
    await session.launch(props.id, appName, profile)
    await router.push('/session')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

async function closeApp() {
  try {
    await api.closeHostApp(props.id)
    app.notify(t('hostDetail.closeApp'), 'success')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

async function loadClients() {
  clientsLoading.value = true
  try {
    const raw = await api.getHostClients(props.id)
    const list = Array.isArray(raw) ? raw : ((raw as { clients?: unknown[] })?.clients ?? [])
    clients.value = list.filter((item): item is Record<string, unknown> => Boolean(item) && typeof item === 'object')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  } finally {
    clientsLoading.value = false
  }
}

async function unpair(client: Record<string, unknown>) {
  const uuid = String(client.uuid ?? client.id ?? '')
  if (!uuid) return
  try {
    await api.unpairClient(props.id, uuid)
    await loadClients()
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

async function unpairAll() {
  if (!window.confirm(`${t('hostDetail.unpair')}?`)) return
  try {
    await api.unpairAll(props.id)
    await loadClients()
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

async function loadConfig() {
  try {
    config.value = await api.getHostConfig(props.id)
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

async function saveConfig(updated: Record<string, unknown>) {
  configSaving.value = true
  try {
    await api.saveHostConfig(props.id, updated)
    app.notify(t('common.save'), 'success')
    restartDialog.value = true
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  } finally {
    configSaving.value = false
  }
}

async function restart() {
  restartDialog.value = false
  try {
    await api.restartHost(props.id)
    app.notify(t('common.restart'), 'success')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

async function loadLogs() {
  logsLoading.value = true
  try {
    hostLogs.value = await api.getHostLogs(props.id)
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  } finally {
    logsLoading.value = false
  }
}

async function loadPermissions() {
  try {
    const raw = await api.getPermissions(props.id)
    const list = Array.isArray(raw?.permissions) ? (raw.permissions as Record<string, unknown>[]) : []
    permissions.value = list
    virtualInput.value = await api.getVirtualInputStatus(props.id)
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

async function requestPermission(id: string) {
  try {
    await api.requestPermission(props.id, id)
    await loadPermissions()
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

async function resetDisplay() {
  try {
    await api.resetDisplayPersistence(props.id)
    app.notify(t('common.apply'), 'success')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

function onPaired() {
  app.notify(t('hosts.pairSuccess'), 'success')
}
</script>

<style scoped>
.min-width-0 {
  min-width: 0;
}
.h-100 {
  height: 100%;
}
.json-box {
  background: rgba(0, 0, 0, 0.25);
  border-radius: 12px;
  padding: 12px;
  max-height: 280px;
  overflow: auto;
}
</style>
