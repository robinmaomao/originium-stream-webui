<template>
  <v-dialog v-model="open" max-width="480" persistent>
    <v-card class="glass pa-2">
      <v-card-title class="d-flex align-center ga-3">
        <v-icon color="primary">mdi-key-chain</v-icon>
        {{ t('common.pair') }} · {{ host?.name }}
      </v-card-title>
      <v-card-text>
        <p class="text-body-2 muted mb-4">{{ t('hosts.pairHint') }}</p>
        <v-text-field
          v-model="pin"
          :label="t('hosts.pairPin')"
          maxlength="4"
          inputmode="numeric"
          autofocus
          class="mono"
          @keyup.enter="submit"
        />
        <v-text-field v-model="name" :label="t('hosts.name')" class="mt-2" />

        <v-alert v-if="result" :type="result.paired ? 'success' : 'error'" variant="tonal" class="mt-3">
          {{ result.paired ? t('hosts.pairSuccess') : t('hosts.pairFailed') }} · {{ result.message }}
        </v-alert>
        <v-alert v-if="error" type="error" variant="tonal" class="mt-3">{{ error }}</v-alert>
      </v-card-text>
      <v-card-actions class="px-4 pb-3">
        <v-spacer />
        <v-btn variant="text" @click="open = false">{{ t('common.close') }}</v-btn>
        <v-btn color="primary" :loading="loading" :disabled="pin.length !== 4" @click="submit">
          {{ t('common.pair') }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import * as api from '@/api'
import type { Host, PairingResult } from '@/api/types'
import { useAppStore } from '@/stores/app'
import { useHostsStore } from '@/stores/hosts'

const open = defineModel<boolean>({ default: false })
const props = defineProps<{ host?: Host | null }>()
const emit = defineEmits<{ paired: [result: PairingResult] }>()

const { t } = useI18n()
const app = useAppStore()
const hosts = useHostsStore()

const pin = ref('')
const name = ref('')
const loading = ref(false)
const result = ref<PairingResult | null>(null)
const error = ref('')

watch(
  () => open.value,
  (value) => {
    if (!value) return
    pin.value = ''
    name.value = props.host?.name ?? ''
    result.value = null
    error.value = ''
  },
)

async function submit() {
  if (!props.host || pin.value.length !== 4) return
  loading.value = true
  error.value = ''
  try {
    result.value = await hosts.pair(props.host.id, pin.value, name.value || undefined)
    if (result.value.paired) {
      app.notify(t('hosts.pairSuccess'), 'success')
      emit('paired', result.value)
    }
  } catch (err) {
    error.value = api.errorMessage(err)
  } finally {
    loading.value = false
  }
}
</script>
