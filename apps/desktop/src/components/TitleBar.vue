<template>
  <header class="titlebar" data-tauri-drag-region>
    <div class="titlebar-brand no-drag" @click="router.push('/local')">
      <div class="brand-mark">◆</div>
      <div class="brand-copy">
        <div class="brand-name">Originium <span class="text-gradient">Stream</span></div>
        <div class="brand-version mono">v{{ app.appVersion }}</div>
      </div>
    </div>

    <div class="titlebar-divider no-drag" />

    <div class="titlebar-heading">
      <div class="titlebar-title">{{ title }}</div>
      <div class="titlebar-sub">{{ subtitle }}</div>
    </div>

    <v-spacer />

    <v-chip
      v-if="session.active"
      color="primary"
      variant="flat"
      size="small"
      class="session-chip no-drag"
      @click="router.push('/session')"
    >
      <span class="status-dot online" />
      {{ t(`session.${session.stateLabel}`) }}
      <span v-if="session.snapshot?.appName" class="ml-1 opacity-80">
        {{ session.snapshot.appName }}
      </span>
    </v-chip>

    <div class="titlebar-actions no-drag">
      <v-btn
        icon
        variant="text"
        size="small"
        :title="isLight ? t('theme.toDark') : t('theme.toLight')"
        @click="toggleTheme"
      >
        <v-icon :icon="isLight ? 'mdi-weather-night' : 'mdi-white-balance-sunny'" />
      </v-btn>
      <v-btn icon variant="text" size="small" :title="t('session.toggleHud')" @click="toggleHud">
        <v-icon icon="mdi-monitor-eye" />
      </v-btn>
      <v-btn icon variant="text" size="small" to="/logs" :title="t('nav.logs')">
        <v-icon icon="mdi-text-box-outline" />
      </v-btn>
      <v-btn icon variant="text" size="small" to="/settings" :title="t('nav.settings')">
        <v-icon icon="mdi-cog-outline" />
      </v-btn>
    </div>

    <div class="window-controls no-drag">
      <button type="button" class="window-button" :title="t('window.minimize')" @click="minimize">
        <v-icon icon="mdi-window-minimize" size="16" />
      </button>
      <button type="button" class="window-button" :title="t('window.maximize')" @click="toggleMaximize">
        <v-icon :icon="isMaximized ? 'mdi-window-restore' : 'mdi-window-maximize'" size="15" />
      </button>
      <button type="button" class="window-button close" :title="t('window.close')" @click="close">
        <v-icon icon="mdi-close" size="17" />
      </button>
    </div>
  </header>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useAppStore } from '@/stores/app'
import { useSessionStore } from '@/stores/session'
import { useThemeStore } from '@/stores/theme'
import { toggleHud } from '@/api'
import { isLightTheme } from '@/utils/theme'

defineProps<{ title: string; subtitle: string }>()

const app = useAppStore()
const session = useSessionStore()
const themeStore = useThemeStore()
const router = useRouter()
const { t } = useI18n()
const appWindow = getCurrentWindow()
const isMaximized = ref(false)
let unlistenResize: (() => void) | null = null

const isLight = computed(() => (themeStore.theme ? isLightTheme(themeStore.theme) : false))

onMounted(async () => {
  isMaximized.value = await appWindow.isMaximized().catch(() => false)
  unlistenResize = await appWindow.onResized(async () => {
    isMaximized.value = await appWindow.isMaximized().catch(() => false)
  })
})

onUnmounted(() => {
  unlistenResize?.()
})

async function minimize() {
  await appWindow.minimize()
}

async function toggleMaximize() {
  await appWindow.toggleMaximize()
  isMaximized.value = await appWindow.isMaximized().catch(() => false)
}

async function close() {
  await appWindow.close()
}

async function toggleTheme() {
  const target = isLight.value ? 'originium' : 'light'
  try {
    await themeStore.applyPreset(target)
  } catch (error) {
    app.notify(String(error), 'error')
  }
}
</script>

<style scoped>
.titlebar {
  height: 48px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  gap: 12px;
  padding-left: 14px;
  padding-right: 8px;
  background: var(--originium-titlebar-bg);
  backdrop-filter: blur(24px) saturate(140%);
  border-bottom: 1px solid var(--originium-border);
  color: var(--originium-text);
  user-select: none;
  -webkit-app-region: drag;
  app-region: drag;
  position: relative;
  z-index: 10;
}

.no-drag {
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.titlebar-brand {
  display: flex;
  align-items: center;
  gap: 9px;
  cursor: pointer;
  flex-shrink: 0;
}

.brand-mark {
  width: 28px;
  height: 28px;
  display: grid;
  place-items: center;
  border-radius: 9px;
  background: linear-gradient(140deg, var(--originium-primary), var(--originium-secondary));
  color: #0b0b0f;
  font-size: 0.82rem;
  box-shadow: 0 6px 20px color-mix(in srgb, var(--originium-primary) 40%, transparent);
}

.brand-copy {
  line-height: 1.1;
}

.brand-name {
  font-size: 0.8rem;
  font-weight: 700;
  letter-spacing: 0.02em;
  white-space: nowrap;
}

.brand-version {
  margin-top: 2px;
  font-size: 0.6rem;
  color: var(--originium-text-faint);
}

.titlebar-divider {
  width: 1px;
  height: 18px;
  background: var(--originium-border);
  flex-shrink: 0;
}

.titlebar-heading {
  min-width: 0;
  line-height: 1.15;
}

.titlebar-title {
  font-size: 0.82rem;
  font-weight: 600;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.titlebar-sub {
  margin-top: 2px;
  font-size: 0.65rem;
  color: var(--originium-text-faint);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.session-chip {
  flex-shrink: 0;
}

.titlebar-actions {
  display: flex;
  align-items: center;
  gap: 0;
  flex-shrink: 0;
}

.window-controls {
  display: flex;
  align-items: center;
  gap: 2px;
  margin-left: 4px;
  height: 100%;
  flex-shrink: 0;
}

.window-button {
  width: 38px;
  height: 32px;
  display: grid;
  place-items: center;
  border: 0;
  border-radius: 8px;
  background: transparent;
  color: var(--originium-text-muted);
  cursor: pointer;
  transition: background 140ms ease, color 140ms ease;
}

.window-button:hover {
  background: var(--originium-hover);
  color: var(--originium-text);
}

.window-button.close:hover {
  background: #e5484d;
  color: #fff;
}

@media (max-width: 1280px) {
  .session-chip {
    display: none;
  }
}

@media (max-width: 1120px) {
  .titlebar {
    gap: 8px;
    padding-left: 10px;
  }

  .titlebar-sub,
  .titlebar-divider {
    display: none;
  }
}

@media (max-width: 900px) {
  .titlebar-heading {
    display: none;
  }
}

@media (max-width: 720px) {
  .brand-copy {
    display: none;
  }

  .window-button {
    width: 32px;
  }
}
</style>
