<template>
  <div>
    <div class="d-flex ga-3 align-center mb-5 flex-wrap">
      <div>
        <div class="page-title">{{ t('session.title') }}</div>
        <div class="page-subtitle">{{ t('session.subtitle') }}</div>
      </div>
      <v-spacer />
      <v-btn variant="tonal" prepend-icon="mdi-monitor-eye" @click="toggleHud">
        {{ t('session.toggleHud') }}
      </v-btn>
      <v-btn color="error" prepend-icon="mdi-stop-circle" :disabled="!active" :loading="stopping" @click="stop">
        {{ t('session.stop') }}
      </v-btn>
    </div>

    <v-row v-if="snapshot && snapshot.state !== 'idle'">
      <v-col cols="12" lg="7">
        <v-card class="glass pa-6" elevation="0">
          <div class="d-flex align-center ga-3 mb-5">
            <div class="state-orb" :class="stateClass">
              <v-icon size="26">{{ stateIcon }}</v-icon>
            </div>
            <div>
              <div class="text-h6">{{ t(`session.${session.stateLabel}`) }}</div>
              <div class="text-caption muted">{{ snapshot.appName ?? '—' }} · {{ snapshot.hostName ?? '—' }}</div>
            </div>
            <v-spacer />
            <v-chip variant="tonal" :color="stateColor">{{ elapsed }}</v-chip>
          </div>

          <v-row dense>
            <v-col cols="12" md="6">
              <div class="section-label">{{ t('session.host') }}</div>
              <div class="mono">{{ snapshot.hostName ?? '—' }}</div>
            </v-col>
            <v-col cols="12" md="6">
              <div class="section-label">{{ t('session.app') }}</div>
              <div>{{ snapshot.appName ?? '—' }}</div>
            </v-col>
            <v-col cols="12" md="6">
              <div class="section-label">{{ t('session.startedAt') }}</div>
              <div class="mono">{{ startedAt }}</div>
            </v-col>
            <v-col cols="12" md="6">
              <div class="section-label">{{ t('session.pid') }}</div>
              <div class="mono">{{ snapshot.pid ?? '—' }}</div>
            </v-col>
          </v-row>

          <v-alert v-if="snapshot.error" type="error" variant="tonal" class="mt-4">
            {{ snapshot.error }}
          </v-alert>
        </v-card>
      </v-col>

      <v-col cols="12" lg="5">
        <v-card class="glass pa-5 h-100" elevation="0">
          <div class="d-flex align-center mb-3">
            <div class="text-h6">{{ t('session.logTail') }}</div>
            <v-spacer />
            <v-btn icon variant="text" @click="refresh">
              <v-icon icon="mdi-refresh" />
            </v-btn>
          </div>
          <pre class="mono text-caption log-tail">{{ (snapshot.logTail ?? []).join('\n') || '—' }}</pre>
        </v-card>
      </v-col>
    </v-row>

    <v-card v-else class="glass pa-12 text-center" elevation="0">
      <v-icon size="64" color="primary" class="mb-4" icon="mdi-play-circle-outline" />
      <div class="text-h5">{{ t('session.noSession') }}</div>
      <div class="muted mt-2">{{ t('hostDetail.launchHint') }}</div>
      <v-btn class="mt-6" color="primary" to="/hosts" prepend-icon="mdi-lan">
        {{ t('hosts.title') }}
      </v-btn>
    </v-card>

    <v-alert type="info" variant="tonal" class="mt-6" density="comfortable">
      {{ t('session.hudHint') }}
    </v-alert>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import * as api from '@/api'
import { useAppStore } from '@/stores/app'
import { useSessionStore } from '@/stores/session'

const { t } = useI18n()
const app = useAppStore()
const session = useSessionStore()

const stopping = ref(false)
const now = ref(Date.now())
let timer: number | null = null

const snapshot = computed(() => session.snapshot)
const active = computed(() => session.active)
const elapsed = computed(() => {
  if (!snapshot.value?.startedAt) return '—'
  const seconds = Math.max(0, Math.floor(now.value / 1000 - snapshot.value.startedAt))
  const h = Math.floor(seconds / 3600)
  const m = Math.floor((seconds % 3600) / 60)
  const s = seconds % 60
  return [h, m, s].map((value) => String(value).padStart(2, '0')).join(':')
})
const startedAt = computed(() =>
  snapshot.value?.startedAt ? new Date(snapshot.value.startedAt * 1000).toLocaleString() : '—',
)
const stateColor = computed(() => {
  switch (snapshot.value?.state) {
    case 'streaming':
      return 'success'
    case 'launching':
    case 'stopping':
      return 'warning'
    case 'error':
      return 'error'
    default:
      return 'info'
  }
})
const stateClass = computed(() => snapshot.value?.state ?? 'idle')
const stateIcon = computed(() => {
  switch (snapshot.value?.state) {
    case 'streaming':
      return 'mdi-broadcast'
    case 'launching':
      return 'mdi-loading'
    case 'error':
      return 'mdi-alert-circle'
    default:
      return 'mdi-pause-circle-outline'
  }
})

onMounted(() => {
  void session.refresh()
  timer = window.setInterval(() => {
    now.value = Date.now()
  }, 1000)
})

onUnmounted(() => {
  if (timer !== null) window.clearInterval(timer)
})

async function refresh() {
  await session.refresh()
}

async function stop() {
  stopping.value = true
  try {
    await session.stop()
    app.notify(t('session.stop'), 'success')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  } finally {
    stopping.value = false
  }
}

async function toggleHud() {
  try {
    await api.toggleHud()
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}
</script>

<style scoped>
.state-orb {
  width: 52px;
  height: 52px;
  border-radius: 18px;
  display: grid;
  place-items: center;
  background: rgba(124, 77, 255, 0.2);
  color: #fff;
}
.state-orb.streaming {
  background: rgba(74, 222, 128, 0.2);
  color: var(--originium-success);
}
.state-orb.error {
  background: rgba(255, 83, 112, 0.2);
  color: var(--originium-error);
}
.log-tail {
  margin: 0;
  max-height: 380px;
  overflow: auto;
  white-space: pre-wrap;
  word-break: break-all;
  color: rgba(232, 232, 240, 0.75);
}
.h-100 {
  height: 100%;
}
</style>
