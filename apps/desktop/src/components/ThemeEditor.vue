<template>
  <div class="d-flex flex-column ga-5">
    <div>
      <div class="section-label mb-2">{{ t('theme.presets') }}</div>
      <div class="d-flex ga-3 flex-wrap">
        <v-card
          v-for="preset in themeStore.presets"
          :key="preset.id"
          class="glass preset-card"
          :class="{ 'preset-active': model.preset === preset.id }"
          elevation="0"
          @click="applyPreset(preset)"
        >
          <div class="preset-swatch" :style="{ background: preset.theme.colors.primary }" />
          <div class="pa-3">
            <div class="text-body-2 font-weight-medium">{{ preset.name }}</div>
            <div class="d-flex ga-1 mt-2">
              <span
                v-for="color in preset.theme.colors"
                :key="color"
                class="dot"
                :style="{ background: color }"
              />
            </div>
          </div>
        </v-card>
      </div>
    </div>

    <div>
      <div class="section-label mb-2">{{ t('theme.colors') }}</div>
      <v-card class="glass pa-4" elevation="0">
        <v-row dense>
          <v-col v-for="color in colorFields" :key="color.key" cols="12" sm="6" md="3">
            <div class="d-flex align-center ga-3">
              <input
                type="color"
                class="color-input"
                :value="model.colors[color.key]"
                @input="setColor(color.key, $event)"
              />
              <v-text-field
                :model-value="model.colors[color.key]"
                :label="color.label"
                hide-details
                density="compact"
                class="mono"
                @update:model-value="setColorValue(color.key, String($event))"
              />
            </div>
          </v-col>
        </v-row>
      </v-card>
    </div>

    <div>
      <div class="section-label mb-2">{{ t('theme.typography') }}</div>
      <v-card class="glass pa-4" elevation="0">
        <v-row dense>
          <v-col cols="12" md="8">
            <v-text-field v-model="model.typography.fontFamily" :label="t('theme.fontFamily')" hide-details />
          </v-col>
          <v-col cols="12" md="4">
            <v-btn block variant="tonal" prepend-icon="mdi-format-font" @click="importFont">
              {{ t('theme.importFont') }}
            </v-btn>
          </v-col>
          <v-col cols="12">
            <div class="d-flex ga-2 flex-wrap mt-2">
              <v-chip
                v-for="font in themeStore.assets.fonts"
                :key="font"
                size="small"
                variant="tonal"
                @click="useFont(font)"
              >
                <v-icon start size="14">mdi-format-font</v-icon>
                {{ font }}
              </v-chip>
            </div>
          </v-col>
          <v-col cols="12" md="6">
            <v-slider
              v-model="model.typography.scale"
              :label="t('theme.fontScale')"
              :min="0.8"
              :max="1.4"
              :step="0.05"
              thumb-label
            />
          </v-col>
        </v-row>
      </v-card>
    </div>

    <div>
      <div class="section-label mb-2">{{ t('theme.background') }}</div>
      <v-card class="glass pa-4" elevation="0">
        <v-row dense>
          <v-col cols="12" md="6">
            <v-select
              :model-value="model.background.image ?? ''"
              :items="backgroundItems"
              :label="t('theme.background')"
              hide-details
              clearable
              @update:model-value="setBackground"
            />
          </v-col>
          <v-col cols="12" md="6">
            <v-btn block variant="tonal" prepend-icon="mdi-image-plus" @click="importBackground">
              {{ t('theme.importBackground') }}
            </v-btn>
          </v-col>
          <v-col cols="12" md="4">
            <v-select v-model="model.background.mode" :items="backgroundModes" :label="t('theme.backgroundMode')" hide-details />
          </v-col>
          <v-col cols="12" md="4">
            <v-slider
              v-model="model.background.opacity"
              :label="t('theme.opacity')"
              :min="0"
              :max="1"
              :step="0.05"
              thumb-label
            />
          </v-col>
          <v-col cols="12" md="4">
            <v-slider v-model="model.background.blur" :label="t('theme.blur')" :min="0" :max="64" :step="1" thumb-label />
          </v-col>
        </v-row>
      </v-card>
    </div>

    <div>
      <div class="section-label mb-2">{{ t('theme.effects') }}</div>
      <v-card class="glass pa-4" elevation="0">
        <v-row dense align="center">
          <v-col cols="12" md="4">
            <v-slider v-model="model.effects.rounded" :label="t('theme.rounded')" :min="0" :max="32" :step="1" thumb-label />
          </v-col>
          <v-col cols="6" md="2">
            <v-switch v-model="model.effects.shadows" :label="t('theme.shadows')" hide-details />
          </v-col>
          <v-col cols="6" md="2">
            <v-switch v-model="model.effects.motion" :label="t('theme.motion')" hide-details />
          </v-col>
          <v-col cols="12" md="4">
            <v-text-field v-model="model.name" :label="t('theme.title')" hide-details />
          </v-col>
        </v-row>
      </v-card>
    </div>

    <div class="d-flex ga-3 flex-wrap">
      <v-btn color="primary" prepend-icon="mdi-content-save" @click="$emit('save', model)">
        {{ t('common.save') }}
      </v-btn>
      <v-btn variant="tonal" prepend-icon="mdi-download" @click="exportTheme">
        {{ t('theme.exportTheme') }}
      </v-btn>
      <v-btn variant="tonal" prepend-icon="mdi-upload" @click="importTheme">
        {{ t('theme.importTheme') }}
      </v-btn>
      <v-spacer />
      <v-chip variant="tonal">{{ model.name }}</v-chip>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { open } from '@tauri-apps/plugin-dialog'
import type { ThemeColors, ThemeConfig, ThemePreset } from '@/api/types'
import { useThemeStore } from '@/stores/theme'
import { useAppStore } from '@/stores/app'
import * as api from '@/api'

const model = defineModel<ThemeConfig>({ required: true })
defineEmits<{ save: [theme: ThemeConfig] }>()

const { t } = useI18n()
const themeStore = useThemeStore()
const app = useAppStore()

const colorFields: { key: keyof ThemeColors; label: string }[] = [
  { key: 'primary', label: 'Primary' },
  { key: 'secondary', label: 'Secondary' },
  { key: 'surface', label: 'Surface' },
  { key: 'background', label: 'Background' },
  { key: 'error', label: 'Error' },
  { key: 'success', label: 'Success' },
  { key: 'info', label: 'Info' },
  { key: 'warning', label: 'Warning' },
]

const backgroundModes = [
  { title: 'Cover', value: 'cover' },
  { title: 'Contain', value: 'contain' },
  { title: 'Tile', value: 'tile' },
  { title: 'Center', value: 'center' },
]

const backgroundItems = computed(() => [
  { title: t('common.none'), value: '' },
  ...themeStore.assets.backgrounds.map((name) => ({ title: name, value: `backgrounds/${name}` })),
])

let previewTimer: number | null = null
watch(
  model,
  (value) => {
    if (previewTimer !== null) window.clearTimeout(previewTimer)
    previewTimer = window.setTimeout(() => {
      void themeStore.apply({ ...value })
    }, 200)
  },
  { deep: true },
)

function setColor(key: keyof ThemeColors, event: Event) {
  setColorValue(key, (event.target as HTMLInputElement).value)
}

function setColorValue(key: keyof ThemeColors, value: string) {
  model.value.colors[key] = value
}

function applyPreset(preset: ThemePreset) {
  model.value = {
    ...preset.theme,
    background: { ...preset.theme.background, image: model.value.background.image ?? null },
    name: preset.theme.name,
  }
}

function useFont(file: string) {
  const family = file.replace(/\.[^.]+$/, '')
  if (!model.value.typography.fontFamily.includes(family)) {
    model.value.typography.fontFamily = `'${family}', ${model.value.typography.fontFamily}`
  }
}

async function importFont() {
  const path = await open({
    multiple: false,
    filters: [{ name: 'Fonts', extensions: ['woff2', 'woff', 'ttf', 'otf'] }],
  })
  if (typeof path !== 'string') return
  try {
    await themeStore.importFont(path)
    useFont(path.split(/[\\/]/).pop() ?? 'Custom')
    app.notify(t('theme.applied'), 'success')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

async function importBackground() {
  const path = await open({
    multiple: false,
    filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'webp', 'gif'] }],
  })
  if (typeof path !== 'string') return
  try {
    const relative = await themeStore.importBackground(path)
    model.value.background.image = relative
    app.notify(t('theme.applied'), 'success')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}

function setBackground(value: string | null) {
  model.value.background.image = value || null
}

async function exportTheme() {
  const json = await themeStore.exportTheme()
  const blob = new Blob([json], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const anchor = document.createElement('a')
  anchor.href = url
  anchor.download = `${model.value.name || 'originium-theme'}.json`
  anchor.click()
  URL.revokeObjectURL(url)
}

async function importTheme() {
  try {
    const text = await navigator.clipboard.readText()
    const theme = await themeStore.importTheme(text)
    model.value = theme
    app.notify(t('theme.applied'), 'success')
  } catch (error) {
    app.notify(api.errorMessage(error), 'error')
  }
}
</script>

<style scoped>
.preset-card {
  width: 190px;
  cursor: pointer;
  border-radius: 16px;
  overflow: hidden;
  transition: transform 160ms ease;
}
.preset-card:hover {
  transform: translateY(-2px);
}
.preset-active {
  border-color: var(--originium-primary) !important;
  box-shadow: 0 0 0 1px var(--originium-primary);
}
.preset-swatch {
  height: 54px;
}
.dot {
  width: 14px;
  height: 14px;
  border-radius: 50%;
  display: inline-block;
}
.color-input {
  width: 44px;
  height: 44px;
  border-radius: 10px;
  border: 1px solid rgba(255, 255, 255, 0.18);
  background: transparent;
  padding: 2px;
  cursor: pointer;
  flex-shrink: 0;
}
</style>
