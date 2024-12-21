<template>
  <div class="login-container">
    <div class="login-box">
      <div class="login-left">
        <img src="../assets/logo.svg" alt="Logo" class="logo" />
        <h1>Vue Salvo Admin</h1>
        <p>一个现代化的后台管理系统</p>
      </div>
      <div class="login-right">
        <div class="login-form">
          <h2>登录</h2>
          <el-form ref="loginForm" :model="form" :rules="rules" @keyup.enter="handleLogin">
            <el-form-item prop="username">
              <el-input v-model="form.username" placeholder="用户名" prefix-icon="User">
                <template #prefix>
                  <el-icon>
                    <User />
                  </el-icon>
                </template>
              </el-input>
            </el-form-item>
            <el-form-item prop="password">
              <el-input v-model="form.password" type="password" placeholder="密码" show-password>
                <template #prefix>
                  <el-icon>
                    <Lock />
                  </el-icon>
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
import { ref } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { ElMessage } from 'element-plus';
import { User, Lock } from '@element-plus/icons-vue';
import request from '../utils/request';

const router = useRouter();
const route = useRoute();
const loginForm = ref(null);
const loading = ref(false);

const form = ref({
  username: 'admin',
  password: 'admin123'
});

const rules = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' }
  ]
};

const handleLogin = async () => {
  if (!loginForm.value) return;

  try {
    await loginForm.value.validate();
    loading.value = true;

    const response = await request.post('/auth/login', {
      username: form.value.username,
      password: form.value.password,
    });

    if (response && response.token) {
      localStorage.setItem('token', response.token);
      if (response.user) {
        localStorage.setItem('user', JSON.stringify(response.user));
      }

      const redirect = route.query.redirect || '/';
      router.push(redirect);

      ElMessage.success('登录成功');
    } else {
      ElMessage.error('登录失败：无效的响应数据');
    }
  } catch (error) {
    console.error('Login failed:', error);
    ElMessage.error(error.response?.data?.message || '登录失败，请检查用户名和密码');
  } finally {
    loading.value = false;
  }
};
</script>

<style lang="scss" scoped>
.login-container {
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--el-color-primary-light-7) 0%, var(--el-color-primary) 100%);

  .login-box {
    width: 900px;
    height: 500px;
    border-radius: 12px;
    background-color: var(--el-bg-color);
    box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
    display: flex;
    overflow: hidden;

    .login-left {
      flex: 1;
      background: linear-gradient(135deg, var(--el-color-primary-light-5) 0%, var(--el-color-primary) 100%);
      padding: 40px;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      color: white;
      text-align: center;

      .logo {
        width: 120px;
        height: 120px;
        margin-bottom: 20px;
      }

      h1 {
        font-size: 28px;
        margin-bottom: 16px;
      }

      p {
        font-size: 16px;
        opacity: 0.8;
      }
    }

    .login-right {
      flex: 1;
      padding: 40px;
      display: flex;
      align-items: center;

      .login-form {
        width: 100%;
        max-width: 340px;
        margin: 0 auto;

        h2 {
          font-size: 24px;
          color: var(--el-text-color-primary);
          margin-bottom: 30px;
          text-align: center;
        }

        .login-button {
          width: 100%;
          padding: 12px 0;
        }

        .login-tips {
          margin-top: 20px;
          text-align: center;
          color: var(--el-text-color-secondary);
          font-size: 14px;

          p {
            margin: 4px 0;
          }
        }
      }
    }
  }
}

@media screen and (max-width: 768px) {
  .login-container {
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