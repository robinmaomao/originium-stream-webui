<template>
  <div class="onboarding-shell scroll-area">
    <div class="onboarding-card glass">
      <div class="d-flex align-center ga-4 mb-2">
        <div class="logo-mark">◆</div>
        <div>
          <div class="page-title">{{ t('onboarding.title') }}</div>
          <div class="page-subtitle">{{ t('onboarding.subtitle') }}</div>
        </div>
        <v-spacer />
        <v-chip variant="tonal" size="small">v{{ app.appVersion }}</v-chip>
      </div>

      <div class="stepper mb-6">
        <div
          v-for="(label, index) in steps"
          :key="label"
          class="step"
          :class="{ active: step === index, done: step > index }"
        >
          <div class="step-index">{{ index + 1 }}</div>
          <span>{{ t(label) }}</span>
        </div>
      </div>

      <div class="step-body">
        <div v-if="step === 0">
          <div class="text-center py-8">
            <div class="hero-mark">◆</div>
            <div class="text-h5 font-weight-bold mt-6">{{ t('app.name') }}</div>
            <div class="muted mt-2">{{ t('app.tagline') }}</div>
            <div class="d-flex justify-center ga-4 mt-8 flex-wrap">
              <v-chip variant="tonal" prepend-icon="mdi-desktop-tower">
                Sunshine {{ environment?.sunshineInstalled ? '✓' : '—' }}
              </v-chip>
              <v-chip variant="tonal" prepend-icon="mdi-play-circle-outline">
                Moonlight {{ environment?.moonlightInstalled ? '✓' : '—' }}
              </v-chip>
              <v-chip variant="tonal" prepend-icon="mdi-microsoft-windows">
                WebView2 {{ environment?.webview2Version ?? '—' }}
              </v-chip>
            </div>
          </div>
        </div>

        <div v-if="step === 1">
          <p class="text-body-1 mb-4">{{ t('onboarding.hostIntro') }}</p>
          <v-card class="glass pa-4 mb-4" elevation="0">
            <div class="d-flex align-center ga-3 mb-3">
              <v-icon color="primary" icon="mdi-weather-sunny" />
              <div class="font-weight-medium">Sunshine</div>
              <v-spacer />
              <v-chip :color="environment?.sunshineInstalled ? 'success' : 'warning'" variant="tonal" size="small">
                {{ environment?.sunshineInstalled ? t('settings.installed') : t('settings.notInstalled') }}
              </v-chip>
              <v-btn
                v-if="!environment?.sunshineInstalled"
                size="small"
                color="primary"
                :loading="installing === 'sunshine'"
                @click="install('sunshine')"
              >
                {{ t('common.install') }}
              </v-btn>
            </div>
            <v-progress-linear
              v-if="progress && progress.id === 'sunshine'"
              :model-value="progress.percent"
              color="primary"
              height="6"
              rounded
              class="mb-3"
            />
            <v-row dense>
              <v-col cols="12" md="6">
                <v-text-field v-model="credentials.username" :label="t('local.username')" autocomplete="username" />
              </v-col>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="credentials.password"
                  type="password"
                  :label="t('local.password')"
                  autocomplete="new-password"
                />
              </v-col>
            </v-row>
            <div class="d-flex ga-2 flex-wrap mt-2">
              <v-btn variant="tonal" prepend-icon="mdi-account-key" :loading="savingCredentials" @click="saveCredentials">
                {{ t('local.credentials') }}
              </v-btn>
              <v-btn variant="tonal" prepend-icon="mdi-shield-lock" :loading="installing === 'service'" @click="installService">
                {{ t('local.installService') }}
              </v-btn>
              <v-btn variant="tonal" prepend-icon="mdi-firewall" :loading="installing === 'firewall'" @click="addFirewall">
                {{ t('local.addFirewall') }}
              </v-btn>
            </div>
            <div class="text-caption muted mt-2">{{ t('onboarding.serviceHint') }}</div>
          </v-card>
          <v-alert v-if="hostError" type="error" variant="tonal">{{ hostError }}</v-alert>
        </div>

        <div v-if="step === 2">
          <p class="text-body-1 mb-4">{{ t('onboarding.clientIntro') }}</p>
          <v-card class="glass pa-4" elevation="0">
            <div class="d-flex align-center ga-3">
              <v-icon color="primary" icon="mdi-play-circle-outline" />
              <div class="font-weight-medium">Moonlight</div>
              <v-spacer />
              <v-chip :color="environment?.moonlightInstalled ? 'success' : 'warning'" variant="tonal" size="small">
                {{ environment?.moonlightVersion ?? t('settings.notInstalled') }}
              </v-chip>
              <v-btn
                v-if="!environment?.moonlightInstalled"
                size="small"
                color="primary"
                :loading="installing === 'moonlight'"
                @click="install('moonlight')"
              >
                {{ t('common.install') }}
              </v-btn>
            </div>
            <v-progress-linear
              v-if="progress && progress.id === 'moonlight'"
              :model-value="progress.percent"
              color="primary"
              height="6"
              rounded
              class="mt-3"
            />
          </v-card>
        </div>

        <div v-if="step === 3">
          <p class="text-body-1 mb-4">{{ t('onboarding.themeIntro') }}</p>
          <div class="d-flex ga-3 flex-wrap">
            <v-card
              v-for="preset in themeStore.presets"
              :key="preset.id"
              class="glass preset-card"
              :class="{ active: themeStore.theme?.preset === preset.id }"
              elevation="0"
              @click="themeStore.applyPreset(preset.id)"
            >
              <div class="preset-swatch" :style="{ background: preset.theme.colors.primary }" />
              <div class="pa-3 text-body-2">{{ preset.name }}</div>
            </v-card>
          </div>
        </div>

        <div v-if="step === 4">
          <div class="text-center py-8">
            <v-icon size="56" color="success" icon="mdi-check-decagram" />
            <div class="text-h5 font-weight-bold mt-4">{{ t('onboarding.doneTitle') }}</div>
            <div class="muted mt-2">{{ t('onboarding.doneDesc') }}</div>
          </div>
        </div>
      </div>

      <div class="d-flex align-center ga-3 mt-6">
        <v-btn variant="text" :disabled="step === 0" @click="step--">{{ t('common.cancel') }}</v-btn>
        <v-spacer />
        <v-btn v-if="step < 4" color="primary" @click="step++">{{ t('common.confirm') }}</v-btn>
        <v-btn v-else color="primary" :loading="finishing" @click="finish">{{ t('onboarding.finish') }}</v-btn>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import * as api from '@/api'
import type { ComponentProgress } from '@/api/types'
import { useAppStore } from '@/stores/app'
import { useHostsStore } from '@/stores/hosts'
import { useThemeStore } from '@/stores/theme'

const { t } = useI18n()
const app = useAppStore()
const hosts = useHostsStore()
const themeStore = useThemeStore()
const router = useRouter()

const steps = ['onboarding.stepWelcome', 'onboarding.stepHost', 'onboarding.stepClient', 'onboarding.stepTheme', 'onboarding.stepDone']
const step = ref(0)
const installing = ref('')
const progress = ref<ComponentProgress | null>(null)
const savingCredentials = ref(false)
const finishing = ref(false)
const hostError = ref('')
const credentials = reactive({ username: '', password: '' })

const environment = computed(() => app.environment)

onMounted(async () => {
  await app.refreshEnvironment()
})

async function install(id: string) {
  installing.value = id
  progress.value = null
  try {
    await app.installComponent(id, (event) => {
      progress.value = event
    })
    await app.refreshEnvironment()
    app.notify(`${id} ${t('settings.installed')}`, 'success')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  } finally {
    installing.value = ''
    progress.value = null
  }
}

async function saveCredentials() {
  if (!credentials.username.trim() || !credentials.password) {
    hostError.value = '用户名与密码不能为空'
    return
  }
  savingCredentials.value = true
  hostError.value = ''
  try {
    await api.setSunshineCredentials(credentials.username.trim(), credentials.password)
    const existing = hosts.hosts.find(
      (host) => host.address === '127.0.0.1' || host.address === 'localhost',
    )
    if (!existing) {
      await hosts.add({
        name: environment.value?.hostname ?? 'This PC',
        address: '127.0.0.1',
        port: 47990,
        username: credentials.username.trim(),
        password: credentials.password,
        source: 'manual',
      })
    } else {
      await hosts.update({
        id: existing.id,
        name: existing.name,
        address: existing.address,
        port: existing.port,
        username: credentials.username.trim(),
        password: credentials.password,
        source: existing.source,
      })
    }
    app.notify(t('onboarding.hostAdded'), 'success')
  } catch (error) {
    hostError.value = api.errorMessage(error)
  } finally {
    savingCredentials.value = false
  }
}

async function installService() {
  installing.value = 'service'
  try {
    await api.installSunshineService()
    await api.startSunshineService()
    await app.refreshEnvironment()
    app.notify(t('local.installService'), 'success')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  } finally {
    installing.value = ''
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

async function finish() {
  finishing.value = true
  try {
    await app.completeOnboarding()
    await router.replace('/local')
  } finally {
    finishing.value = false
  }
}
</script>

<style scoped>
.onboarding-shell {
  height: calc(100vh - 48px);
  display: flex;
  justify-content: center;
  align-items: flex-start;
  padding: clamp(14px, 3vh, 24px);
  background:
    radial-gradient(circle at 10% 0%, rgba(124, 77, 255, 0.25), transparent 45%),
    radial-gradient(circle at 90% 10%, rgba(0, 229, 255, 0.16), transparent 40%),
    #08080c;
}
.onboarding-card {
  width: min(980px, 100%);
  margin-block: auto;
  border-radius: 28px;
  padding: clamp(20px, 3vw, 32px);
}
.logo-mark {
  width: 46px;
  height: 46px;
  border-radius: 14px;
  display: grid;
  place-items: center;
  background: linear-gradient(140deg, var(--originium-primary), var(--originium-secondary));
  color: #0b0b0f;
  font-size: 1.3rem;
}
.stepper {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}
.step {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 14px;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.04);
  color: rgba(232, 232, 240, 0.55);
  font-size: 0.78rem;
}
.step.active {
  background: rgba(124, 77, 255, 0.2);
  color: #fff;
}
.step.done {
  color: var(--originium-success);
}
.step-index {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  display: grid;
  place-items: center;
  font-size: 0.7rem;
  background: rgba(255, 255, 255, 0.1);
}
.hero-mark {
  font-size: 3.6rem;
  background: linear-gradient(140deg, var(--originium-primary), var(--originium-secondary));
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
}
.preset-card {
  width: 190px;
  cursor: pointer;
  border-radius: 16px;
  overflow: hidden;
}
.preset-card.active {
  border-color: var(--originium-primary) !important;
}
.preset-swatch {
  height: 56px;
}
</style>
