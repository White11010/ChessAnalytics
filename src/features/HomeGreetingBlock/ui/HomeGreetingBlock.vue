<template>
  <v-card v-if="user" title="Wellcome"> Connected: Lichess {{ user.username }} </v-card>
</template>

<script setup lang="ts">
import { useGetUserQuery } from '@/entities/user';

const { user } = useGetUserQuery();
</script>
