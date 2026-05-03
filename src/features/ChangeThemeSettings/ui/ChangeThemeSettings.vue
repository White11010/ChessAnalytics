<template>
  <v-container fluid>
    <p class="text-title-medium">Theme</p>
    <v-radio-group v-model="themeMode" inline hide-details>
      <v-radio label="Light" value="light" />
      <v-radio class="ml-2" label="Dark" value="dark" />
    </v-radio-group>
  </v-container>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useTheme } from 'vuetify';

const THEME_KEY = 'app-theme';

const theme = useTheme();

const themeMode = ref<'light' | 'dark'>(theme.global.name.value as 'light' | 'dark');

watch(themeMode, (val) => {
  theme.change(val);
  localStorage.setItem(THEME_KEY, val);
});
</script>
