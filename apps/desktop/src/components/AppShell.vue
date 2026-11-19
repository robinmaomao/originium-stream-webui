<template>
  <div class="originium-bg" />
  <div class="originium-shell">
    <TitleBar :title="pageTitle" :subtitle="pageSubtitle" />

    <div class="originium-body">
      <aside class="originium-nav" :class="{ 'is-rail': rail }">
        <div class="nav-scroll">
          <v-list nav density="comfortable" class="px-2 pt-2">
            <v-list-item
              v-for="item in navItems"
              :key="item.to"
              :to="item.to"
              :prepend-icon="item.icon"
              :title="rail ? undefined : t(item.title)"
              rounded="lg"
              class="nav-item"
              active-class="nav-item-active"
            />
          </v-list>
        </div>

        <div class="nav-footer">
          <div v-if="!rail" class="session-mini glass pa-3 mb-3">
            <div class="d-flex align-center justify-space-between">
              <span class="section-label">{{ t('session.current') }}</span>
              <span class="status-dot" :class="session.active ? 'online' : 'offline'" />
            </div>
            <div class="text-body-2 mt-1">
              {{ session.active ? session.snapshot?.appName : t('session.noSession') }}
            </div>
            <div v-if="session.active" class="text-caption muted">
              {{ session.snapshot?.hostName }}
            </div>
          </div>
          <v-btn
            block
            variant="tonal"
            color="primary"
            :prepend-icon="rail ? undefined : isLight ? 'mdi-white-balance-sunny' : 'mdi-weather-night'"
            @click="toggleQuickTheme"
          >
            <v-icon v-if="rail" :icon="isLight ? 'mdi-white-balance-sunny' : 'mdi-weather-night'" />
            <span v-else>{{ isLight ? t('theme.light') : t('theme.dark') }}</span>
          </v-btn>
        </div>
      </aside>

      <main class="originium-main scroll-area">
        <div class="page">
          <slot />
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useTheme } from 'vuetify'
import { useAppStore } from '@/stores/app'
import { useSessionStore } from '@/stores/session'
import { useThemeStore } from '@/stores/theme'
import { isLightTheme } from '@/utils/theme'
import TitleBar from '@/components/TitleBar.vue'

const app = useAppStore()
const session = useSessionStore()
const themeStore = useThemeStore()
const route = useRoute()
const { t } = useI18n()

const rail = ref(false)
const isLight = computed(() => (themeStore.theme ? isLightTheme(themeStore.theme) : false))

const navItems = [
  { to: '/local', icon: 'mdi-desktop-tower-monitor', title: 'nav.local' },
  { to: '/hosts', icon: 'mdi-lan', title: 'nav.hosts' },
  { to: '/session', icon: 'mdi-play-circle-outline', title: 'nav.session' },
  { to: '/logs', icon: 'mdi-text-box-outline', title: 'nav.logs' },
  { to: '/settings', icon: 'mdi-tune', title: 'nav.settings' },
]

const pageTitle = computed(() => {
  switch (route.name) {
    case 'local':
      return t('local.title')
    case 'hosts':
      return t('hosts.title')
    case 'host-detail':
      return t('hosts.detail')
    case 'session':
      return t('session.title')
    case 'settings':
      return t('settings.title')
    case 'logs':
      return t('logs.title')
    case 'onboarding':
      return t('onboarding.title')
    default:
      return t('app.name')
  }
})

const pageSubtitle = computed(() => {
  switch (route.name) {
    case 'local':
      return t('local.subtitle')
    case 'hosts':
      return t('hosts.subtitle')
    case 'host-detail':
      return t('hosts.subtitle')
    case 'session':
      return t('session.subtitle')
    case 'settings':
      return t('settings.subtitle')
    default:
      return t('app.tagline')
  }
})

const vuetifyTheme = useTheme()
watch(
  () => themeStore.theme,
  (theme) => {
    if (!theme) return
    const dark = !isLightTheme(theme)
    const current = vuetifyTheme.themes.value.originium
    vuetifyTheme.themes.value.originium = {
      ...current,
      dark,
      colors: {
        ...current.colors,
        primary: theme.colors.primary,
        secondary: theme.colors.secondary,
        surface: theme.colors.surface,
        background: theme.colors.background,
        error: theme.colors.error,
        success: theme.colors.success,
        info: theme.colors.info,
        warning: theme.colors.warning,
        'on-surface': dark ? '#E8E8F0' : '#1F2937',
        'on-background': dark ? '#E8E8F0' : '#1F2937',
        'on-primary': dark ? '#0B0B0F' : '#FFFFFF',
        'on-secondary': dark ? '#0B0B0F' : '#FFFFFF',
        'on-error': '#FFFFFF',
        'on-info': dark ? '#0B0B0F' : '#FFFFFF',
        'on-success': dark ? '#0B0B0F' : '#FFFFFF',
        'on-warning': dark ? '#0B0B0F' : '#FFFFFF',
      },
    }
    vuetifyTheme.global.name.value = 'originium'
  },
  { immediate: true, deep: true },
)

async function toggleQuickTheme() {
  const target = isLight.value ? 'originium' : 'light'
  try {
    await themeStore.applyPreset(target)
  } catch (error) {
    app.notify(String(error), 'error')
  }
}
</script>

<style scoped>
.originium-shell {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.originium-body {
  flex: 1;
  min-height: 0;
  display: flex;
}

.originium-nav {
  width: 252px;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background: var(--originium-nav-bg);
  backdrop-filter: blur(20px) saturate(135%);
  border-right: 1px solid var(--originium-border);
  transition: width 180ms ease;
}

.originium-nav.is-rail {
  width: 78px;
}

.nav-scroll {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  overflow-x: hidden;
  padding-top: 10px;
}

.nav-footer {
  padding: 12px;
  border-top: 1px solid var(--originium-border);
}

.nav-item {
  margin-bottom: 4px;
  color: var(--originium-text-muted);
}

.nav-item-active {
  background: color-mix(in srgb, var(--originium-primary) 18%, transparent) !important;
  color: var(--originium-text) !important;
}

.originium-main {
  flex: 1;
  min-width: 0;
  height: 100%;
  overflow-y: auto;
}

.session-mini {
  border-radius: 14px;
}
</style>
