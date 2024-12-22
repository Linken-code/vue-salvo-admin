<template>
  <el-container class="layout-container">
    <el-aside :width="isCollapse ? '64px' : '240px'" class="aside">
      <div class="logo" :class="{ collapsed: isCollapse }" @click="router.push('/')">
        <img src="../assets/logo.svg" alt="Logo" />
        <span>Jing Salvo Admin</span>
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
            <el-dropdown trigger="click" @command="handleCommand" popper-class="user-dropdown">
              <div class="avatar-container" tabindex="0" role="button" aria-haspopup="true" aria-expanded="false">
                <el-avatar :size="32" :src="getAvatarUrl(userInfo?.avatar)" />
                <span class="username">{{ userInfo?.nickname || '用户' }}</span>
                <el-icon class="el-icon--right">
                  <CaretBottom />
                </el-icon>
              </div>
              <template #dropdown>
                <el-dropdown-menu role="menu" aria-label="用户菜单">
                  <el-dropdown-item command="profile" role="menuitem" tabindex="0">
                    <el-icon>
                      <User />
                    </el-icon>
                    个人信息
                  </el-dropdown-item>
                  <el-dropdown-item command="password" role="menuitem" tabindex="0">
                    <el-icon>
                      <Lock />
                    </el-icon>
                    修改密码
                  </el-dropdown-item>
                  <el-dropdown-item divided command="logout" role="menuitem" tabindex="0">
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
          Copyright © {{ new Date().getFullYear() }} Jing Salvo Admin. All rights reserved.
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
  background: linear-gradient(135deg, var(--primary-light-9) 0%, var(--bg-color-page) 100%);
  position: relative;
  overflow: hidden;

  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(135deg, var(--primary-light-7) 0%, var(--primary-color) 100%);
    opacity: 0.02;
    pointer-events: none;
  }

  .aside {
    transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    background: linear-gradient(180deg, var(--primary-light-9) 0%, var(--primary-light-8) 100%);
    box-shadow: 0 0 20px rgba(0, 0, 0, 0.05);
    z-index: 1000;
    overflow: hidden;
    border-right: none;
    position: relative;

    &::before {
      content: '';
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background: linear-gradient(180deg, var(--primary-light-8) 0%, var(--primary-light-7) 100%);
      opacity: 0.03;
      pointer-events: none;
    }

    .logo {
      height: 60px;
      display: flex;
      align-items: center;
      padding: 0 20px;
      cursor: pointer;
      position: relative;
      overflow: hidden;
      border-bottom: 1px solid rgba(255, 255, 255, 0.2);
      background: linear-gradient(135deg, var(--primary-light-5) 0%, var(--primary-color) 100%);
      transition: padding 0.3s cubic-bezier(0.4, 0, 0.2, 1);

      img {
        height: 32px;
        width: 32px;
        margin-right: 12px;
        filter: brightness(0) invert(1);
        position: relative;
        z-index: 1;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        object-fit: contain;
      }

      span {
        font-size: 18px;
        font-weight: 600;
        color: white;
        white-space: nowrap;
        overflow: hidden;
        text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        position: relative;
        z-index: 1;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        opacity: 1;
        transform: translateX(0);
      }

      &.collapsed {
        padding: 0 16px;
        justify-content: center;

        img {
          margin-right: 0;
          width: 32px;
          height: 32px;
        }

        span {
          width: 0;
          margin-left: 0;
          opacity: 0;
          transform: translateX(-20px);
        }
      }
    }

    :deep(.el-menu) {
      border-right: none;
      background-color: transparent;

      &.el-menu--collapse {
        width: 64px;

        .el-menu-item,
        .el-sub-menu__title {
          padding: 0 20px !important;
        }

        .el-sub-menu__title {
          .el-sub-menu__icon-arrow {
            display: none;
          }
        }
      }

      .el-menu-item,
      .el-sub-menu__title {
        height: 50px;
        line-height: 50px;
        color: var(--text-primary);
        background-color: transparent;
        margin: 4px 8px;
        border-radius: var(--border-radius-base);
        transition: all 0.3s;

        .el-icon {
          width: 24px;
          height: 24px;
          text-align: center;
          font-size: 18px;
        }

        &:hover {
          color: var(--primary-color);
          background: linear-gradient(135deg, rgba(255, 255, 255, 0.95) 0%, rgba(255, 255, 255, 0.98) 100%);
          transform: translateX(4px);
        }

        .el-icon {
          color: inherit;
          margin-right: 8px;
          transition: all 0.3s;
        }
      }

      .el-menu-item.is-active {
        color: white;
        background: linear-gradient(135deg, var(--primary-light-5) 0%, var(--primary-color) 100%);
        border-right: none;
        transform: translateX(4px);
        box-shadow: 0 4px 12px rgba(64, 158, 255, 0.2);

        .el-icon {
          color: white;
        }
      }

      .el-sub-menu.is-active {
        >.el-sub-menu__title {
          color: var(--primary-color);
          background: linear-gradient(135deg, rgba(255, 255, 255, 0.95) 0%, rgba(255, 255, 255, 0.98) 100%);

          .el-icon {
            color: var(--primary-color);
          }
        }
      }

      .el-sub-menu {
        .el-menu {
          background-color: rgba(255, 255, 255, 0.02);
          padding: 4px;

          .el-menu-item {
            margin: 4px;
            height: 44px;
            line-height: 44px;
            border-radius: var(--border-radius-base);

            &:hover {
              background: linear-gradient(135deg, rgba(255, 255, 255, 0.95) 0%, rgba(255, 255, 255, 0.98) 100%);
            }

            &.is-active {
              background: linear-gradient(135deg, var(--primary-light-5) 0%, var(--primary-color) 100%);
              color: white;

              .el-icon {
                color: white;
              }
            }
          }
        }
      }
    }
  }

  .main-container {
    position: relative;
    z-index: 1;
    display: flex;
    flex-direction: column;
    min-height: 100vh;

    .header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 0 20px;
      background: linear-gradient(135deg, var(--primary-light-8) 0%, rgba(255, 255, 255, 0.95) 100%);
      backdrop-filter: blur(10px);
      border-bottom: 1px solid rgba(255, 255, 255, 0.2);
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
      z-index: 999;
      height: 60px;

      .left {
        display: flex;
        align-items: center;
        gap: 20px;

        .collapse-btn {
          font-size: 20px;
          cursor: pointer;
          color: var(--text-regular);
          transition: all 0.3s;
          padding: 8px;
          width: 36px;
          height: 36px;
          display: flex;
          align-items: center;
          justify-content: center;
          border-radius: var(--border-radius-base);
          background: rgba(255, 255, 255, 0.5);

          &:hover {
            color: var(--primary-color);
            background: linear-gradient(135deg, var(--primary-light-8) 0%, var(--primary-light-9) 100%);
            transform: rotate(180deg);
          }
        }

        :deep(.el-breadcrumb) {
          font-size: 15px;

          .el-breadcrumb__item {
            .el-breadcrumb__inner {
              color: var(--text-regular);
              font-weight: normal;

              &.is-link {
                color: var(--primary-color);
                font-weight: 500;

                &:hover {
                  color: var(--primary-dark-2);
                }
              }
            }

            &:last-child {
              .el-breadcrumb__inner {
                color: var(--text-primary);
                font-weight: 600;
              }
            }
          }
        }
      }

      .right {
        display: flex;
        align-items: center;
        gap: 12px;

        .action-icon {
          font-size: 20px;
          padding: 8px;
          width: 36px;
          height: 36px;
          display: flex;
          align-items: center;
          justify-content: center;
          cursor: pointer;
          color: var(--text-regular);
          transition: all 0.3s;
          border-radius: var(--border-radius-base);
          background: rgba(255, 255, 255, 0.5);

          &:hover {
            color: var(--primary-color);
            background: linear-gradient(135deg, var(--primary-light-8) 0%, var(--primary-light-9) 100%);
          }
        }

        .avatar-container {
          display: flex;
          align-items: center;
          cursor: pointer;
          padding: 4px 10px;
          height: 36px;
          border-radius: var(--border-radius-base);
          transition: all 0.3s;
          background: rgba(255, 255, 255, 0.5);

          &:hover {
            background: linear-gradient(135deg, var(--primary-light-8) 0%, var(--primary-light-9) 100%);
          }

          .el-avatar {
            width: 28px;
            height: 28px;
          }

          .username {
            margin: 0 8px;
            color: var(--text-regular);
            font-weight: 500;
            font-size: 14px;
          }

          .el-icon {
            color: var(--text-regular);
            font-size: 16px;
            transition: transform 0.3s;
          }

          &:hover .el-icon {
            transform: rotate(180deg);
          }
        }
      }
    }

    .main {
      padding: 20px;
      background: transparent;
      overflow-y: auto;
      position: relative;
      z-index: 1;
      flex: 1;
    }

    .footer {
      display: flex;
      align-items: center;
      justify-content: center;
      background: linear-gradient(135deg, var(--primary-light-8) 0%, rgba(255, 255, 255, 0.95) 100%);
      backdrop-filter: blur(10px);
      color: var(--text-secondary);
      font-size: 14px;
      border-top: 1px solid rgba(255, 255, 255, 0.2);
      position: relative;
      z-index: 2;
      height: 40px;
      max-width: 800px;
      margin: 0 auto;
      width: 100%;
      border-radius: 12px 12px 0 0;
      box-shadow: 0 -4px 12px rgba(0, 0, 0, 0.05);

      .copyright {
        background: linear-gradient(to right, var(--primary-color), var(--primary-dark-2));
        -webkit-background-clip: text;
        color: transparent;
        font-weight: 500;
      }
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

:deep(.el-dropdown-menu) {
  padding: 4px 0;
  border-radius: var(--border-radius-base);
  border: 1px solid var(--border-light);
  box-shadow: var(--box-shadow-light);
  background-color: var(--bg-color);

  .el-dropdown-menu__item {
    padding: 8px 16px;
    font-size: 14px;
    color: var(--text-regular);
    cursor: pointer;
    transition: all 0.3s;

    &:hover {
      color: var(--primary-color);
      background-color: var(--primary-light-9);
    }

    .el-icon {
      margin-right: 8px;
      color: inherit;
    }
  }
}

@media screen and (max-width: 768px) {
  .layout-container {
    .aside {
      position: fixed;
      height: 100vh;
      transform: translateX(-100%);
      transition: transform 0.3s;

      &.is-expanded {
        transform: translateX(0);
      }
    }

    .main-container {
      margin-left: 0 !important;

      .header {
        .left {
          .collapse-btn {
            display: block;
          }
        }
      }
    }
  }
}
</style>