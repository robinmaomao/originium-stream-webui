<template>
  <div class="hud-window">
    <div class="hud-card" data-tauri-drag-region>
      <div class="d-flex align-center ga-3 no-drag">
        <div class="hud-orb" :class="stateClass">
          <v-icon size="20">{{ stateIcon }}</v-icon>
        </div>
        <div class="min-width-0">
          <div class="text-body-2 font-weight-bold text-truncate">
            {{ snapshot?.appName ?? t('session.noSession') }}
          </div>
          <div class="text-caption muted text-truncate">
            {{ snapshot?.hostName ?? t('session.host') }} · {{ t(`session.${session.stateLabel}`) }}
          </div>
        </div>
        <v-spacer />
        <v-chip size="small" variant="tonal" :color="stateColor" class="mono">{{ elapsed }}</v-chip>
      </div>

      <div class="d-flex align-center ga-2 mt-2 no-drag">
        <v-btn
          v-if="active"
          size="small"
          color="error"
          variant="flat"
          prepend-icon="mdi-stop"
          :loading="stopping"
          @click="stop"
        >
          {{ t('hud.stop') }}
        </v-btn>
        <v-spacer />
        <v-btn size="small" variant="text" icon @click="hide">
          <v-icon icon="mdi-close" />
        </v-btn>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import * as api from '@/api'
import { useSessionStore } from '@/stores/session'

const { t } = useI18n()
const session = useSessionStore()

const stopping = ref(false)
const now = ref(Date.now())
let timer: number | null = null

const snapshot = computed(() => session.snapshot)
const active = computed(() => session.active)
const stateClass = computed(() => snapshot.value?.state ?? 'idle')
const stateColor = computed(() => (active.value ? 'success' : 'info'))
const stateIcon = computed(() => (active.value ? 'mdi-broadcast' : 'mdi-pause'))
const elapsed = computed(() => {
  if (!snapshot.value?.startedAt) return '--:--'
  const seconds = Math.max(0, Math.floor(now.value / 1000 - snapshot.value.startedAt))
  const m = Math.floor(seconds / 60)
  const s = seconds % 60
  return `${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`
})

onMounted(() => {
  void session.refresh()
  timer = window.setInterval(() => {
    now.value = Date.now()
    void session.refresh()
  }, 2000)
})

onUnmounted(() => {
  if (timer !== null) window.clearInterval(timer)
})

async function stop() {
  stopping.value = true
  try {
    await session.stop()
  } catch (error) {
    console.error(api.errorMessage(error))
  } finally {
    stopping.value = false
  }
}

async function hide() {
  await getCurrentWebviewWindow().hide()
}
</script>

<style scoped>
.hud-orb {
  width: 36px;
  height: 36px;
  border-radius: 12px;
  display: grid;
  place-items: center;
  background: rgba(124, 77, 255, 0.25);
  flex-shrink: 0;
}
.hud-orb.streaming {
  background: rgba(74, 222, 128, 0.24);
  color: var(--originium-success);
}
.min-width-0 {
  min-width: 0;
}
</style>
