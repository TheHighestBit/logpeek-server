<template>
  <div class="login-container">
    <div class="login-form">
      <v-row>
        <v-text-field v-model="password" type="password" label="Password"></v-text-field>
        <v-btn type="button" @click="validate_password" class="ml-4 mt-3">Submit</v-btn>
      </v-row>
      <div class="error-message" v-if="errorMessage">{{ errorMessage }}</div> </div>
  </div>
</template>


<script setup lang="ts">
import {onMounted, ref} from "vue";
import { fetchWithAuth } from "@/utils";

const password = ref('');
const errorMessage = ref('');

onMounted(() => {
  window.addEventListener("keydown", (e) => {
    if (e.key === "Enter") {
      validate_password();
      
    }
  });
})

const validate_password = async () => {
  localStorage.setItem("secret", password.value);
  const response = await fetchWithAuth("api/authenticate", false);

  switch (response.status) {
    case 401:
      errorMessage.value = "Invalid password";
      password.value = "";
      break;
    case 400:
      errorMessage.value = "Server error (400)";
      break;
    case 429:
      errorMessage.value = "Too many login attempts! The server has been locked and requires a manual restart.";
      break;
    default:
      window.location.href = "/";
  }
};

</script>

<style scoped>

.login-container {
  height: 100vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.login-form {
  width: 300px;
  padding: 20px;
  border: 1px solid #ddd;
  border-radius: 5px;
}

.error-message {
  color: red;
  margin-bottom: 10px;
}
</style>
