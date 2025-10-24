// /src/stores/settings.ts
import { defineStore } from 'pinia'

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    // 🔷 General project info
    name: '',
    identifier: '',
    author: '',
    version: '1.0.0',
    description: '',
    
    // 🔷 App preferences
    theme: 'system', // 'dark' | 'light' | 'system'
    autoSave: true,
    lastOpenedProject: null,

    // 🔷 Default paths (for Tauri file operations)
    exportPath: '',
  }),

  actions: {
    load() {
      try {
        const data = localStorage.getItem('addonSettings')
        if (data) Object.assign(this, JSON.parse(data))
      } catch (err) {
        console.warn('Failed to load settings:', err)
      }
    },

    save() {
      localStorage.setItem('addonSettings', JSON.stringify(this.$state))
    },

    reset() {
      localStorage.removeItem('addonSettings')
      this.$reset()
    },
  },
})
