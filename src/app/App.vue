<template>
  <v-layout class="rounded rounded-md border" style="height: 100vh">
    <v-navigation-drawer permanent :width="240">
      <v-list-item
        class="pa-4"
        title="ChessLens"
        subtitle="Analytics platform"
        @click="router?.push('/')"
      ></v-list-item>
      <v-divider></v-divider>
      <v-list-item
        class="mt-4"
        :active="route?.name === 'Home'"
        link
        title="Home"
        prepend-icon="mdi-home"
        @click="router?.push('/')"
      ></v-list-item>
      <v-list-item
        class="mt-2"
        :active="route?.name === 'Insights'"
        link
        title="Insights"
        prepend-icon="mdi-creation"
        @click="router?.push('/insights')"
      ></v-list-item>
      <v-list-item
        class="mt-2"
        :active="route?.name === 'MyGames'"
        link
        title="My games"
        prepend-icon="mdi-table-large"
        @click="router?.push('/my-games')"
      ></v-list-item>
      <v-list-item
        class="mt-2"
        :active="route?.name === 'AnalizeBoard'"
        link
        title="Analize Board"
        prepend-icon="mdi-microscope"
        @click="router?.push('/analize-board')"
      ></v-list-item>

      <template #append>
        <v-list-item
          class="mb-4"
          :active="route?.name === 'Settings'"
          link
          title="Settings"
          prepend-icon="mdi-cog"
          @click="router?.push('/settings')"
        ></v-list-item>
      </template>
    </v-navigation-drawer>
    <v-main class="d-flex align-center justify-center">
      <router-view></router-view>
    </v-main>
  </v-layout>
</template>

<script setup lang="ts">
import { useRoute, useRouter } from 'vuetify/lib/composables/router.mjs';

const route = useRoute();
const router = useRouter();
</script>

<style scoped>
.container {
  padding: 2rem;
}
</style>
<style>
.button--medium {
  height: 3rem;
}
.button-width-12 {
  width: 12rem;
}

:root {
  --app-font-scale: 1;
}

html {
  font-size: calc(16px * var(--app-font-scale));
}
</style>
