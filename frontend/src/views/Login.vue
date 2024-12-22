<template>
  <div class="login-container">
    <div class="login-box">
      <div class="login-left">
        <img src="../assets/logo.svg" alt="Logo" class="logo" />
        <h1>Jing Salvo Admin</h1>
        <p>一个现代化的后台管理系统</p>
      </div>
      <div class="login-right">
        <div class="login-form">
          <h2>登录</h2>
          <el-form ref="loginFormRef" :model="loginForm" :rules="loginRules" @keyup.enter="handleLogin">
            <el-form-item prop="username">
              <el-input v-model="loginForm.username" placeholder="用户名" prefix-icon="User">
                <template #prefix>
                </template>
              </el-input>
            </el-form-item>

            <el-form-item prop="password">
              <el-input v-model="loginForm.password" type="password" placeholder="密码" show-password prefix-icon="Lock">
                <template #prefix>
                </template>
              </el-input>
            </el-form-item>

            <el-form-item>
              <el-button type="primary" :loading="loading" class="login-button" @click="handleLogin">
                登录
              </el-button>
            </el-form-item>
          </el-form>
          <div class="login-tips">
            <p>默认用户名：admin</p>
            <p>默认密码：admin123</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useUserStore } from '@/store/user'
import { ElMessage } from 'element-plus'

const router = useRouter()
const route = useRoute()
const userStore = useUserStore()

const loginForm = reactive({
  username: 'admin',
  password: 'admin123'
})

const loginRules = {
  username: [{ required: true, trigger: 'blur', message: '请输入用户名' }],
  password: [{ required: true, trigger: 'blur', message: '请输入密码' }]
}

const loading = ref(false)
const loginFormRef = ref(null)

const handleLogin = async () => {
  if (!loginFormRef.value) return

  try {
    loading.value = true
    await loginFormRef.value.validate()
    await userStore.login(loginForm.username, loginForm.password)

    // 获取重定向地址或默认跳转到首页
    const redirect = route.query.redirect || '/'
    router.push(redirect)

    ElMessage.success('登录成功')
  } catch (error) {
    console.error('Login error:', error)
    ElMessage.error(error.message || '登录失败')
  } finally {
    loading.value = false
  }
}
</script>

<style lang="scss" scoped>
.login-container {
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--primary-light-7) 0%, var(--primary-color) 100%);
  padding: 20px;

  .login-box {
    width: 900px;
    height: 500px;
    border-radius: var(--border-radius-base);
    background-color: var(--bg-color);
    box-shadow: var(--box-shadow-light);
    display: flex;
    overflow: hidden;

    .login-left {
      flex: 1;
      background: linear-gradient(135deg, var(--primary-light-5) 0%, var(--primary-color) 100%);
      padding: 40px;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      color: white;
      text-align: center;
      position: relative;
      overflow: hidden;

      &::before {
        content: '';
        position: absolute;
        top: -50%;
        left: -50%;
        width: 200%;
        height: 200%;
        background: radial-gradient(circle, rgba(255, 255, 255, 0.1) 0%, transparent 60%);
        animation: rotate 30s linear infinite;
      }

      .logo {
        width: 120px;
        height: 120px;
        margin-bottom: 20px;
        filter: drop-shadow(0 4px 6px rgba(0, 0, 0, 0.1));
        animation: float 6s ease-in-out infinite;
      }

      h1 {
        font-size: 28px;
        margin-bottom: 16px;
        font-weight: 600;
        text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
      }

      p {
        font-size: 16px;
        opacity: 0.9;
        text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
      }
    }

    .login-right {
      flex: 1;
      padding: 40px;
      display: flex;
      align-items: center;
      background-color: var(--bg-color);

      .login-form {
        width: 100%;
        max-width: 340px;
        margin: 0 auto;

        h2 {
          font-size: 24px;
          color: var(--text-primary);
          margin-bottom: 30px;
          text-align: center;
          font-weight: 600;
        }

        :deep(.el-input) {
          --el-input-bg-color: var(--bg-color);
          --el-input-border-color: var(--border-color);
          --el-input-hover-border-color: var(--primary-light-3);
          --el-input-focus-border-color: var(--primary-color);

          .el-input__wrapper {
            padding: 1px 11px;
            background-color: var(--bg-color);
            border: 1px solid var(--border-color);
            box-shadow: none;
            transition: all 0.3s;

            &:hover {
              border-color: var(--primary-light-3);
            }

            &.is-focus {
              border-color: var(--primary-color);
              box-shadow: 0 0 0 2px var(--primary-light-8);
            }
          }

          .el-input__inner {
            height: 40px;
            color: var(--text-primary);
            font-size: 14px;

            &::placeholder {
              color: var(--text-placeholder);
            }
          }

          .el-input__prefix {
            .el-icon {
              font-size: 16px;
              color: var(--text-secondary);
            }
          }
        }

        :deep(.el-form-item) {
          margin-bottom: 24px;

          &.is-error {
            .el-input__wrapper {
              border-color: var(--danger-color);
            }
          }

          .el-form-item__error {
            color: var(--danger-color);
            font-size: 12px;
            padding-top: 4px;
          }
        }

        .login-button {
          width: 100%;
          height: 40px;
          font-size: 16px;
          font-weight: 500;
          background: linear-gradient(to right, var(--primary-color), var(--primary-dark-2));
          border: none;
          transition: all 0.3s;

          &:hover {
            transform: translateY(-1px);
            box-shadow: 0 4px 12px rgba(64, 158, 255, 0.4);
          }

          &:active {
            transform: translateY(0);
          }
        }

        .login-tips {
          margin-top: 20px;
          text-align: center;
          color: var(--text-secondary);
          font-size: 14px;

          p {
            margin: 4px 0;
          }
        }
      }
    }
  }
}

@keyframes rotate {
  from {
    transform: rotate(0deg);
  }

  to {
    transform: rotate(360deg);
  }
}

@keyframes float {

  0%,
  100% {
    transform: translateY(0);
  }

  50% {
    transform: translateY(-10px);
  }
}

@media screen and (max-width: 768px) {
  .login-container {
    padding: 0;

    .login-box {
      width: 100%;
      height: 100%;
      border-radius: 0;

      .login-left {
        display: none;
      }

      .login-right {
        padding: 20px;

        .login-form {
          padding: 0;
        }
      }
    }
  }
}
</style>