import router from './index'
import { useUserStore } from '@/store/user'
import { usePermissionStore } from '@/store/permission'
import { ElMessage } from 'element-plus'

const whiteList = ['/login', '/403'] // 白名单

router.beforeEach(async (to, from, next) => {
  const userStore = useUserStore()
  const permissionStore = usePermissionStore()

  // 获取token
  const hasToken = userStore.token

  // 没有token
  if (!hasToken) {
    if (whiteList.indexOf(to.path) !== -1) {
      // 在免登录白名单，直接进入
      next()
    } else {
      // 其他没有访问权限的页面将被重定向到登录页面
      next(`/login?redirect=${to.path}`)
    }
    return
  }

  // 已登录
  if (to.path === '/login') {
    // 已登录且要跳转的页面是登录页
    next({ path: '/' })
    return
  }

  // 判断当前用户是否已拉取完user_info信息
  const hasRoles = userStore.roles && userStore.roles.length > 0
  if (hasRoles) {
    // 判断是否有权限访问该页面
    if (to.path === '/' || to.path === '/dashboard' || !to.meta.permission || permissionStore.hasPermission(to.meta.permission)) {
      next()
    } else {
      ElMessage.error('没有权限访问该页面')
      next('/403')
    }
  } else {
    try {
      // 获取用户信息
      await userStore.getUserInfo()
      // 获取用户权限
      await permissionStore.generateRoutes()
      // hack方法 确保addRoutes已完成
      next({ ...to, replace: true })
    } catch (error) {
      // 移除token并跳转登录页面
      await userStore.resetToken()
      ElMessage.error(error.message || 'Has Error')
      next(`/login?redirect=${to.path}`)
    }
  }
}) 