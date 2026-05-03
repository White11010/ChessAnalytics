<template>
  <v-container fluid>
    <p class="text-title-medium">Font size</p>
    <v-radio-group v-model="fontMode" inline hide-details>
      <v-radio label="Normal" value="normal" />
      <v-radio class="ml-2" label="Large (+25%)" value="large" />
    </v-radio-group>
  </v-container>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';

const FONT_KEY = 'app-font-size';

type FontMode = 'normal' | 'large';

const fontMode = ref<FontMode>('normal');

function applyFont(mode: FontMode): void {
  const scale = mode === 'large' ? 1.25 : 1;
  document.documentElement.style.setProperty('--app-font-scale', String(scale));
}

/**
 * init
 */
onMounted(() => {
  const saved = localStorage.getItem(FONT_KEY) as FontMode | null;

  fontMode.value = saved === 'large' ? 'large' : 'normal';

  applyFont(fontMode.value);
});

/**
 * sync
 */
watch(fontMode, (val) => {
  applyFont(val);
  localStorage.setItem(FONT_KEY, val);
});
</script>
