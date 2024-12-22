import { createRouter, createWebHistory } from 'vue-router'
import DefaultLayout from '../layouts/DefaultLayout.vue'

const routes = [
  {
    path: '/login',
    name: 'Login',
    component: () => import('../views/Login.vue'),
    meta: {
      title: '登录'
    }
  },
  {
    path: '/403',
    name: '403',
    component: () => import('../views/403.vue'),
    meta: {
      title: '403'
    }
  },
  {
    path: '/',
    component: DefaultLayout,
    redirect: '/dashboard',
    children: [
      {
        path: 'dashboard',
        name: 'Dashboard',
        component: () => import('../views/Dashboard.vue'),
        meta: {
          title: '仪表盘',
          icon: 'Odometer'
        }
      },
      {
        path: '/menus',
        name: 'MenuList',
        component: () => import('../views/system/MenuList.vue'),
        meta: {
          title: '菜单管理',
          icon: 'Menu',
          permission: 'system:menu'
        }
      },
      {
        path: '/users',
        name: 'UserList',
        component: () => import('../views/system/UserList.vue'),
        meta: {
          title: '用户管理',
          icon: 'User',
          permission: 'system:user'
        }
      },
      {
        path: '/roles',
        name: 'RoleList',
        component: () => import('../views/system/RoleList.vue'),
        meta: {
          title: '角色管理',
          icon: 'UserFilled',
          permission: 'system:role'
        }
      },
      {
        path: '/permissions',
        name: 'PermissionList',
        component: () => import('../views/system/PermissionList.vue'),
        meta: {
          title: '权限管理',
          icon: 'Lock',
          permission: 'system:permission'
        }
      },
      {
        path: '/operation-logs',
        name: 'OperationLogList',
        component: () => import('../views/system/OperationLogList.vue'),
        meta: {
          title: '操作日志',
          icon: 'Document',
          permission: 'system:log'
        }
      },
      {
        path: '/profile',
        name: 'Profile',
        component: () => import('../views/user/Profile.vue'),
        meta: {
          title: '个人信息',
          icon: 'User',
          permission: 'system:profile'
        }
      }
    ]
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router 