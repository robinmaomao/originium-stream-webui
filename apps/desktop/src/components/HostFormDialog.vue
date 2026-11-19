<template>
  <v-dialog v-model="open" max-width="640" persistent>
    <v-card class="glass pa-2">
      <v-card-title class="d-flex align-center ga-3">
        <v-icon color="primary">mdi-desktop-tower</v-icon>
        {{ host ? t('hosts.edit') : t('hosts.addManual') }}
      </v-card-title>
      <v-card-text>
        <v-form @submit.prevent="save">
          <v-row dense>
            <v-col cols="12" md="6">
              <v-text-field v-model="form.name" :label="t('hosts.name')" :rules="[required]" />
            </v-col>
            <v-col cols="12" md="8">
              <v-text-field v-model="form.address" :label="t('hosts.address')" :rules="[required]" placeholder="192.168.1.10" />
            </v-col>
            <v-col cols="12" md="4">
              <v-text-field v-model.number="form.port" type="number" :label="t('hosts.port')" />
            </v-col>
            <v-col cols="12" md="6">
              <v-text-field v-model="form.username" :label="t('hosts.account')" autocomplete="username" />
            </v-col>
            <v-col cols="12" md="6">
              <v-text-field
                v-model="form.password"
                type="password"
                :label="t('local.password')"
                :hint="host ? '留空表示保持原密码' : ''"
                autocomplete="current-password"
              />
            </v-col>
          </v-row>

          <v-alert
            v-if="testResult"
            :type="testResult.ok ? 'success' : 'warning'"
            variant="tonal"
            density="comfortable"
            class="mt-2"
          >
            {{ testResult.ok ? t('hosts.testSuccess') : t('hosts.pairFailed') }} ·
            {{ testResult.platform }} · {{ testResult.version }} · {{ testResult.latencyMs }} ms
          </v-alert>

          <v-alert v-if="error" type="error" variant="tonal" density="comfortable" class="mt-2">
            {{ error }}
          </v-alert>
        </v-form>
      </v-card-text>
      <v-card-actions class="px-4 pb-3">
        <v-btn variant="text" :loading="testing" prepend-icon="mdi-lan-connect" @click="test">
          {{ t('hosts.test') }}
        </v-btn>
        <v-spacer />
        <v-btn variant="text" @click="open = false">{{ t('common.cancel') }}</v-btn>
        <v-btn color="primary" :loading="saving" @click="save">{{ t('common.save') }}</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { reactive, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import * as api from '@/api'
import type { Host } from '@/api/types'
import { useAppStore } from '@/stores/app'
import { useHostsStore } from '@/stores/hosts'

const open = defineModel<boolean>({ default: false })
const props = defineProps<{ host?: Host | null }>()
const emit = defineEmits<{ saved: [host: Host] }>()

const { t } = useI18n()
const app = useAppStore()
const hosts = useHostsStore()

const form = reactive({
  name: '',
  address: '',
  port: 47990,
  username: '',
  password: '',
})
const saving = ref(false)
const testing = ref(false)
const error = ref('')
const testResult = ref<api.TestHostResult | null>(null)

watch(
  () => open.value,
  (value) => {
    if (!value) return
    testResult.value = null
    error.value = ''
    form.name = props.host?.name ?? ''
    form.address = props.host?.address ?? ''
    form.port = props.host?.port ?? 47990
    form.username = props.host?.username ?? ''
    form.password = ''
  },
  { immediate: true },
)

function required(value: unknown) {
  return Boolean(String(value ?? '').trim()) || '必填'
}

function payload() {
  return {
    id: props.host?.id,
    name: form.name.trim(),
    address: form.address.trim(),
    port: Number(form.port) || 47990,
    username: form.username.trim() || undefined,
    password: form.password || undefined,
    source: props.host?.source ?? ('manual' as const),
  }
}

async function test() {
  testing.value = true
  error.value = ''
  try {
    testResult.value = await api.testHost(payload())
  } catch (err) {
    error.value = api.errorMessage(err)
  } finally {
    testing.value = false
  }
}

async function save() {
  if (!form.name.trim() || !form.address.trim()) {
    error.value = '名称与地址不能为空'
    return
  }
  saving.value = true
  error.value = ''
  try {
    const host = props.host ? await hosts.update(payload()) : await hosts.add(payload())
    app.notify(t('common.save'), 'success')
    emit('saved', host)
    open.value = false
  } catch (err) {
    error.value = api.errorMessage(err)
  } finally {
    saving.value = false
  }
}
</script>
