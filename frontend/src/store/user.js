import { defineStore } from 'pinia'
import request from '../utils/request'
import { ElMessage } from 'element-plus'

export const useUserStore = defineStore('user', {
  state: () => ({
    token: localStorage.getItem('token') || '',
    userInfo: null,
    roles: []
  }),

  getters: {
    isLogin: (state) => !!state.token,
    username: (state) => state.userInfo?.username || '',
    nickname: (state) => state.userInfo?.nickname || '',
    avatar: (state) => state.userInfo?.avatar || '',
    email: (state) => state.userInfo?.email || ''
  },

  actions: {
    // 登录
    async login(username, password) {
      try {
        const response = await request.post('/auth/login', { username, password })
        if (response.code === 0 && response.data) {
          this.token = response.data.token
          localStorage.setItem('token', response.data.token)
          return response.data
        } else {
          throw new Error(response.message || '登录失败')
        }
      } catch (error) {
        console.error('Login failed:', error)
        throw error
      }
    },

    // 获取用户信息
    async getUserInfo() {
      try {
        const response = await request.get('/auth/current-user')
        if (response.code === 0 && response.data) {
          this.userInfo = response.data
          this.roles = response.data.roles || []
          return response.data
        } else {
          throw new Error(response.message || '获取用户信息失败')
        }
      } catch (error) {
        console.error('Failed to get user info:', error)
        throw error
      }
    },

    // 更新用户信息
    async updateProfile(profile) {
      try {
        const response = await request.patch('/profile', profile)
        if (response.code === 0 && response.data) {
          this.userInfo = { ...this.userInfo, ...response.data }
          ElMessage.success('个人信息更新成功')
          return response.data
        } else {
          throw new Error(response.message || '更新个人信息失败')
        }
      } catch (error) {
        console.error('Failed to update profile:', error)
        throw error
      }
    },

    // 更新密码
    async updatePassword(oldPassword, newPassword) {
      try {
        const response = await request.patch('/profile/password', {
          old_password: oldPassword,
          new_password: newPassword
        })
        if (response.code === 0) {
          ElMessage.success('密码修改成功')
        } else {
          throw new Error(response.message || '修改密码失败')
        }
      } catch (error) {
        console.error('Failed to update password:', error)
        throw error
      }
    },

    // 登出
    async logout() {
      this.token = ''
      this.userInfo = null
      this.roles = []
      localStorage.removeItem('token')
    },

    // 重置token
    async resetToken() {
      this.token = ''
      this.userInfo = null
      this.roles = []
      localStorage.removeItem('token')
    }
  }
}) 