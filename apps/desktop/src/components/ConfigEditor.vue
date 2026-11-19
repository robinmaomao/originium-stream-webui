<template>
  <div>
    <div class="d-flex ga-3 align-center mb-4 flex-wrap">
      <v-text-field
        v-model="search"
        :placeholder="t('common.search')"
        prepend-inner-icon="mdi-magnify"
        hide-details
        density="compact"
        style="max-width: 320px"
      />
      <v-spacer />
      <v-btn variant="tonal" prepend-icon="mdi-code-json" @click="rawDialog = true">
        JSON
      </v-btn>
      <v-btn color="primary" prepend-icon="mdi-content-save" :loading="saving" @click="save">
        {{ t('common.save') }}
      </v-btn>
    </div>

    <v-alert v-if="error" type="error" variant="tonal" class="mb-4">{{ error }}</v-alert>

    <div v-for="section in visibleSections" :key="section.id" class="mb-5">
      <div class="section-label mb-2">{{ section.title }}</div>
      <v-card class="glass pa-4" elevation="0">
        <v-row dense>
          <v-col
            v-for="key in section.keys"
            :key="key"
            cols="12"
            :md="fieldSpan(key)"
          >
            <v-switch
              v-if="typeof config[key] === 'boolean'"
              :model-value="Boolean(config[key])"
              :label="key"
              hide-details
              @update:model-value="setValue(key, $event)"
            />
            <v-text-field
              v-else-if="typeof config[key] === 'number'"
              :model-value="Number(config[key])"
              type="number"
              :label="key"
              hide-details
              @update:model-value="setValue(key, Number($event))"
            />
            <v-text-field
              v-else-if="isSimple(config[key])"
              :model-value="String(config[key] ?? '')"
              :label="key"
              hide-details
              @update:model-value="setValue(key, $event)"
            />
            <v-textarea
              v-else
              v-model="complexValues[key]"
              :label="key"
              rows="2"
              auto-grow
              hide-details
              class="mono"
              @blur="applyComplex(key)"
            />
          </v-col>
        </v-row>
      </v-card>
    </div>

    <v-alert v-if="visibleSections.length === 0" type="info" variant="tonal">
      {{ t('common.none') }}
    </v-alert>

    <v-dialog v-model="rawDialog" max-width="900">
      <v-card class="glass pa-2">
        <v-card-title class="d-flex align-center ga-2">
          <v-icon color="primary">mdi-code-json</v-icon>
          JSON
        </v-card-title>
        <v-card-text>
          <v-textarea v-model="rawText" rows="20" class="mono" hide-details />
          <v-alert v-if="rawError" type="error" variant="tonal" class="mt-3">{{ rawError }}</v-alert>
        </v-card-text>
        <v-card-actions class="px-4 pb-3">
          <v-spacer />
          <v-btn variant="text" @click="rawDialog = false">{{ t('common.cancel') }}</v-btn>
          <v-btn color="primary" @click="applyRaw">{{ t('common.apply') }}</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import type { AppError } from '@/api/types'

const props = defineProps<{
  config: Record<string, unknown>
  saving?: boolean
}>()
const emit = defineEmits<{ save: [config: Record<string, unknown>] }>()

const { t } = useI18n()
const search = ref('')
const error = ref('')
const rawDialog = ref(false)
const rawText = ref('')
const rawError = ref('')
const complexValues = reactive<Record<string, string>>({})

const metaKeys = new Set(['platform', 'version', 'status', 'configVersion'])

const groups = [
  { id: 'audio', title: '音频 / Audio', pattern: /audio|sink|mic|sound/i },
  { id: 'video', title: '视频 / Video', pattern: /video|encoder|codec|fps|pixfmt|hdr|av1|hevc|nvenc|amf|vaapi|vt_|capture|display/i },
  { id: 'network', title: '网络 / Network', pattern: /port|address|bind|upnp|external|origin|csrf|wan|lan|dns|ipv/i },
  { id: 'input', title: '输入 / Input', pattern: /input|mouse|keyboard|gamepad|controller|x360|ds4|ds5|switch|motion|pen|touch/i },
  { id: 'stream', title: '串流 / Streaming', pattern: /stream|bitrate|fec|packet|ping|encryption|dscp|apps|prep|cmd|quality/i },
  { id: 'advanced', title: '高级 / Advanced', pattern: /.*/ },
]

const keys = computed(() => Object.keys(props.config).filter((key) => !metaKeys.has(key)))

const visibleSections = computed(() => {
  const needle = search.value.trim().toLowerCase()
  const filtered = keys.value.filter((key) => !needle || key.toLowerCase().includes(needle))
  const seen = new Set<string>()
  const sections = groups
    .map((group) => {
      const sectionKeys = filtered.filter((key) => {
        if (seen.has(key)) return false
        if (!group.pattern.test(key)) return false
        seen.add(key)
        return true
      })
      return { ...group, keys: sectionKeys }
    })
    .filter((section) => section.keys.length > 0)
  return sections
})

watch(
  () => props.config,
  (config) => {
    rawText.value = JSON.stringify(config, null, 2)
    for (const key of Object.keys(complexValues)) delete complexValues[key]
    for (const [key, value] of Object.entries(config)) {
      if (!isSimple(value)) complexValues[key] = JSON.stringify(value)
    }
  },
  { immediate: true, deep: false },
)

function isSimple(value: unknown) {
  return value === null || ['string', 'number', 'boolean'].includes(typeof value)
}

function fieldSpan(key: string) {
  return key.length > 28 ? 8 : 4
}

function setValue(key: string, value: unknown) {
  props.config[key] = value
  rawText.value = JSON.stringify(props.config, null, 2)
}

function applyComplex(key: string) {
  try {
    const parsed = JSON.parse(complexValues[key] || 'null')
    props.config[key] = parsed
    rawText.value = JSON.stringify(props.config, null, 2)
    error.value = ''
  } catch (err) {
    error.value = `解析 ${key} 失败: ${(err as AppError).message ?? err}`
  }
}

function applyRaw() {
  try {
    const parsed = JSON.parse(rawText.value) as Record<string, unknown>
    for (const key of Object.keys(props.config)) {
      if (!(key in parsed)) delete props.config[key]
    }
    Object.assign(props.config, parsed)
    rawError.value = ''
    rawDialog.value = false
  } catch (err) {
    rawError.value = (err as Error).message
  }
}

function save() {
  for (const key of Object.keys(complexValues)) {
    applyComplex(key)
  }
  emit('save', props.config)
}
</script>
