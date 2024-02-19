<template>
  <v-main>
    <Menu />
    <Suspense>
      <router-view />
    </Suspense>
    <Snackbar />
  </v-main>
</template>

<script lang="ts" setup>
  import Menu from '@/components/Menu.vue';
  import Snackbar from "@/components/Snackbar.vue";
</script>
