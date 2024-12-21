<template>
  <el-container class="layout-container">
    <el-aside :width="isCollapse ? '64px' : '240px'" class="aside">
      <div class="logo" @click="router.push('/')">
        <img src="../assets/logo.svg" alt="Logo" />
        <span v-show="!isCollapse">Vue Salvo Admin</span>
      </div>
      <el-scrollbar>
        <el-menu :default-active="route.path" :collapse="isCollapse" :unique-opened="false" router class="menu">
          <template v-for="menu in menus" :key="menu.id">
            <!-- 有子菜单的情况 -->
            <el-sub-menu v-if="menu.children && menu.children.length > 0" :index="menu.path">
              <template #title>
                <el-icon v-if="menu.icon">
                  <component :is="menu.icon" />
                </el-icon>
                <span>{{ menu.title }}</span>
              </template>
              <el-menu-item v-for="child in menu.children" :key="child.id" :index="child.path">
                <el-icon v-if="child.icon">
                  <component :is="child.icon" />
                </el-icon>
                <template #title>{{ child.title }}</template>
              </el-menu-item>
            </el-sub-menu>
            <!-- 没有子菜单的情况 -->
            <el-menu-item v-else :index="menu.path">
              <el-icon v-if="menu.icon">
                <component :is="menu.icon" />
              </el-icon>
              <template #title>{{ menu.title }}</template>
            </el-menu-item>
          </template>
        </el-menu>
      </el-scrollbar>
    </el-aside>

    <el-container class="main-container">
      <el-header class="header" height="60px">
        <div class="left">
          <el-icon class="collapse-btn" @click="isCollapse = !isCollapse">
            <Fold v-if="!isCollapse" />
            <Expand v-else />
          </el-icon>
          <el-breadcrumb separator="/">
            <el-breadcrumb-item :to="{ path: '/' }">首页</el-breadcrumb-item>
            <el-breadcrumb-item v-if="route.meta.title">{{ route.meta.title }}</el-breadcrumb-item>
          </el-breadcrumb>
        </div>
        <div class="right">
          <el-space>
            <el-tooltip content="全屏" placement="bottom">
              <el-icon class="action-icon" @click="toggleFullscreen">
                <FullScreen v-if="!isFullscreen" />
                <Aim v-else />
              </el-icon>
            </el-tooltip>
            <el-tooltip content="主题设置" placement="bottom">
              <el-icon class="action-icon" @click="isDark = !isDark">
                <Sunny v-if="isDark" />
                <Moon v-else />
              </el-icon>
            </el-tooltip>
            <el-dropdown trigger="click" @command="handleCommand" :teleported="false">
              <div class="avatar-container">
                <el-avatar :size="32" :src="getAvatarUrl(userInfo?.avatar)" />
                <span class="username">{{ userInfo?.nickname || '用户' }}</span>
                <el-icon class="el-icon--right">
                  <CaretBottom />
                </el-icon>
              </div>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item command="profile">
                    <el-icon>
                      <User />
                    </el-icon>
                    个人信息
                  </el-dropdown-item>
                  <el-dropdown-item command="password">
                    <el-icon>
                      <Lock />
                    </el-icon>
                    修改密码
                  </el-dropdown-item>
                  <el-dropdown-item divided command="logout">
                    <el-icon>
                      <SwitchButton />
                    </el-icon>
                    退出登录
                  </el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </el-space>
        </div>
      </el-header>

      <el-main class="main">
        <router-view v-slot="{ Component }">
          <transition name="fade-transform" mode="out-in">
            <component :is="Component" />
          </transition>
        </router-view>
      </el-main>

      <el-footer class="footer" height="40px">
        <div class="copyright">
          Copyright © {{ new Date().getFullYear() }} Vue Salvo Admin. All rights reserved.
        </div>
      </el-footer>
    </el-container>
  </el-container>
</template>

<script setup>
import { ref, watch, onMounted, onBeforeUnmount } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { ElMessage } from 'element-plus';
import request from '../utils/request';
import emitter from '../utils/eventBus';
import * as ElementPlusIconsVue from '@element-plus/icons-vue';
import { Fold, Expand, CaretBottom, User, Lock, SwitchButton, FullScreen, Aim, Sunny, Moon } from '@element-plus/icons-vue';

const route = useRoute();
const router = useRouter();
const isCollapse = ref(false);
const menus = ref([]);
const userInfo = ref(null);
const isDark = ref(false);
const isFullscreen = ref(false);

const fetchMenus = async () => {
  try {
    const data = await request.get('/menus');
    // 处理菜单数据，构建树形结构
    const menuMap = new Map();
    const menuTree = [];

    // 先把所有菜单放入 Map
    data.forEach(menu => {
      menuMap.set(menu.id, { ...menu, children: [] });
    });

    // 构建树形结构
    data.forEach(menu => {
      const menuItem = menuMap.get(menu.id);
      if (menu.parent_id) {
        const parent = menuMap.get(menu.parent_id);
        if (parent) {
          parent.children.push(menuItem);
        }
      } else {
        menuTree.push(menuItem);
      }
    });

    menus.value = menuTree;
  } catch (error) {
    console.error('Failed to fetch menus:', error);
  }
};

const fetchUserInfo = async () => {
  try {
    // 先从 localStorage 获取用户信息
    const localUser = localStorage.getItem('user')
    if (localUser) {
      userInfo.value = JSON.parse(localUser)
    }
    // 然后从服务器获取最新信息
    const data = await request.get('/auth/current-user')
    if (data.user) {
      userInfo.value = data.user
      localStorage.setItem('user', JSON.stringify(data.user))
    }
  } catch (error) {
    console.error('Failed to fetch user info:', error)
  }
};

const handleCommand = (command) => {
  switch (command) {
    case 'profile':
      router.push('/profile');
      break;
    case 'password':
      router.push('/profile/password');
      break;
    case 'logout':
      localStorage.removeItem('token');
      localStorage.removeItem('user');
      router.push('/login');
      ElMessage.success('已退出登录');
      break;
  }
};

const toggleFullscreen = () => {
  if (!document.fullscreenElement) {
    document.documentElement.requestFullscreen();
    isFullscreen.value = true;
  } else {
    document.exitFullscreen();
    isFullscreen.value = false;
  }
};

// 监听主题变化
watch(isDark, (newValue) => {
  if (newValue) {
    document.documentElement.classList.add('dark');
  } else {
    document.documentElement.classList.remove('dark');
  }
});

// 监听全屏变化
const handleFullscreenChange = () => {
  isFullscreen.value = !!document.fullscreenElement;
};

const handleStorageChange = (e) => {
  if (e.key === 'user') {
    const user = JSON.parse(e.newValue);
    userInfo.value = user;
  }
};

const getAvatarUrl = (avatar) => {
  if (!avatar) return 'https://cube.elemecdn.com/3/7c/3ea6beec64369c2642b92c6726f1epng.png'
  return avatar.startsWith('http') ? avatar : `http://localhost:3000${avatar}`
}

onMounted(() => {
  fetchMenus();
  fetchUserInfo();
  document.addEventListener('fullscreenchange', handleFullscreenChange);
  // 监听用户信息更新事件
  emitter.on('user-updated', (user) => {
    userInfo.value = user;
  });
});

onBeforeUnmount(() => {
  document.removeEventListener('fullscreenchange', handleFullscreenChange);
  // 移除事件监听
  emitter.off('user-updated');
});
</script>

<style lang="scss" scoped>
.layout-container {
  height: 100vh;

  .aside {
    transition: width 0.3s;
    overflow: hidden;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.12);
    z-index: 1000;

    .logo {
      height: 60px;
      display: flex;
      align-items: center;
      padding: 0 20px;
      cursor: pointer;
      transition: background-color 0.3s;

      &:hover {
        background-color: var(--el-menu-hover-bg-color);
      }

      img {
        height: 32px;
        margin-right: 12px;
      }

      span {
        font-size: 18px;
        font-weight: 600;
        white-space: nowrap;
        overflow: hidden;
      }
    }

    .menu {
      border-right: none;
      height: calc(100vh - 60px);
    }
  }

  .main-container {
    .header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 0 20px;
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.08);
      background-color: var(--el-bg-color);
      z-index: 999;

      .left {
        display: flex;
        align-items: center;
        gap: 20px;

        .collapse-btn {
          font-size: 20px;
          cursor: pointer;
          transition: color 0.3s;

          &:hover {
            color: var(--el-color-primary);
          }
        }
      }

      .right {
        .action-icon {
          font-size: 32px;
          padding: 8px;
          cursor: pointer;
          color: var(--el-text-color-regular);
          transition: all 0.3s;
          border-radius: 4px;

          &:hover {
            color: var(--el-color-primary);
            background-color: var(--el-color-primary-light-9);
          }
        }

        .avatar-container {
          display: flex;
          align-items: center;
          cursor: pointer;
          padding: 0 8px;
          border-radius: 4px;
          transition: all 0.3s;

          &:hover {
            background-color: var(--el-color-primary-light-9);
          }

          .username {
            margin: 0 8px;
            color: var(--el-text-color-regular);
          }
        }
      }
    }

    .main {
      padding: 20px;
      background-color: var(--el-fill-color-blank);
    }

    .footer {
      display: flex;
      align-items: center;
      justify-content: center;
      background-color: var(--el-bg-color);
      color: var(--el-text-color-secondary);
      font-size: 14px;
      border-top: 1px solid var(--el-border-color-lighter);
    }
  }
}

// 路由切换动画
.fade-transform-enter-active,
.fade-transform-leave-active {
  transition: all 0.3s;
}

.fade-transform-enter-from {
  opacity: 0;
  transform: translateX(-30px);
}

.fade-transform-leave-to {
  opacity: 0;
  transform: translateX(30px);
}
</style>