import { defineStore } from 'pinia'
import { isTauri } from '@tauri-apps/api/core'

export const useAppStore = defineStore('app', {
  state: () => ({
    isTauri: isTauri()
  })
})
