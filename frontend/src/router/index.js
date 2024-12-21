import { createRouter, createWebHistory } from 'vue-router'
import DefaultLayout from '../layouts/DefaultLayout.vue'
import Dashboard from '../views/Dashboard.vue'
import Login from '../views/Login.vue'
import MenuList from '../views/system/MenuList.vue'
import UserList from '../views/system/UserList.vue'
import RoleList from '../views/system/RoleList.vue'
import PermissionList from '../views/system/PermissionList.vue'
import Profile from '../views/user/Profile.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/login',
      name: 'Login',
      component: Login
    },
    {
      path: '/',
      component: DefaultLayout,
      children: [
        {
          path: '',
          name: 'Dashboard',
          component: Dashboard
        },
        {
          path: 'menus',
          name: 'MenuList',
          component: MenuList
        },
        {
          path: 'users',
          name: 'UserList',
          component: UserList
        },
        {
          path: 'roles',
          name: 'RoleList',
          component: RoleList
        },
        {
          path: 'permissions',
          name: 'PermissionList',
          component: PermissionList
        },
        {
          path: 'profile',
          name: 'Profile',
          component: Profile
        }
      ]
    }
  ]
})

router.beforeEach((to, from, next) => {
  const token = localStorage.getItem('token')
  if (to.path !== '/login' && !token) {
    next('/login')
  } else {
    next()
  }
})

export default router 