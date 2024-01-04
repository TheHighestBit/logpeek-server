<template>
  <v-main>
    <Menu />
    <Suspense>
      <router-view />
    </Suspense>
  </v-main>
</template>

<script lang="ts" setup>
  import Menu from '@/components/Menu.vue';
</script>
