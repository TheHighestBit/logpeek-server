<template>
  <v-snackbar v-model="isVisible" :timeout="duration" :color="type">
    {{ message }}
  </v-snackbar>
</template>

<script setup lang="ts">
import { useAppStore } from '../store/app';
import {computed} from "vue";

const snackbarStore = useAppStore();

// You don't need mapState with 'setup'
const isVisible = computed(() => snackbarStore.isVisible);
const message = computed(() => snackbarStore.message);
const type = computed(() => snackbarStore.type);
const duration = computed(() => snackbarStore.duration);
</script>
