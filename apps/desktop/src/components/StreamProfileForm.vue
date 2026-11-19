<template>
  <div class="d-flex flex-column ga-3">
    <div class="d-flex ga-3 flex-wrap">
      <v-select
        :model-value="resolutionPreset"
        :items="resolutionOptions"
        :label="t('profile.resolution')"
        style="min-width: 180px"
        @update:model-value="applyResolution"
      />
      <v-text-field v-model.number="model.width" type="number" :label="t('profile.width')" style="max-width: 130px" />
      <v-text-field v-model.number="model.height" type="number" :label="t('profile.height')" style="max-width: 130px" />
      <v-text-field v-model.number="model.fps" type="number" :label="t('profile.fps')" style="max-width: 130px" />
      <v-text-field
        v-model.number="model.bitrateKbps"
        type="number"
        :label="t('profile.bitrate')"
        style="max-width: 170px"
      />
    </div>

    <div class="d-flex ga-3 flex-wrap">
      <v-select v-model="model.videoCodec" :items="codecs" :label="t('profile.codec')" style="min-width: 160px" />
      <v-select v-model="model.audioConfig" :items="audioOptions" :label="t('profile.audio')" style="min-width: 170px" />
      <v-select v-model="model.displayMode" :items="displayOptions" :label="t('profile.display')" style="min-width: 170px" />
      <v-select
        v-model="model.captureSystemKeys"
        :items="captureOptions"
        :label="t('profile.captureKeys')"
        style="min-width: 180px"
      />
    </div>

    <div class="d-flex flex-wrap ga-6">
      <v-switch v-model="model.hdr" :label="t('profile.hdr')" hide-details />
      <v-switch v-model="model.yuv444" :label="t('profile.yuv444')" hide-details />
      <v-switch v-model="model.vsync" :label="t('profile.vsync')" hide-details />
      <v-switch v-model="model.framePacing" :label="t('profile.framePacing')" hide-details />
      <v-switch v-model="model.keepAwake" :label="t('profile.keepAwake')" hide-details />
      <v-switch v-model="model.performanceOverlay" :label="t('profile.performanceOverlay')" hide-details />
      <v-switch v-model="model.quitAfter" :label="t('profile.quitAfter')" hide-details />
    </div>

    <div class="d-flex ga-3 flex-wrap">
      <v-text-field
        v-model.number="packetSize"
        type="number"
        :label="t('profile.packetSize')"
        style="max-width: 180px"
      />
      <v-textarea
        v-model="extraArgs"
        :label="t('profile.extraArgs')"
        rows="2"
        auto-grow
        class="flex-grow-1"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { StreamProfile } from '@/api/types'

const model = defineModel<StreamProfile>({ required: true })
const { t } = useI18n()

const codecs = [
  { title: t('profile.auto'), value: 'auto' },
  { title: 'H.264', value: 'H.264' },
  { title: 'HEVC', value: 'HEVC' },
  { title: 'AV1', value: 'AV1' },
]
const audioOptions = [
  { title: t('profile.stereo'), value: 'stereo' },
  { title: t('profile.surround51'), value: '5.1-surround' },
  { title: t('profile.surround71'), value: '7.1-surround' },
]
const displayOptions = [
  { title: t('profile.borderless'), value: 'borderless' },
  { title: t('profile.fullscreen'), value: 'fullscreen' },
  { title: t('profile.windowed'), value: 'windowed' },
]
const captureOptions = [
  { title: t('profile.never'), value: 'never' },
  { title: t('profile.fullscreen'), value: 'fullscreen' },
  { title: t('profile.always'), value: 'always' },
]
const resolutionOptions = [
  { title: '1280 × 720', value: '1280x720' },
  { title: '1920 × 1080', value: '1920x1080' },
  { title: '2560 × 1440', value: '2560x1440' },
  { title: '3840 × 2160', value: '3840x2160' },
  { title: t('profile.width') + ' × ' + t('profile.height'), value: 'custom' },
]

const resolutionPreset = computed(() => {
  const key = `${model.value.width}x${model.value.height}`
  return resolutionOptions.some((option) => option.value === key) ? key : 'custom'
})

const packetSize = computed({
  get: () => model.value.packetSize ?? undefined,
  set: (value: number | undefined) => {
    model.value.packetSize = value && value > 1024 ? value : null
  },
})

const extraArgs = computed({
  get: () => model.value.extraArgs.join('\n'),
  set: (value: string) => {
    model.value.extraArgs = value
      .split('\n')
      .map((line) => line.trim())
      .filter(Boolean)
  },
})

function applyResolution(value: string) {
  if (value === 'custom') return
  const [width, height] = value.split('x').map(Number)
  model.value.width = width
  model.value.height = height
}
</script>
