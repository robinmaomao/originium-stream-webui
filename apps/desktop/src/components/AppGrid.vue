<template>
  <div>
    <div class="d-flex ga-3 align-center mb-4 flex-wrap">
      <v-text-field
        v-model="search"
        :placeholder="t('hostDetail.searchPlaceholder')"
        prepend-inner-icon="mdi-magnify"
        hide-details
        density="compact"
        style="max-width: 320px"
      />
      <v-spacer />
      <v-btn variant="tonal" prepend-icon="mdi-refresh" :loading="loading" @click="load">
        {{ t('common.refresh') }}
      </v-btn>
    </div>

    <v-alert v-if="error" type="error" variant="tonal" class="mb-4">{{ error }}</v-alert>
    <v-alert v-else-if="!loading && filtered.length === 0" type="info" variant="tonal">
      {{ t('hostDetail.noApps') }}
    </v-alert>

    <div class="app-grid">
      <v-card
        v-for="app in filtered"
        :key="app.index"
        class="glass app-card"
        elevation="0"
        @click="$emit('launch', app.name)"
      >
        <div class="cover">
          <img v-if="covers[app.index]" :src="covers[app.index]" :alt="app.name" />
          <div v-else class="cover-fallback">
            {{ app.name.slice(0, 1).toUpperCase() }}
          </div>
          <div class="cover-overlay">
            <v-icon size="34" color="white" icon="mdi-play-circle" />
          </div>
        </div>
        <div class="pa-3">
          <div class="text-body-1 font-weight-medium text-truncate">{{ app.name }}</div>
          <div class="text-caption muted">{{ t('hostDetail.launch') }}</div>
        </div>
      </v-card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import * as api from '@/api'

interface AppEntry {
  name: string
  index: number
  imagePath: string
  raw: Record<string, unknown>
}

const props = defineProps<{ hostId: string }>()
defineEmits<{ launch: [name: string] }>()

const { t } = useI18n()
const apps = ref<AppEntry[]>([])
const covers = reactive<Record<number, string>>({})
const loading = ref(false)
const error = ref('')
const search = ref('')

const filtered = computed(() => {
  const needle = search.value.trim().toLowerCase()
  if (!needle) return apps.value
  return apps.value.filter((app) => app.name.toLowerCase().includes(needle))
})

watch(
  () => props.hostId,
  () => {
    void load()
  },
  { immediate: true },
)

function normalize(raw: unknown): AppEntry[] {
  const list = Array.isArray(raw)
    ? raw
    : ((raw as { apps?: unknown[] } | null)?.apps ?? [])
  return list.map((item, index) => {
    const record = (item ?? {}) as Record<string, unknown>
    return {
      name: String(record.name ?? `App ${index + 1}`),
      index: Number(record.index ?? index),
      imagePath: String(record.image_path ?? record.imagePath ?? ''),
      raw: record,
    }
  })
}

async function load() {
  loading.value = true
  error.value = ''
  try {
    apps.value = normalize(await api.getHostApps(props.hostId))
    await Promise.allSettled(
      apps.value.map(async (app) => {
        if (covers[app.index]) return
        try {
          covers[app.index] = await api.getCover(props.hostId, app.index)
        } catch {
          // cover is optional
        }
      }),
    )
  } catch (err) {
    error.value = api.errorMessage(err)
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.app-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(190px, 1fr));
  gap: 16px;
}
.app-card {
  cursor: pointer;
  border-radius: 18px;
  overflow: hidden;
  transition: transform 180ms ease, box-shadow 180ms ease;
}
.app-card:hover {
  transform: translateY(-3px);
  box-shadow: 0 18px 44px rgba(0, 0, 0, 0.4);
}
.cover {
  position: relative;
  aspect-ratio: 16 / 9;
  background: rgba(255, 255, 255, 0.04);
  overflow: hidden;
}
.cover img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}
.cover-fallback {
  width: 100%;
  height: 100%;
  display: grid;
  place-items: center;
  font-size: 2.2rem;
  font-weight: 700;
  color: rgba(255, 255, 255, 0.7);
  background: linear-gradient(140deg, rgba(124, 77, 255, 0.4), rgba(0, 229, 255, 0.22));
}
.cover-overlay {
  position: absolute;
  inset: 0;
  display: grid;
  place-items: center;
  background: rgba(8, 8, 12, 0.45);
  opacity: 0;
  transition: opacity 160ms ease;
}
.app-card:hover .cover-overlay {
  opacity: 1;
}
</style>
