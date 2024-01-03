<template>
  <v-container class="fill-height">
    <v-data-table :items="items"></v-data-table>
  </v-container>
</template>

<script lang="ts" setup>
  import { ref } from 'vue';

  const items = ref([
    { name: 'John', age: 30 },
    { name: 'Jane', age: 24 },
    { name: 'Susan', age: 28 },
  ])
</script>