<template>
  <div class="appearance-settings">
    <v-card :title="'Appearance'">
      <v-card-text>
        <change-theme-settings />
      </v-card-text>
      <v-card-text>
        <change-font-size />
      </v-card-text>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import { ChangeFontSize } from '@/features/ChangeFontSize';
import { ChangeThemeSettings } from '@/features/ChangeThemeSettings';
</script>
