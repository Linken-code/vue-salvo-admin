<template>
  <div class="app-container">
    <el-container>
      <el-aside :width="isCollapse ? '64px' : '200px'">
        <div class="logo" :class="{ 'logo-collapse': isCollapse }">
          <span v-if="!isCollapse">Vue Salvo Admin</span>
          <el-icon v-else>
            <Monitor />
          </el-icon>
        </div>
        <el-menu :default-active="route.path" class="el-menu-vertical" :collapse="isCollapse" background-color="#304156"
          text-color="#bfcbd9" active-text-color="#409EFF" router>
          <template v-for="menu in menus" :key="menu.id">
            <!-- 有子菜单的情况 -->
            <el-sub-menu v-if="menu.children && menu.children.length > 0" :index="menu.path">
              <template #title>
                <el-icon v-if="menu.icon">
                  <component :is="menu.icon" />
                </el-icon>
                <span>{{ menu.title }}</span>
              </template>
              <el-menu-item v-for="child in menu.children" :key="child.id" :index="child.path" :route="child.path">
                <el-icon v-if="child.icon">
                  <component :is="child.icon" />
                </el-icon>
                <template #title>{{ child.title }}</template>
              </el-menu-item>
            </el-sub-menu>
            <!-- 没有子菜单的情况 -->
            <el-menu-item v-else :index="menu.path" :route="menu.path">
              <el-icon v-if="menu.icon">
                <component :is="menu.icon" />
              </el-icon>
              <template #title>{{ menu.title }}</template>
            </el-menu-item>
          </template>
        </el-menu>
      </el-aside>
      <el-container>
        <el-header>
          <div class="header-left">
            <el-icon class="collapse-btn" @click="toggleCollapse">
              <component :is="isCollapse ? 'Expand' : 'Fold'" />
            </el-icon>
          </div>
          <div class="header-right">
            <el-dropdown trigger="click">
              <span class="user-dropdown">
                <el-avatar :size="32" :src="userInfo?.avatar">
                  {{ userInfo?.nickname?.charAt(0) }}
                </el-avatar>
                <span class="username">{{ userInfo?.nickname }}</span>
              </span>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item @click="router.push('/profile')">个人信息</el-dropdown-item>
                  <el-dropdown-item @click="handleLogout">退出登录</el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </div>
        </el-header>
        <el-main>
          <router-view v-slot="{ Component }">
            <transition name="fade" mode="out-in">
              <component :is="Component" />
            </transition>
          </router-view>
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { Monitor, Expand, Fold } from '@element-plus/icons-vue'
import axios from 'axios'

const route = useRoute()
const router = useRouter()
const isCollapse = ref(false)
const menus = ref([])
const userInfo = ref(null)

const toggleCollapse = () => {
  isCollapse.value = !isCollapse.value
}

const handleLogout = () => {
  localStorage.removeItem('token')
  router.push('/login')
}

const fetchMenus = async () => {
  try {
    const response = await axios.get('http://localhost:3000/menus')
    const menuData = response.data
    menus.value = buildMenuTree(menuData)
  } catch (error) {
    ElMessage.error('获取菜单失败')
  }
}

const buildMenuTree = (items, parentId = null) => {
  return items
    .filter(item => !item.is_hidden && item.parent_id === parentId)
    .sort((a, b) => a.sort - b.sort)
    .map(item => ({
      ...item,
      children: buildMenuTree(items, item.id)
    }))
}

const fetchUserInfo = async () => {
  try {
    const token = localStorage.getItem('token')
    if (!token) {
      router.push('/login')
      return
    }

    const response = await axios.get('http://localhost:3000/auth/current-user', {
      headers: {
        Authorization: `Bearer ${token}`
      }
    })
    userInfo.value = response.data
  } catch (error) {
    ElMessage.error('获取用户信息失败')
    console.error('Error:', error)
    router.push('/login')
  }
}

onMounted(() => {
  fetchMenus()
  fetchUserInfo()
})
</script>

<style scoped>
.app-container {
  height: 100vh;
}

.el-container {
  height: 100%;
}

.el-aside {
  background-color: #304156;
  transition: width 0.3s;
}

.logo {
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #2b2f3a;
  color: #fff;
  font-size: 18px;
  font-weight: bold;
  transition: all 0.3s;
  overflow: hidden;
  white-space: nowrap;
}

.logo-collapse {
  font-size: 24px;
}

.el-menu-vertical {
  border-right: none;
}

.el-menu-vertical:not(.el-menu--collapse) {
  width: 200px;
}

.el-header {
  background-color: #fff;
  border-bottom: 1px solid #e6e6e6;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
}

.header-left {
  display: flex;
  align-items: center;
}

.collapse-btn {
  font-size: 20px;
  cursor: pointer;
  transition: all 0.3s;
}

.collapse-btn:hover {
  color: #409EFF;
}

.header-right {
  display: flex;
  align-items: center;
}

.user-dropdown {
  display: flex;
  align-items: center;
  cursor: pointer;
}

.username {
  margin-left: 8px;
  font-size: 14px;
}

.el-main {
  background-color: #f0f2f5;
  padding: 20px;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>