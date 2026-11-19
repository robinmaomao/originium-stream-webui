<template>
  <div>
    <div class="d-flex ga-3 align-center mb-3 flex-wrap">
      <v-text-field
        v-model="search"
        :placeholder="t('logs.search')"
        prepend-inner-icon="mdi-magnify"
        hide-details
        density="compact"
        style="max-width: 300px"
      />
      <v-select
        v-if="entries"
        v-model="level"
        :items="levels"
        hide-details
        density="compact"
        style="max-width: 150px"
      />
      <v-spacer />
      <v-btn variant="tonal" prepend-icon="mdi-refresh" :loading="loading" @click="$emit('refresh')">
        {{ t('common.refresh') }}
      </v-btn>
      <v-btn v-if="entries" variant="text" color="error" prepend-icon="mdi-delete-outline" @click="$emit('clear')">
        {{ t('logs.clear') }}
      </v-btn>
    </div>

    <v-card class="glass log-card" elevation="0">
      <div v-if="entries" class="scroll-area log-scroll">
        <v-virtual-scroll :items="filteredEntries" height="520" item-height="26">
          <template #default="{ item }">
            <div class="log-row mono">
              <span class="log-time">{{ item.timestamp }}</span>
              <span class="log-level" :class="item.level.toLowerCase()">{{ item.level.toUpperCase() }}</span>
              <span class="log-target">[{{ item.target }}]</span>
              <span class="log-message">{{ item.message }}</span>
            </div>
          </template>
        </v-virtual-scroll>
        <div v-if="filteredEntries.length === 0" class="pa-6 text-center muted">
          {{ t('logs.empty') }}
        </div>
      </div>
      <div v-else class="scroll-area log-scroll">
        <pre class="text-lines mono">{{ filteredText || t('logs.empty') }}</pre>
      </div>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import type { LogEntry } from '@/api/types'

const props = defineProps<{
  entries?: LogEntry[]
  text?: string
  loading?: boolean
}>()
defineEmits<{ refresh: []; clear: [] }>()

const { t } = useI18n()
const search = ref('')
const level = ref('all')
const levels = [
  { title: t('logs.all'), value: 'all' },
  { title: 'INFO', value: 'info' },
  { title: 'WARN', value: 'warn' },
  { title: 'ERROR', value: 'error' },
]

const filteredEntries = computed(() => {
  const needle = search.value.trim().toLowerCase()
  return (props.entries ?? []).filter((entry) => {
    const levelMatch = level.value === 'all' || entry.level.toLowerCase() === level.value
    const searchMatch =
      !needle ||
      entry.message.toLowerCase().includes(needle) ||
      entry.target.toLowerCase().includes(needle)
    return levelMatch && searchMatch
  })
})

const filteredText = computed(() => {
  const lines = (props.text ?? '').split('\n')
  const needle = search.value.trim().toLowerCase()
  if (!needle) return (props.text ?? '').trim()
  return lines
    .filter((line) => line.toLowerCase().includes(needle))
    .join('\n')
    .trim()
})
</script>

<style scoped>
.log-card {
  border-radius: 18px;
  overflow: hidden;
}
.log-scroll {
  max-height: 520px;
}
.log-row {
  display: flex;
  gap: 10px;
  align-items: baseline;
  padding: 3px 14px;
  font-size: 0.74rem;
  line-height: 20px;
  white-space: nowrap;
}
.log-row:hover {
  background: rgba(124, 77, 255, 0.08);
}
.log-time {
  color: rgba(232, 232, 240, 0.4);
  flex-shrink: 0;
}
.log-level {
  width: 46px;
  flex-shrink: 0;
  font-weight: 600;
}
.log-level.info {
  color: var(--originium-info);
}
.log-level.warn {
  color: var(--originium-warning);
}
.log-level.error {
  color: var(--originium-error);
}
.log-target {
  color: rgba(0, 229, 255, 0.65);
  flex-shrink: 0;
}
.log-message {
  overflow: hidden;
  text-overflow: ellipsis;
}
.text-lines {
  margin: 0;
  padding: 16px;
  font-size: 0.74rem;
  line-height: 1.6;
  white-space: pre-wrap;
  word-break: break-all;
  color: rgba(232, 232, 240, 0.82);
  max-height: 520px;
  overflow: auto;
}
</style>
