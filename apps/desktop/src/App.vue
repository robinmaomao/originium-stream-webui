<template>
  <v-app v-if="isHud">
    <SessionHud />
  </v-app>
  <v-app v-else class="originium-app">
    <AppShell v-if="app.config && !isOnboarding">
      <router-view />
    </AppShell>
    <router-view v-else-if="app.config" />
    <div v-else class="boot-screen">
      <div class="boot-logo">Originium Stream</div>
      <div class="boot-hint">{{ t('common.loading') }}</div>
    </div>

    <v-snackbar
      v-model="app.toast.show"
      :color="app.toast.color"
      location="bottom right"
      timeout="5000"
      elevation="12"
    >
      <div class="font-weight-medium">{{ app.toast.message }}</div>
      <div v-if="app.toast.hint" class="text-caption opacity-80 mt-1">{{ app.toast.hint }}</div>
    </v-snackbar>
  </v-app>
</template>

<script setup lang="ts">
import { computed, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import AppShell from '@/components/AppShell.vue'
import SessionHud from '@/views/SessionHud.vue'
import { useAppStore } from '@/stores/app'
import { useThemeStore } from '@/stores/theme'
import { useSessionStore } from '@/stores/session'
import { i18n } from '@/locales'

const app = useAppStore()
const theme = useThemeStore()
const session = useSessionStore()
const route = useRoute()
const router = useRouter()
const { t } = useI18n()

const isHud = computed(() => route.path === '/hud')
const isOnboarding = computed(() => route.name === 'onboarding')

onMounted(async () => {
  await app.bootstrap()
  if (!app.config) return
  await theme.apply(app.config.theme)
  i18n.global.locale.value = app.config.settings.language as 'zh-CN' | 'en'
  if (!app.config.onboardingComplete && route.path !== '/onboarding' && route.path !== '/hud') {
    await router.replace('/onboarding')
  } else if (app.config.onboardingComplete && route.path === '/') {
    await router.replace('/local')
  }
  session.startPolling()
})

watch(
  () => app.config?.onboardingComplete,
  (value) => {
    if (value && route.path === '/onboarding') {
      void router.replace('/local')
    }
  },
)

watch(
  () => app.config?.settings.language,
  (value) => {
    if (value) i18n.global.locale.value = value as 'zh-CN' | 'en'
  },
)
</script>

<style scoped>
.boot-screen {
  height: 100vh;
  display: grid;
  place-items: center;
  align-content: center;
  gap: 12px;
  background: var(--originium-background, #0b0b0f);
}
.boot-logo {
  font-size: 1.4rem;
  letter-spacing: 0.4em;
  text-transform: uppercase;
  color: var(--originium-primary, #7c4dff);
}
.boot-hint {
  color: rgba(232, 232, 240, 0.6);
  font-size: 0.8rem;
  letter-spacing: 0.1em;
}
</style>
