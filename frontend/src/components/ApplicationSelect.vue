<template>
  <v-btn-toggle
    :model-value="props.application"
    @update:model-value="emit('update:application', $event)"
    background-color="primary"
    multiple
  >
    <v-btn
      v-for="app in application_list"
      :key="app"
      :value="app"
    >
      {{ app }}
    </v-btn>
  </v-btn-toggle>
</template>

<script setup lang="ts">
import {fetchWithAuth} from "@/utils";
import {useAppStore} from "@/store/app";

const store = useAppStore();

const props = defineProps({
  application: {
    type: Array,
    required: true,
    default: () => [],
  }
});

const application_list: string[] = await fetchWithAuth("/api/application_list").then((res) => res.json())
  .catch(() => {
    store.showSnackbar("Failed to fetch applications", "error");
  });

const emit = defineEmits(["update:application"]);
</script>
