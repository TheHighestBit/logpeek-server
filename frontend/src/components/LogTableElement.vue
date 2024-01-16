<template>
  <v-container fluid>
    <v-row>
      <v-col cols="auto"><p class="text">{{ index }}.</p></v-col>
      <v-col cols="auto"><p class="text">{{ timestamp }}</p></v-col>
      <v-col cols="auto">
        <p v-if="level === 'ERROR'" class="text" style="color: #ff0335">{{ level }}</p>
        <p v-else-if="level === 'WARN'" class="text" style="color: #FFC107">{{ level }}</p>
        <p v-else-if="level === 'INFO'" class="text" style="color: #2ebb36">{{ level }}</p>
        <p v-else-if="level === 'DEBUG'" class="text" style="color: #2196f3">{{ level }}</p>
        <p v-else class="text" style="color: #8764a2">{{ level }}</p>
      </v-col>
      <v-col cols="auto"><p class="text">{{ module_name }}</p></v-col>
      <v-col cols="auto"><p class="text">{{ message }}</p></v-col>
    </v-row>
  </v-container>
  <v-divider></v-divider>
</template>

<script setup lang="ts">
const props = defineProps({
  index: Number,
  timestamp: String,
  level: String,
  module_name: String,
  message: String,
});
</script>


<style scoped>

</style>
