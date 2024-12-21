import { defineStore } from 'pinia'

export const useThemeStore = defineStore('theme', {
  state: () => ({
    isDark: localStorage.getItem('theme') === 'dark'
  }),
  
  actions: {
    toggleTheme() {
      this.isDark = !this.isDark
      localStorage.setItem('theme', this.isDark ? 'dark' : 'light')
      this.applyTheme()
    },
    
    applyTheme() {
      document.documentElement.classList.toggle('dark', this.isDark)
      // 设置 Element Plus 主题
      document.documentElement.style.colorScheme = this.isDark ? 'dark' : 'light'
    },
    
    init() {
      this.applyTheme()
    }
  }
}) 