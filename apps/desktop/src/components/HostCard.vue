<template>
  <v-card class="glass host-card" elevation="0">
    <div class="d-flex align-start justify-space-between pa-4 pb-2">
      <div class="d-flex align-center ga-3 min-width-0">
        <div class="host-avatar">
          <v-icon size="22" icon="mdi-desktop-tower" />
        </div>
        <div class="min-width-0">
          <div class="text-subtitle-1 font-weight-bold text-truncate">{{ host.name }}</div>
          <div class="text-caption muted mono">{{ host.address }}:{{ host.port }}</div>
        </div>
      </div>
      <v-menu>
        <template #activator="{ props }">
          <v-btn icon variant="text" size="small" v-bind="props">
            <v-icon icon="mdi-dots-vertical" />
          </v-btn>
        </template>
        <v-list density="compact">
          <v-list-item :title="t('hosts.edit')" prepend-icon="mdi-pencil" @click="$emit('edit', host)" />
          <v-list-item
            :title="t('hosts.resetFingerprint')"
            prepend-icon="mdi-shield-refresh"
            @click="$emit('reset-fingerprint', host)"
          />
          <v-list-item
            :title="t('common.delete')"
            prepend-icon="mdi-delete-outline"
            base-color="error"
            @click="$emit('remove', host)"
          />
        </v-list>
      </v-menu>
    </div>

    <div class="px-4 d-flex flex-wrap ga-2">
      <v-chip size="small" variant="tonal" :color="host.paired ? 'success' : 'warning'">
        {{ host.paired ? t('hosts.paired') : t('hosts.unpaired') }}
      </v-chip>
      <v-chip size="small" variant="tonal" color="info">
        {{ host.source === 'mdns' ? 'mDNS' : t('hosts.addManual') }}
      </v-chip>
      <v-chip v-if="host.favorite" size="small" variant="tonal" color="warning">
        <v-icon start size="14" icon="mdi-star" />
        {{ t('common.yes') }}
      </v-chip>
    </div>

    <div class="px-4 py-3 text-caption muted">
      <div>{{ t('hosts.lastConnected') }}: {{ lastConnected }}</div>
      <div v-if="host.certFingerprint" class="mono text-truncate">
        {{ t('hosts.fingerprint') }}: {{ host.certFingerprint.slice(0, 24) }}…
      </div>
    </div>

    <v-divider opacity="0.08" />

    <div class="d-flex ga-2 pa-3">
      <v-btn color="primary" variant="flat" prepend-icon="mdi-connection" @click="$emit('connect', host)">
        {{ t('common.connect') }}
      </v-btn>
      <v-btn variant="tonal" prepend-icon="mdi-key-chain" @click="$emit('pair', host)">
        {{ t('common.pair') }}
      </v-btn>
      <v-spacer />
      <v-btn icon variant="text" :title="t('hosts.detail')" @click="$emit('detail', host)">
        <v-icon icon="mdi-arrow-right" />
      </v-btn>
    </div>
  </v-card>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Host } from '@/api/types'

const props = defineProps<{ host: Host }>()
defineEmits<{
  connect: [host: Host]
  pair: [host: Host]
  edit: [host: Host]
  remove: [host: Host]
  detail: [host: Host]
  'reset-fingerprint': [host: Host]
}>()

const { t } = useI18n()

const lastConnected = computed(() => {
  if (!props.host.lastConnectedAt) return t('hosts.neverConnected')
  return new Date(props.host.lastConnectedAt * 1000).toLocaleString()
})
</script>

<style scoped>
.host-card {
  border-radius: 20px;
  transition: transform 180ms ease, box-shadow 180ms ease;
}
.host-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 16px 40px rgba(0, 0, 0, 0.35);
}
.host-avatar {
  width: 42px;
  height: 42px;
  border-radius: 14px;
  display: grid;
  place-items: center;
  background: linear-gradient(140deg, rgba(124, 77, 255, 0.35), rgba(0, 229, 255, 0.2));
  color: #fff;
  flex-shrink: 0;
}
.min-width-0 {
  min-width: 0;
}
</style>
