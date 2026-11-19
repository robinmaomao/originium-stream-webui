<template>
  <div>
    <div class="d-flex ga-3 align-center mb-5 flex-wrap">
      <div>
        <div class="page-title">{{ t('local.title') }}</div>
        <div class="page-subtitle">{{ t('local.subtitle') }}</div>
      </div>
      <v-spacer />
      <v-btn variant="tonal" prepend-icon="mdi-refresh" :loading="refreshing" @click="refresh">
        {{ t('common.refresh') }}
      </v-btn>
    </div>

    <v-row>
      <v-col cols="12" lg="6">
        <v-card class="glass pa-5 h-100" elevation="0">
          <div class="d-flex align-center ga-3 mb-4">
            <v-icon color="primary" size="28" icon="mdi-desktop-tower-monitor" />
            <div>
              <div class="text-h6">{{ t('local.environment') }}</div>
              <div class="text-caption muted">{{ environment?.platform }} / {{ environment?.arch }}</div>
            </div>
            <v-spacer />
            <v-chip :color="environment?.isAdmin ? 'success' : 'warning'" variant="tonal" size="small">
              {{ environment?.isAdmin ? t('local.admin') : t('local.notAdmin') }}
            </v-chip>
          </div>
          <div class="info-grid">
            <div class="info-item">
              <span class="section-label">{{ t('hosts.name') }}</span>
              <span>{{ environment?.hostname ?? '—' }}</span>
            </div>
            <div class="info-item">
              <span class="section-label">{{ t('local.version') }}</span>
              <span class="mono">{{ environment?.appVersion }}</span>
            </div>
            <div class="info-item">
              <span class="section-label">{{ t('local.webview2') }}</span>
              <span class="mono">{{ environment?.webview2Version ?? '—' }}</span>
            </div>
            <div class="info-item">
              <span class="section-label">{{ t('settings.components') }}</span>
              <div class="d-flex ga-2 flex-wrap mt-1">
                <v-chip size="small" variant="tonal" :color="sunshineInstalled ? 'success' : 'warning'">
                  Sunshine {{ sunshineVersion }}
                </v-chip>
                <v-chip size="small" variant="tonal" :color="moonlightInstalled ? 'success' : 'warning'">
                  Moonlight {{ moonlightVersion }}
                </v-chip>
              </div>
            </div>
          </div>
        </v-card>
      </v-col>

      <v-col cols="12" lg="6">
        <v-card class="glass pa-5 h-100" elevation="0">
          <div class="d-flex align-center ga-3 mb-4">
            <v-icon :color="serviceRunning ? 'success' : 'warning'" size="28" icon="mdi-weather-sunny" />
            <div>
              <div class="text-h6">{{ t('local.service') }}</div>
              <div class="text-caption muted">
                {{ service?.name ?? t('local.serviceMissing') }}
              </div>
            </div>
            <v-spacer />
            <v-chip :color="serviceRunning ? 'success' : service?.found ? 'warning' : 'default'" variant="tonal" size="small">
              {{ serviceLabel }}
            </v-chip>
          </div>

          <div class="d-flex ga-2 flex-wrap">
            <v-btn
              v-if="!sunshineInstalled"
              color="primary"
              prepend-icon="mdi-download"
              :loading="installing === 'sunshine'"
              @click="installComponent('sunshine')"
            >
              {{ t('local.installSunshine') }}
            </v-btn>
            <v-btn
              v-if="sunshineInstalled && !service?.found"
              color="primary"
              prepend-icon="mdi-cog-transfer"
              :loading="installing === 'service'"
              @click="installService"
            >
              {{ t('local.installService') }}
            </v-btn>
            <v-btn
              v-if="service?.found && !serviceRunning"
              color="success"
              prepend-icon="mdi-play"
              :loading="serviceBusy"
              @click="startService"
            >
              {{ t('common.start') }}
            </v-btn>
            <v-btn
              v-if="serviceRunning"
              color="warning"
              prepend-icon="mdi-stop"
              :loading="serviceBusy"
              @click="stopService"
            >
              {{ t('common.stop') }}
            </v-btn>
            <v-btn variant="tonal" prepend-icon="mdi-firewall" :loading="installing === 'firewall'" @click="addFirewall">
              {{ t('local.addFirewall') }}
            </v-btn>
            <v-btn variant="tonal" prepend-icon="mdi-account-key" @click="credentialsDialog = true">
              {{ t('local.credentials') }}
            </v-btn>
          </div>

          <v-progress-linear
            v-if="progress"
            :model-value="progress.percent"
            color="primary"
            height="6"
            rounded
            class="mt-4"
          />
          <div v-if="progress" class="text-caption muted mt-1">{{ progress.message }}</div>
        </v-card>
      </v-col>

      <v-col cols="12" lg="6">
        <v-card class="glass pa-5 h-100" elevation="0">
          <div class="d-flex align-center ga-3 mb-4">
            <v-icon color="info" size="28" icon="mdi-lan" />
            <div class="text-h6">{{ t('local.addresses') }}</div>
          </div>
          <div class="d-flex ga-2 flex-wrap">
            <v-chip
              v-for="ip in environment?.lanIps ?? []"
              :key="ip"
              variant="tonal"
              class="mono"
              @click="copy(ip)"
            >
              https://{{ ip }}:47990
            </v-chip>
            <span v-if="!environment?.lanIps?.length" class="muted">{{ t('common.none') }}</span>
          </div>
          <div class="text-caption muted mt-3">
            其他设备可在「主机」页通过 mDNS 自动发现本机。
          </div>
        </v-card>
      </v-col>

      <v-col cols="12" lg="6">
        <v-card class="glass pa-5 h-100" elevation="0">
          <div class="d-flex align-center ga-3 mb-4">
            <v-icon color="secondary" size="28" icon="mdi-key-chain" />
            <div>
              <div class="text-h6">{{ t('local.pending') }}</div>
              <div class="text-caption muted">{{ localHost?.name ?? t('local.credentials') }}</div>
            </div>
            <v-spacer />
            <v-btn icon variant="text" :loading="pairingLoading" @click="loadPairings">
              <v-icon icon="mdi-refresh" />
            </v-btn>
          </div>

          <v-alert v-if="!localHost" type="info" variant="tonal" density="comfortable">
            请先设置 Sunshine 账号并将本机添加为主机
          </v-alert>
          <v-alert v-else-if="pairings.length === 0" type="info" variant="tonal" density="comfortable">
            {{ t('local.noPending') }}
          </v-alert>
          <v-list v-else density="comfortable" class="bg-transparent">
            <v-list-item v-for="pairing in pairings" :key="pairing.id" rounded="lg">
              <template #prepend>
                <v-icon color="primary" icon="mdi-cellphone-link" />
              </template>
              <v-list-item-title>{{ pairing.name }}</v-list-item-title>
              <v-list-item-subtitle class="mono">{{ pairing.address }}</v-list-item-subtitle>
              <template #append>
                <v-btn size="small" color="primary" variant="tonal" @click="openApprove(pairing)">
                  {{ t('local.approve') }}
                </v-btn>
              </template>
            </v-list-item>
          </v-list>
        </v-card>
      </v-col>
    </v-row>

    <v-dialog v-model="credentialsDialog" max-width="460">
      <v-card class="glass pa-2">
        <v-card-title>{{ t('local.credentials') }}</v-card-title>
        <v-card-text>
          <v-text-field v-model="credentials.username" :label="t('local.username')" autocomplete="username" />
          <v-text-field
            v-model="credentials.password"
            type="password"
            :label="t('local.password')"
            autocomplete="new-password"
          />
          <v-alert v-if="credentialError" type="error" variant="tonal" class="mt-3">{{ credentialError }}</v-alert>
        </v-card-text>
        <v-card-actions class="px-4 pb-3">
          <v-spacer />
          <v-btn variant="text" @click="credentialsDialog = false">{{ t('common.cancel') }}</v-btn>
          <v-btn color="primary" :loading="savingCredentials" @click="saveCredentials">{{ t('common.save') }}</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog v-model="approveDialog" max-width="420">
      <v-card class="glass pa-2">
        <v-card-title>{{ t('local.approve') }} · {{ approving?.name }}</v-card-title>
        <v-card-text>
          <p class="text-body-2 muted mb-4">{{ t('hosts.pairHint') }}</p>
          <v-text-field v-model="pin" :label="t('local.pin')" maxlength="4" inputmode="numeric" class="mono" />
          <v-alert v-if="approveError" type="error" variant="tonal" class="mt-3">{{ approveError }}</v-alert>
        </v-card-text>
        <v-card-actions class="px-4 pb-3">
          <v-spacer />
          <v-btn variant="text" @click="approveDialog = false">{{ t('common.cancel') }}</v-btn>
          <v-btn color="primary" :loading="approvingBusy" :disabled="pin.length !== 4" @click="approve">
            {{ t('local.approve') }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, reactive, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import * as api from '@/api'
import type { ComponentProgress, PendingPairing, ServiceStatus } from '@/api/types'
import { useAppStore } from '@/stores/app'
import { useHostsStore } from '@/stores/hosts'

const { t } = useI18n()
const app = useAppStore()
const hosts = useHostsStore()

const refreshing = ref(false)
const installing = ref('')
const progress = ref<ComponentProgress | null>(null)
const service = ref<ServiceStatus | null>(null)
const serviceBusy = ref(false)
const pairings = ref<PendingPairing[]>([])
const pairingLoading = ref(false)
const credentialsDialog = ref(false)
const credentialError = ref('')
const savingCredentials = ref(false)
const credentials = reactive({ username: '', password: '' })
const approveDialog = ref(false)
const approving = ref<PendingPairing | null>(null)
const approvingBusy = ref(false)
const approveError = ref('')
const pin = ref('')

let pollTimer: number | null = null

const environment = computed(() => app.environment)
const localHost = computed(() => hosts.localHost)
const sunshineInstalled = computed(() => Boolean(environment.value?.sunshineInstalled))
const moonlightInstalled = computed(() => Boolean(environment.value?.moonlightInstalled))
const sunshineVersion = computed(
  () => app.components.find((component) => component.id === 'sunshine')?.version ?? '—',
)
const moonlightVersion = computed(
  () => app.components.find((component) => component.id === 'moonlight')?.version ??
    environment.value?.moonlightVersion ?? '—',
)
const serviceRunning = computed(() => service.value?.state?.toLowerCase() === 'running')
const serviceLabel = computed(() => {
  if (!service.value?.found) return t('local.serviceMissing')
  return serviceRunning.value ? t('local.serviceRunning') : service.value.state ?? t('local.serviceStopped')
})

onMounted(async () => {
  await refresh()
  await loadPairings()
  pollTimer = window.setInterval(() => {
    void loadPairings()
  }, 5000)
})

onUnmounted(() => {
  if (pollTimer !== null) window.clearInterval(pollTimer)
})

async function refresh() {
  refreshing.value = true
  try {
    await app.refreshEnvironment()
    await hosts.refresh()
    service.value = await api.getServiceStatus()
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  } finally {
    refreshing.value = false
  }
}

async function installComponent(id: string) {
  installing.value = id
  progress.value = null
  try {
    await app.installComponent(id, (event) => {
      progress.value = event
    })
    await app.refreshEnvironment()
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  } finally {
    installing.value = ''
    progress.value = null
  }
}

async function installService() {
  installing.value = 'service'
  try {
    service.value = await api.installSunshineService()
    app.notify(t('local.installService'), 'success')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  } finally {
    installing.value = ''
  }
}

async function startService() {
  serviceBusy.value = true
  try {
    service.value = await api.startSunshineService()
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  } finally {
    serviceBusy.value = false
  }
}

async function stopService() {
  serviceBusy.value = true
  try {
    service.value = await api.stopSunshineService()
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  } finally {
    serviceBusy.value = false
  }
}

async function addFirewall() {
  installing.value = 'firewall'
  try {
    await api.addFirewallRules()
    app.notify(t('local.addFirewall'), 'success')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  } finally {
    installing.value = ''
  }
}

async function loadPairings() {
  if (!localHost.value) return
  pairingLoading.value = true
  try {
    pairings.value = await api.getPendingPairings(localHost.value.id)
  } catch {
    pairings.value = []
  } finally {
    pairingLoading.value = false
  }
}

function openApprove(pairing: PendingPairing) {
  approving.value = pairing
  pin.value = ''
  approveError.value = ''
  approveDialog.value = true
}

async function approve() {
  if (!localHost.value || !approving.value) return
  approvingBusy.value = true
  approveError.value = ''
  try {
    const result = await api.submitPin(
      localHost.value.id,
      approving.value.id,
      pin.value,
      approving.value.name,
    )
    const ok = Boolean((result as { status?: boolean })?.status)
    if (ok) {
      app.notify(t('hosts.pairSuccess'), 'success')
      approveDialog.value = false
      await loadPairings()
    } else {
      approveError.value = t('hosts.pairFailed')
    }
  } catch (error) {
    approveError.value = api.errorMessage(error)
  } finally {
    approvingBusy.value = false
  }
}

async function saveCredentials() {
  if (!credentials.username.trim() || !credentials.password) {
    credentialError.value = '用户名与密码不能为空'
    return
  }
  savingCredentials.value = true
  credentialError.value = ''
  try {
    await api.setSunshineCredentials(credentials.username.trim(), credentials.password)
    const existing = localHost.value
    const payload = {
      id: existing?.id,
      name: existing?.name ?? environment.value?.hostname ?? 'This PC',
      address: existing?.address ?? '127.0.0.1',
      port: existing?.port ?? 47990,
      username: credentials.username.trim(),
      password: credentials.password,
      source: existing?.source ?? ('manual' as const),
    }
    if (existing) await hosts.update(payload)
    else await hosts.add(payload)
    app.notify(t('onboarding.hostAdded'), 'success')
    credentialsDialog.value = false
    credentials.password = ''
  } catch (error) {
    credentialError.value = api.errorMessage(error)
  } finally {
    savingCredentials.value = false
  }
}

async function copy(text: string) {
  try {
    await navigator.clipboard.writeText(text)
    app.notify(t('common.copied'), 'success')
  } catch {
    // clipboard may be unavailable
  }
}
</script>

<style scoped>
.info-grid {
  display: grid;
  gap: 16px;
}
.info-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}
.h-100 {
  height: 100%;
}
</style>
