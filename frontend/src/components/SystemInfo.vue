<template>
  <v-card class="ma-3 top-bar">
    <v-row align="center">
      <v-icon size="large" icon="mdi-cpu-64-bit"></v-icon>
      <p class="text-overline mr-3">CPU - {{ system_info.cpu_usage }}%</p>

      <v-icon size="large" icon="mdi-memory"></v-icon>
      <p class="text-overline mr-3">MEMORY - {{ system_info.memory_usage }}/{{ system_info.total_memory }} GB</p>

      <v-icon size="large" icon="mdi-server-network"></v-icon>
      <p class="text-overline mr-3">OS - {{ system_info.os }}</p>

      <v-icon size="large" icon="mdi-home-circle-outline"></v-icon>
      <p class="text-overline mr-3">HOST - {{ system_info.host_name }}</p>

      <v-icon size="large" icon="mdi-clock-time-eight-outline"></v-icon>
      <p class="text-overline mr-3">SYSTEM UPTIME - {{ system_info.uptime }}</p>

      <v-icon size="large" icon="mdi-timer-cog-outline"></v-icon>
      <p class="text-overline mr-3">SERVER UPTIME - {{ system_info.server_uptime }}</p>

      <v-spacer></v-spacer>

      <v-icon size="large" :color="statusColor">mdi-circle</v-icon>
    </v-row>
  </v-card>
</template>

<script setup lang="ts">
import { SystemInfo } from "@/interfaces/SystemInfo";
import {onBeforeUnmount, ref} from "vue";
import {fetchWithAuth} from "@/utils";
import {useAppStore} from "@/store/app";

const store = useAppStore();
const statusColor = ref<string>("white");

const system_info = ref<SystemInfo>(await fetchWithAuth("/api/sysinfo").then((res) => {
  statusColor.value = "green";
  return res.json();
}).catch(() => {
  store.showSnackbar("Failed to fetch system info", "error");
  statusColor.value = "red";
}));

const inervalID = setInterval(() => {
  fetchWithAuth("/api/sysinfo").then((res) => res.json()).then((data) => {
    system_info.value = data;
    statusColor.value = "green";
  }).catch(() => {
    statusColor.value = "red";
  });
}, 5000);

onBeforeUnmount(() => {
  clearInterval(inervalID);
});
</script>


<style scoped>
.top-bar { /* Apply styling to the v-card */
  width: 100%;
  padding: 15px; /* Add some padding for visual comfort */
}
</style>

