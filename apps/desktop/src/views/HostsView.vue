<template>
  <div>
    <div class="d-flex ga-3 align-center mb-5 flex-wrap">
      <div>
        <div class="page-title">{{ t('hosts.title') }}</div>
        <div class="page-subtitle">{{ t('hosts.subtitle') }}</div>
      </div>
      <v-spacer />
      <v-btn variant="tonal" prepend-icon="mdi-radar" :loading="hosts.scanning" @click="scan">
        {{ t('common.scan') }}
      </v-btn>
      <v-btn color="primary" prepend-icon="mdi-plus" @click="openAdd">
        {{ t('hosts.addManual') }}
      </v-btn>
    </div>

    <v-alert type="info" variant="tonal" density="comfortable" class="mb-5">
      {{ t('hosts.scanHint') }}
    </v-alert>

    <div v-if="hosts.discovered.length > 0" class="mb-8">
      <div class="section-label mb-3">{{ t('hosts.discovered') }}</div>
      <v-row>
        <v-col v-for="device in hosts.discovered" :key="`${device.address}:${device.port}`" cols="12" md="6" lg="4">
          <v-card class="glass pa-4" elevation="0">
            <div class="d-flex align-center ga-3">
              <v-icon color="secondary" icon="mdi-access-point" />
              <div class="min-width-0">
                <div class="text-body-1 font-weight-medium text-truncate">{{ device.name }}</div>
                <div class="text-caption muted mono">{{ device.address }}:{{ device.port }}</div>
              </div>
              <v-spacer />
              <v-btn size="small" color="primary" variant="tonal" @click="addDiscovered(device)">
                {{ t('common.add') }}
              </v-btn>
            </div>
          </v-card>
        </v-col>
      </v-row>
    </div>

    <div class="section-label mb-3">{{ t('hosts.saved') }}</div>
    <v-row v-if="hosts.hosts.length > 0">
      <v-col v-for="host in hosts.hosts" :key="host.id" cols="12" md="6" lg="4">
        <HostCard
          :host="host"
          @connect="connect"
          @pair="openPair"
          @edit="openEdit"
          @remove="remove"
          @detail="detail"
          @reset-fingerprint="resetFingerprint"
        />
      </v-col>
    </v-row>
    <v-card v-else class="glass pa-10 text-center" elevation="0">
      <v-icon size="52" color="primary" class="mb-3" icon="mdi-lan-disconnect" />
      <div class="text-h6">{{ t('hosts.noHosts') }}</div>
      <div class="muted mt-2">{{ t('hosts.noDiscovered') }}</div>
    </v-card>

    <HostFormDialog v-model="formOpen" :host="editingHost" @saved="onSaved" />
    <PairDialog v-model="pairOpen" :host="pairingHost" @paired="onPaired" />
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import * as api from '@/api'
import type { DiscoveredHost, Host } from '@/api/types'
import HostCard from '@/components/HostCard.vue'
import HostFormDialog from '@/components/HostFormDialog.vue'
import PairDialog from '@/components/PairDialog.vue'
import { useAppStore } from '@/stores/app'
import { useHostsStore } from '@/stores/hosts'

const { t } = useI18n()
const app = useAppStore()
const hosts = useHostsStore()
const router = useRouter()

const formOpen = ref(false)
const pairOpen = ref(false)
const editingHost = ref<Host | null>(null)
const pairingHost = ref<Host | null>(null)

onMounted(async () => {
  await hosts.refresh()
  void scan()
})

async function scan() {
  await hosts.discover(3000)
}

function openAdd() {
  editingHost.value = null
  formOpen.value = true
}

function openEdit(host: Host) {
  editingHost.value = host
  formOpen.value = true
}

function openPair(host: Host) {
  pairingHost.value = host
  pairOpen.value = true
}

function onSaved(host: Host) {
  editingHost.value = host
}

async function addDiscovered(device: DiscoveredHost) {
  try {
    await hosts.addDiscovered(device)
    app.notify(t('common.add'), 'success')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

async function connect(host: Host) {
  try {
    await hosts.connect(host.id)
    app.notify(t('hosts.testSuccess'), 'success')
    await router.push(`/host/${host.id}`)
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

async function remove(host: Host) {
  if (!window.confirm(`${t('common.delete')} ${host.name}?`)) return
  try {
    await hosts.remove(host.id)
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

async function resetFingerprint(host: Host) {
  try {
    await hosts.resetFingerprint(host.id)
    app.notify(t('hosts.fingerprintReset'), 'success')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

async function detail(host: Host) {
  try {
    await hosts.setActive(host.id)
    await router.push(`/host/${host.id}`)
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
</style>
