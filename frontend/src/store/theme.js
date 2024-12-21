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
      if (this.isDark) {
        document.documentElement.classList.add('dark')
        document.documentElement.setAttribute('data-theme', 'dark')
      } else {
        document.documentElement.classList.remove('dark')
        document.documentElement.removeAttribute('data-theme')
      }
      // 设置Element Plus主题
      document.documentElement.style.colorScheme = this.isDark ? 'dark' : 'light'
    },
    
    init() {
      this.applyTheme()
    }
  }
}) 