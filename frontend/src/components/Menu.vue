<template>
  <v-navigation-drawer expand-on-hover rail permanent>
    <v-list density="compact" nav>
      <v-list-item :to="{ name: 'Home' }" :exact="true" prepend-icon="mdi-folder" title="Home"></v-list-item>
      <v-list-item :to="{ name: 'LogTable' }" :exact="true" prepend-icon="mdi-table" title="Log Table"></v-list-item>
      <v-list-item @click="force_refresh" prepend-icon="mdi-refresh" title="Force Refresh"></v-list-item>
      <v-list-item @click="logout" prepend-icon="mdi-logout" title="Log out"></v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<script lang="ts" setup>
import {fetchWithAuth} from "@/utils";
import {useAppStore} from "@/store/app";

const store = useAppStore();
const logout = () => {
  if (localStorage.getItem('secret')) {
    localStorage.removeItem('secret');
    window.location.href = "/login";
  } else {
    store.showSnackbar("Failed to log out! Make sure authentication is enabled on the server side.", "error");
  }

}

const force_refresh = () => {
  // Authenticate is the cheapest endpoint to trigger the middleware
  fetchWithAuth("/api/authenticate", false, {
    "force-refresh": true,
  }).then((res) => {
    if (res.status === 200) {
      store.showSnackbar("Refresh successful", "success");
    }
  }).catch(() => {
    store.showSnackbar("Failed to force refresh", "error");
  });
}
</script>
