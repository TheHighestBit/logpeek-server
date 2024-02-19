import { defineStore } from 'pinia';

export const useAppStore = defineStore('app', {
  state: () => ({
    isVisible: false,
    message: '',
    type: 'info', // Default type
    duration: 3000 // Default duration in milliseconds
  }),
  actions: {
    showSnackbar(message: string, type = 'info', duration = 3000) {
      this.message = message;
      this.type = type;
      this.isVisible = true;
      this.duration = duration;

      // Optionally: Close after timeout
      if (duration) {
        setTimeout(() => {
          this.isVisible = false;
        }, duration);
      }
    },
    hideSnackbar() {
      this.isVisible = false;
    }
  }
});
