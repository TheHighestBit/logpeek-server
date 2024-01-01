/**
 * main.ts
 *
 * Bootstraps Vuetify and other plugins then mounts the App`
 */

// Plugins
import { registerPlugins } from '@/plugins'

// Components
import App from './App.vue'

// Composables
import { createApp } from 'vue'
import "vue-data-ui/style.css";

const app = createApp(App)

registerPlugins(app)

app.mount('#app')
