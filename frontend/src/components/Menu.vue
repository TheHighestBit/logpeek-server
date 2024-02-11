<template>
  <v-navigation-drawer expand-on-hover rail permanent>
    <v-list density="compact" nav>
      <v-list-item :to="{ name: 'Home' }" :exact="true" prepend-icon="mdi-folder" title="Home"></v-list-item>
      <v-list-item :to="{ name: 'LogTable' }" :exact="true" prepend-icon="mdi-table" title="Log Table"></v-list-item>
      <v-list-item @click="logout" prepend-icon="mdi-logout" title="Log out"></v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<script lang="ts" setup>

  const logout = () => {
    localStorage.removeItem('secret');
    window.location.href = "/login";
  }
</script>
