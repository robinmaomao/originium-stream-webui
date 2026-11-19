<template>
  <div class="originium-bg" />
  <div class="originium-shell">
    <v-navigation-drawer
      v-model="rail"
      :rail="rail"
      permanent
      class="originium-nav"
      :width="252"
      :rail-width="78"
    >
      <div class="brand" @click="rail = !rail">
        <div class="brand-mark">◆</div>
        <div v-if="!rail" class="brand-text">
          <div class="brand-name">Originium <span class="text-gradient">Stream</span></div>
          <div class="brand-version mono">v{{ app.appVersion }}</div>
        </div>
      </div>

      <v-divider class="my-2" opacity="0.08" />

      <v-list nav density="comfortable" class="px-2">
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

      <template #append>
        <div class="pa-3">
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
            :prepend-icon="rail ? undefined : 'mdi-weather-night'"
            @click="toggleQuickTheme"
          >
            <v-icon v-if="rail">mdi-weather-night</v-icon>
            <span v-else>{{ themeStore.theme?.name }}</span>
          </v-btn>
        </div>
      </template>
    </v-navigation-drawer>

    <div class="originium-main scroll-area">
      <div class="app-bar">
        <div class="min-width-0">
          <div class="app-bar-title">{{ pageTitle }}</div>
          <div class="app-bar-sub">{{ pageSubtitle }}</div>
        </div>
        <v-spacer />
        <v-chip
          v-if="session.active"
          color="primary"
          variant="flat"
          class="mr-2"
          @click="router.push('/session')"
        >
          <span class="status-dot online" />
          {{ t(`session.${session.stateLabel}`) }}
          <span v-if="session.snapshot?.appName" class="ml-2 opacity-80">
            {{ session.snapshot.appName }}
          </span>
        </v-chip>
        <v-btn icon variant="text" :title="t('session.toggleHud')" @click="toggleHud">
          <v-icon>mdi-monitor-eye</v-icon>
        </v-btn>
        <v-btn icon variant="text" to="/logs" :title="t('nav.logs')">
          <v-icon>mdi-text-box-outline</v-icon>
        </v-btn>
        <v-btn icon variant="text" to="/settings" :title="t('nav.settings')">
          <v-icon>mdi-cog-outline</v-icon>
        </v-btn>
      </div>

      <div class="page">
        <slot />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useTheme } from 'vuetify'
import { useAppStore } from '@/stores/app'
import { useSessionStore } from '@/stores/session'
import { useThemeStore } from '@/stores/theme'
import { toggleHud } from '@/api'
import { isLightTheme } from '@/utils/theme'

const app = useAppStore()
const session = useSessionStore()
const themeStore = useThemeStore()
const route = useRoute()
const router = useRouter()
const { t } = useI18n()

const rail = ref(false)

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
    vuetifyTheme.themes.value.originium = {
      dark,
      colors: {
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
        'on-primary': '#0B0B0F',
        'on-secondary': '#0B0B0F',
        'on-error': '#FFFFFF',
        'on-info': '#0B0B0F',
        'on-success': '#0B0B0F',
        'on-warning': '#0B0B0F',
      },
      variables: {},
    }
    vuetifyTheme.global.name.value = 'originium'
  },
  { immediate: true, deep: true },
)

async function toggleQuickTheme() {
  const target = themeStore.theme?.preset === 'light' ? 'originium' : 'light'
  try {
    await themeStore.applyPreset(target)
  } catch (error) {
    app.notify(String(error), 'error')
  }
}
</script>

<style scoped>
.brand {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 18px 16px 10px;
  cursor: pointer;
}
.brand-mark {
  width: 38px;
  height: 38px;
  display: grid;
  place-items: center;
  border-radius: 12px;
  background: linear-gradient(140deg, var(--originium-primary), var(--originium-secondary));
  color: #0b0b0f;
  font-size: 1.1rem;
  box-shadow: 0 8px 28px rgba(124, 77, 255, 0.45);
  flex-shrink: 0;
}
.brand-name {
  font-weight: 700;
  font-size: 0.95rem;
  letter-spacing: 0.02em;
}
.brand-version {
  font-size: 0.68rem;
  color: rgba(232, 232, 240, 0.45);
}
.nav-item {
  margin-bottom: 4px;
  color: rgba(232, 232, 240, 0.72);
}
.nav-item-active {
  background: rgba(124, 77, 255, 0.18) !important;
  color: #fff !important;
}
.app-bar {
  position: sticky;
  top: 0;
  z-index: 5;
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 20px 32px 14px;
  background: linear-gradient(180deg, rgba(10, 10, 16, 0.92), rgba(10, 10, 16, 0.65), transparent);
  backdrop-filter: blur(8px);
}
.app-bar-title {
  font-size: 1.05rem;
  font-weight: 700;
  letter-spacing: 0.02em;
}
.app-bar-sub {
  font-size: 0.75rem;
  color: rgba(232, 232, 240, 0.5);
}
.session-mini {
  border-radius: 14px;
}
.min-width-0 {
  min-width: 0;
}
</style>
