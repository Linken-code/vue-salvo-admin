import { createRouter, createWebHistory } from 'vue-router'
import DefaultLayout from '../layouts/DefaultLayout.vue'
import Dashboard from '../views/Dashboard.vue'
import Login from '../views/Login.vue'
import MenuList from '../views/system/MenuList.vue'
import UserList from '../views/system/UserList.vue'
import RoleList from '../views/system/RoleList.vue'
import PermissionList from '../views/system/PermissionList.vue'
import Profile from '../views/user/Profile.vue'
import Password from '../views/user/Password.vue'

const routes = [
  {
    path: '/login',
    name: 'Login',
    component: Login,
    meta: { title: '登录' }
  },
  {
    path: '/',
    component: DefaultLayout,
    redirect: '/dashboard',
    children: [
      {
        path: 'dashboard',
        name: 'Dashboard',
        component: Dashboard,
        meta: { title: '仪表盘', requiresAuth: true }
      },
      {
        path: 'users',
        name: 'UserList',
        component: UserList,
        meta: { title: '用户管理', requiresAuth: true }
      },
      {
        path: 'roles',
        name: 'RoleList',
        component: RoleList,
        meta: { title: '角色管理', requiresAuth: true }
      },
      {
        path: 'permissions',
        name: 'PermissionList',
        component: PermissionList,
        meta: { title: '权限管理', requiresAuth: true }
      },
      {
        path: 'menus',
        name: 'MenuList',
        component: MenuList,
        meta: { title: '菜单管理', requiresAuth: true }
      },
      {
        path: 'profile',
        name: 'Profile',
        component: Profile,
        meta: { title: '个人信息', requiresAuth: true, hidden: true }
      },
      {
        path: 'profile/password',
        name: 'Password',
        component: Password,
        meta: { title: '修改密码', requiresAuth: true, hidden: true }
      }
    ]
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

router.beforeEach((to, from, next) => {
  document.title = to.meta.title ? `${to.meta.title} - Vue Salvo Admin` : 'Vue Salvo Admin'

  if (to.matched.some(record => record.meta.requiresAuth)) {
    const token = localStorage.getItem('token')
    if (!token) {
      next({
        path: '/login',
        query: { redirect: to.fullPath }
      })
    } else {
      next()
    }
  } else {
    if (to.path === '/login' && localStorage.getItem('token')) {
      next('/')
    } else {
      next()
    }
  }
})

export default router 