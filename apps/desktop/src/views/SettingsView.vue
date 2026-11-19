<template>
  <div>
    <div class="d-flex ga-3 align-center mb-5 flex-wrap">
      <div>
        <div class="page-title">{{ t('settings.title') }}</div>
        <div class="page-subtitle">{{ t('settings.subtitle') }}</div>
      </div>
      <v-spacer />
      <v-btn variant="tonal" prepend-icon="mdi-radar" :loading="detecting" @click="detect">
        {{ t('common.refresh') }}
      </v-btn>
    </div>

    <v-tabs v-model="tab" class="mb-4">
      <v-tab value="components" prepend-icon="mdi-package-variant">{{ t('settings.components') }}</v-tab>
      <v-tab value="app" prepend-icon="mdi-cog-outline">{{ t('settings.app') }}</v-tab>
      <v-tab value="theme" prepend-icon="mdi-palette">{{ t('settings.theme') }}</v-tab>
      <v-tab value="moonlight" prepend-icon="mdi-play-circle-outline">{{ t('settings.moonlight') }}</v-tab>
      <v-tab value="advanced" prepend-icon="mdi-wrench">{{ t('settings.advanced') }}</v-tab>
    </v-tabs>

    <!-- Components -->
    <div v-show="tab === 'components'">
      <v-alert type="info" variant="tonal" density="comfortable" class="mb-4">
        {{ t('settings.componentHint') }}
      </v-alert>
      <v-row>
        <v-col v-for="spec in manifest" :key="spec.id" cols="12" lg="6">
          <v-card class="glass pa-5 h-100" elevation="0">
            <div class="d-flex align-center ga-3 mb-3">
              <v-icon size="30" :color="spec.id === 'sunshine' ? 'warning' : 'primary'">
                {{ spec.id === 'sunshine' ? 'mdi-weather-sunny' : 'mdi-play-circle-outline' }}
              </v-icon>
              <div>
                <div class="text-h6">{{ spec.id }}</div>
                <div class="text-caption muted mono">{{ spec.variant }} · {{ spec.version }}</div>
              </div>
              <v-spacer />
              <v-chip
                :color="isInstalled(spec.id) ? 'success' : 'default'"
                variant="tonal"
                size="small"
              >
                {{ isInstalled(spec.id) ? t('settings.installed') : t('settings.notInstalled') }}
              </v-chip>
            </div>

            <div v-if="spec.capabilities.length" class="d-flex ga-2 flex-wrap mb-3">
              <v-chip v-for="capability in spec.capabilities" :key="capability" size="x-small" variant="outlined">
                {{ capability }}
              </v-chip>
            </div>

            <v-progress-linear
              v-if="progress && progress.id === spec.id"
              :model-value="progress.percent"
              color="primary"
              height="6"
              rounded
              class="mb-2"
            />
            <div v-if="progress && progress.id === spec.id" class="text-caption muted mb-2">
              {{ progress.message }}
            </div>

            <div class="d-flex ga-2 flex-wrap mb-3">
              <v-btn
                v-if="!isInstalled(spec.id)"
                color="primary"
                prepend-icon="mdi-download"
                :loading="installing === spec.id"
                @click="install(spec.id)"
              >
                {{ t('common.install') }}
              </v-btn>
              <v-btn
                v-else
                variant="tonal"
                prepend-icon="mdi-refresh"
                :loading="installing === spec.id"
                @click="install(spec.id)"
              >
                {{ t('common.refresh') }}
              </v-btn>
              <v-btn
                v-if="isInstalled(spec.id)"
                variant="text"
                color="error"
                prepend-icon="mdi-delete-outline"
                @click="uninstall(spec.id)"
              >
                {{ t('common.uninstall') }}
              </v-btn>
            </div>

            <v-text-field
              :model-value="spec.id === 'sunshine' ? form.sunshinePath ?? '' : form.moonlightPath ?? ''"
              :label="t('settings.path')"
              hide-details
              density="compact"
              class="mono"
              append-inner-icon="mdi-folder-search"
              @click:append-inner="browsePath(spec.id)"
              @update:model-value="setPathDraft(spec.id, String($event))"
            />
          </v-card>
        </v-col>
      </v-row>
    </div>

    <!-- App settings -->
    <div v-show="tab === 'app'">
      <v-row>
        <v-col cols="12" lg="5">
          <v-card class="glass pa-5" elevation="0">
            <div class="text-h6 mb-4">{{ t('settings.app') }}</div>
            <v-select v-model="form.language" :items="localeOptions" :label="t('settings.language')" />
            <v-switch v-model="form.autostart" :label="t('settings.autostart')" hide-details />
            <v-switch v-model="form.startMinimized" :label="t('settings.startMinimized')" hide-details />
            <v-switch v-model="form.closeToTray" :label="t('settings.closeToTray')" hide-details />
            <v-switch v-model="form.preferServiceMode" :label="t('settings.serviceMode')" hide-details />
            <v-text-field v-model="form.globalHotkey" :label="t('settings.hotkey')" class="mt-3 mono" />
            <v-select
              v-model="form.updateChannel"
              :items="[
                { title: t('settings.stable'), value: 'stable' },
                { title: t('settings.beta'), value: 'beta' },
              ]"
              :label="t('settings.updateChannel')"
            />
            <v-btn class="mt-3" color="primary" block prepend-icon="mdi-content-save" :loading="saving" @click="saveSettings">
              {{ t('common.save') }}
            </v-btn>
          </v-card>
        </v-col>
        <v-col cols="12" lg="7">
          <v-card class="glass pa-5" elevation="0">
            <div class="text-h6 mb-4">{{ t('settings.defaultProfile') }}</div>
            <StreamProfileForm v-model="form.defaultProfile" />
            <v-btn class="mt-4" color="primary" prepend-icon="mdi-content-save" :loading="saving" @click="saveSettings">
              {{ t('common.save') }}
            </v-btn>
          </v-card>
        </v-col>
      </v-row>
    </div>

    <!-- Theme -->
    <div v-show="tab === 'theme'">
      <ThemeEditor v-model="themeModel" @save="saveTheme" />
    </div>

    <!-- Moonlight -->
    <div v-show="tab === 'moonlight'">
      <v-row>
        <v-col cols="12" lg="7">
          <v-card class="glass pa-5" elevation="0">
            <div class="d-flex align-center mb-3">
              <div class="text-h6">{{ t('settings.rawIni') }}</div>
              <v-spacer />
              <v-chip size="small" variant="tonal" class="mono">
                {{ moonlight?.exists ? t('settings.installed') : t('settings.notInstalled') }}
              </v-chip>
            </div>
            <div class="text-caption muted mb-3 mono text-truncate">{{ moonlight?.path ?? '—' }}</div>
            <p class="text-body-2 muted mb-3">{{ t('settings.iniHint') }}</p>
            <v-textarea v-model="iniText" rows="18" class="mono" hide-details />
            <v-alert v-if="iniError" type="error" variant="tonal" class="mt-3">{{ iniError }}</v-alert>
            <div class="d-flex ga-2 mt-3">
              <v-btn color="primary" prepend-icon="mdi-content-save" :loading="savingIni" @click="saveIni">
                {{ t('common.save') }}
              </v-btn>
              <v-btn variant="tonal" prepend-icon="mdi-refresh" @click="loadIni">
                {{ t('common.refresh') }}
              </v-btn>
            </div>
          </v-card>
        </v-col>
        <v-col cols="12" lg="5">
          <v-card class="glass pa-5" elevation="0">
            <div class="text-h6 mb-3">{{ t('settings.defaultProfile') }}</div>
            <StreamProfileForm v-model="form.defaultProfile" />
            <v-btn class="mt-4" color="primary" prepend-icon="mdi-content-save" :loading="saving" @click="saveSettings">
              {{ t('common.save') }}
            </v-btn>
          </v-card>
        </v-col>
      </v-row>
    </div>

    <!-- Advanced -->
    <div v-show="tab === 'advanced'">
      <v-row>
        <v-col cols="12" lg="6">
          <v-card class="glass pa-5 h-100" elevation="0">
            <div class="text-h6 mb-3">{{ t('settings.diagnostics') }}</div>
            <p class="text-body-2 muted">{{ t('settings.diagnosticsHint') }}</p>
            <div class="d-flex ga-2 flex-wrap">
              <v-btn color="primary" prepend-icon="mdi-download" @click="exportDiagnostics">
                {{ t('settings.diagnostics') }}
              </v-btn>
              <v-btn variant="tonal" prepend-icon="mdi-folder-open" @click="openLogDir">
                {{ t('settings.openLogDir') }}
              </v-btn>
            </div>
            <div class="mt-4">
              <div class="section-label">{{ t('settings.logPath') }}</div>
              <div class="mono text-caption">{{ logPath ?? '—' }}</div>
            </div>
          </v-card>
        </v-col>
        <v-col cols="12" lg="6">
          <v-card class="glass pa-5 h-100" elevation="0">
            <div class="text-h6 mb-3">{{ t('settings.about') }}</div>
            <div class="info-row">
              <span class="muted">{{ t('local.version') }}</span>
              <span class="mono">{{ app.appVersion }}</span>
            </div>
            <div class="info-row">
              <span class="muted">{{ t('settings.license') }}</span>
              <span>GPL-3.0-or-later</span>
            </div>
            <div class="info-row">
              <span class="muted">Data</span>
              <span class="mono text-caption">{{ environment?.dataDir }}</span>
            </div>
            <div class="d-flex ga-2 flex-wrap mt-4">
              <v-btn variant="tonal" prepend-icon="mdi-github" @click="openUrl('https://github.com')">
                GitHub
              </v-btn>
              <v-btn variant="text" color="error" prepend-icon="mdi-power" @click="quit">
                {{ t('common.close') }}
              </v-btn>
            </div>
          </v-card>
        </v-col>
      </v-row>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { open } from '@tauri-apps/plugin-dialog'
import { openPath, openUrl as openExternal } from '@tauri-apps/plugin-opener'
import * as api from '@/api'
import type {
  AppSettings,
  ComponentProgress,
  ComponentSpec,
  MoonlightSettings,
  ThemeConfig,
} from '@/api/types'
import StreamProfileForm from '@/components/StreamProfileForm.vue'
import ThemeEditor from '@/components/ThemeEditor.vue'
import { useAppStore } from '@/stores/app'
import { useThemeStore } from '@/stores/theme'
import { availableLocales, i18n } from '@/locales'

const { t } = useI18n()
const app = useAppStore()
const themeStore = useThemeStore()

const tab = ref('components')
const manifest = ref<ComponentSpec[]>([])
const progress = ref<ComponentProgress | null>(null)
const installing = ref('')
const detecting = ref(false)
const saving = ref(false)
const savingIni = ref(false)
const iniError = ref('')
const moonlight = ref<MoonlightSettings | null>(null)
const iniText = ref('')
const logPath = ref<string | null>(null)
const autostart = ref(false)

const localeOptions = availableLocales

const environment = computed(() => app.environment)
const installedIds = computed(() => new Set(app.components.map((component) => component.id)))

const form = reactive<AppSettings>(defaultSettings())
const themeModel = ref<ThemeConfig>(defaultTheme())

watch(
  () => app.settings,
  (value) => {
    if (value) Object.assign(form, JSON.parse(JSON.stringify(value)) as AppSettings)
  },
  { immediate: true, deep: true },
)

watch(
  () => app.config?.theme,
  (value) => {
    if (value) themeModel.value = JSON.parse(JSON.stringify(value)) as ThemeConfig
  },
  { immediate: true, deep: true },
)

onMounted(async () => {
  await loadAll()
})

async function loadAll() {
  try {
    manifest.value = await api.getComponentManifest()
    moonlight.value = await api.getMoonlightSettings()
    iniText.value = JSON.stringify(moonlight.value.sections, null, 2)
    logPath.value = await api.getLogPath()
    autostart.value = await api.getAutostart()
    form.autostart = autostart.value
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

function isInstalled(id: string) {
  return installedIds.value.has(id)
}

async function install(id: string) {
  installing.value = id
  progress.value = null
  try {
    await app.installComponent(id, (event) => {
      progress.value = event
    })
    await app.refreshEnvironment()
    app.notify(`${id} ✓`, 'success')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  } finally {
    installing.value = ''
    progress.value = null
  }
}

async function uninstall(id: string) {
  try {
    await app.removeComponent(id)
    await app.refreshEnvironment()
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

async function browsePath(id: string) {
  const path = await open({
    multiple: false,
    filters: [{ name: 'Executable', extensions: ['exe'] }],
  })
  if (typeof path !== 'string') return
  setPathDraft(id, path)
  try {
    await api.setComponentPath(id, path)
    if (id === 'sunshine') form.sunshinePath = path
    if (id === 'moonlight') form.moonlightPath = path
    await app.refreshEnvironment()
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

function setPathDraft(id: string, value: string) {
  if (id === 'sunshine') form.sunshinePath = value || null
  if (id === 'moonlight') form.moonlightPath = value || null
}

async function saveSettings() {
  saving.value = true
  try {
    await app.saveSettings({ ...form, defaultProfile: { ...form.defaultProfile } })
    await api.setAutostart(form.autostart)
    await api.setGlobalHotkey(form.globalHotkey)
    i18n.global.locale.value = form.language as 'zh-CN' | 'en'
    app.notify(t('common.save'), 'success')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  } finally {
    saving.value = false
  }
}

async function saveTheme(theme: ThemeConfig) {
  try {
    const saved = await themeStore.save(theme)
    themeModel.value = saved
    app.notify(t('theme.applied'), 'success')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

async function loadIni() {
  try {
    moonlight.value = await api.getMoonlightSettings()
    iniText.value = JSON.stringify(moonlight.value.sections, null, 2)
    iniError.value = ''
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

async function saveIni() {
  savingIni.value = true
  iniError.value = ''
  try {
    const sections = JSON.parse(iniText.value) as Record<string, Record<string, unknown>>
    moonlight.value = await api.saveMoonlightSettings(sections)
    app.notify(t('common.save'), 'success')
  } catch (error) {
    iniError.value = api.errorMessage(error)
  } finally {
    savingIni.value = false
  }
}

async function detect() {
  detecting.value = true
  try {
    await app.refreshEnvironment()
    await loadAll()
  } finally {
    detecting.value = false
  }
}

async function exportDiagnostics() {
  try {
    const path = await api.exportDiagnostics()
    app.notify(path, 'success')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

async function openLogDir() {
  try {
    const path = logPath.value
    await openPath(path ? path.replace(/[\\/][^\\/]+$/, '') : '.')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

async function openUrl(url: string) {
  try {
    await openExternal(url)
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

async function quit() {
  await api.quitApp()
}

function defaultProfile() {
  return {
    width: 1920,
    height: 1080,
    fps: 60,
    bitrateKbps: 40000,
    videoCodec: 'auto',
    audioConfig: 'stereo',
    displayMode: 'borderless',
    hdr: false,
    yuv444: false,
    vsync: true,
    framePacing: true,
    quitAfter: false,
    keepAwake: true,
    performanceOverlay: false,
    captureSystemKeys: 'always',
    packetSize: null,
    extraArgs: [] as string[],
  }
}

function defaultSettings(): AppSettings {
  return {
    language: 'zh-CN',
    autostart: false,
    startMinimized: false,
    closeToTray: true,
    globalHotkey: 'CommandOrControl+Alt+Shift+O',
    sunshinePath: null,
    moonlightPath: null,
    defaultProfile: defaultProfile(),
    updateChannel: 'stable',
    preferServiceMode: true,
  }
}

function defaultTheme(): ThemeConfig {
  return {
    version: 1,
    name: 'Originium',
    preset: 'originium',
    colors: {
      primary: '#7C4DFF',
      secondary: '#00E5FF',
      surface: '#121218',
      background: '#0B0B0F',
      error: '#FF5370',
      success: '#4ADE80',
      info: '#38BDF8',
      warning: '#FBBF24',
    },
    typography: {
      fontFamily: "Inter, 'Noto Sans SC', 'Microsoft YaHei', sans-serif",
      scale: 1,
    },
    background: { mode: 'cover', opacity: 0.35, blur: 12, image: null },
    effects: { rounded: 14, shadows: true, motion: true },
  }
}
</script>

<style scoped>
.h-100 {
  height: 100%;
}
.info-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 6px 0;
}
</style>
