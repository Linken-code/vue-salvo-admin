import { defineStore } from 'pinia'
import request from '../utils/request'

export const usePermissionStore = defineStore('permission', {
  state: () => ({
    permissions: [], // 用户的所有权限
    menus: [] // 用户的菜单权限
  }),

  getters: {
    // 判断是否有某个权限
    hasPermission: (state) => (permission) => {
      if (!permission) return true
      return state.permissions.some(p => p.code === permission)
    },

    // 获取用户的菜单权限
    userMenus: (state) => {
      return state.menus
    }
  },

  actions: {
    // 设置权限
    setPermissions(permissions) {
      this.permissions = permissions
    },

    // 设置菜单
    setMenus(menus) {
      this.menus = menus
    },

    // 生成路由和权限
    async generateRoutes() {
      try {
        // 获取用户的权限列表
        const response = await request.get('/user/permissions')
        if (response.code === 0 && response.data) {
          const permissions = response.data.permissions || []
          const menus = response.data.menus || []

          // 设置权限和菜单
          this.setPermissions(permissions)
          this.setMenus(menus)

          return { permissions, menus }
        } else {
          throw new Error(response.message || '获取权限失败')
        }
      } catch (error) {
        console.error('Failed to generate routes:', error)
        throw error
      }
    },

    // 重置权限状态
    resetPermission() {
      this.permissions = []
      this.menus = []
    }
  }
}) 