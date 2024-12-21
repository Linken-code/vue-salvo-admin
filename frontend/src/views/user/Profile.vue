<script setup>
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { useRouter } from 'vue-router'
import axios from 'axios'

const router = useRouter()
const userInfo = ref(null)
const loading = ref(false)
const formRef = ref(null)
const passwordFormRef = ref(null)
const passwordDialogVisible = ref(false)

const formData = ref({
  nickname: '',
  email: '',
  avatar: ''
})

const passwordForm = ref({
  old_password: '',
  new_password: '',
  confirm_password: ''
})

const rules = {
  nickname: [{ required: true, message: '请输入昵称', trigger: 'blur' }],
  email: [{ type: 'email', message: '请输入正确的邮箱地址', trigger: 'blur' }]
}

const passwordRules = {
  old_password: [{ required: true, message: '请输入原密码', trigger: 'blur' }],
  new_password: [{ required: true, message: '请输入新密码', trigger: 'blur' }],
  confirm_password: [
    { required: true, message: '请确认新密码', trigger: 'blur' },
    {
      validator: (rule, value, callback) => {
        if (value !== passwordForm.value.new_password) {
          callback(new Error('两次输入的密码不一致'))
        } else {
          callback()
        }
      },
      trigger: 'blur'
    }
  ]
}

// 获取用户信息
const fetchUserInfo = async () => {
  const token = localStorage.getItem('token')
  if (!token) {
    ElMessage.error('用户未登录')
    router.push('/login')
    return
  }

  try {
    // 先获取当前用户信息
    const currentUserResponse = await axios.get('http://localhost:3000/auth/current-user', {
      headers: {
        Authorization: `Bearer ${token}`
      }
    })
    userInfo.value = currentUserResponse.data

    // 获取用户详细信息
    const response = await axios.get(`http://localhost:3000/users/${userInfo.value.id}`, {
      headers: {
        Authorization: `Bearer ${token}`
      }
    })
    const data = response.data
    formData.value = {
      nickname: data.nickname,
      email: data.email || '',
      avatar: data.avatar || ''
    }
  } catch (error) {
    ElMessage.error('获取用户信息失败')
    console.error('Error:', error)
    router.push('/login')
  }
}

// 保存个人信息
const handleSave = async () => {
  if (!formRef.value || !userInfo.value) return

  await formRef.value.validate(async (valid) => {
    if (valid) {
      loading.value = true
      try {
        const token = localStorage.getItem('token')
        const response = await axios.put(
          `http://localhost:3000/users/${userInfo.value.id}`,
          {
            ...formData.value,
            status: 1
          },
          {
            headers: {
              Authorization: `Bearer ${token}`
            }
          }
        )

        if (response.status === 200) {
          // 更新本地存储的用户信息
          const newUserInfo = {
            ...userInfo.value,
            ...formData.value
          }
          localStorage.setItem('userInfo', JSON.stringify(newUserInfo))
          ElMessage.success('保存成功')
        } else {
          throw new Error('保存失败')
        }
      } catch (error) {
        ElMessage.error('保存失败')
        console.error('Error:', error)
      } finally {
        loading.value = false
      }
    }
  })
}

// 修改密码
const handleUpdatePassword = async () => {
  if (!passwordFormRef.value || !userInfo.value) return

  await passwordFormRef.value.validate(async (valid) => {
    if (valid) {
      loading.value = true
      try {
        const token = localStorage.getItem('token')
        const response = await axios.post(
          `http://localhost:3000/users/${userInfo.value.id}/password`,
          {
            old_password: passwordForm.value.old_password,
            new_password: passwordForm.value.new_password
          },
          {
            headers: {
              Authorization: `Bearer ${token}`
            }
          }
        )

        if (response.status === 200) {
          ElMessage.success('密码修改成功')
          passwordDialogVisible.value = false
          // 清空表单
          passwordForm.value = {
            old_password: '',
            new_password: '',
            confirm_password: ''
          }
        } else {
          throw new Error('原密码错误')
        }
      } catch (error) {
        ElMessage.error('原密码错误')
        console.error('Error:', error)
      } finally {
        loading.value = false
      }
    }
  })
}

onMounted(() => {
  fetchUserInfo()
})
</script>

<template>
  <div class="profile">
    <div class="page-header">
      <h2>个人信息</h2>
    </div>

    <el-card shadow="never">
      <el-form ref="formRef" :model="formData" :rules="rules" label-width="100px" class="profile-form">
        <el-form-item label="用户名">
          <el-input :model-value="userInfo?.username" disabled />
        </el-form-item>
        <el-form-item label="昵称" prop="nickname">
          <el-input v-model="formData.nickname" placeholder="请输入昵称" />
        </el-form-item>
        <el-form-item label="邮箱" prop="email">
          <el-input v-model="formData.email" placeholder="请输入邮箱" />
        </el-form-item>
        <el-form-item label="头像" prop="avatar">
          <el-input v-model="formData.avatar" placeholder="请输入头像地址" />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" :loading="loading" @click="handleSave">
            保存
          </el-button>
          <el-button @click="passwordDialogVisible = true">修改密码</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 修改密码对话框 -->
    <el-dialog v-model="passwordDialogVisible" title="修改密码" width="500px" destroy-on-close>
      <el-form ref="passwordFormRef" :model="passwordForm" :rules="passwordRules" label-width="100px">
        <el-form-item label="原密码" prop="old_password">
          <el-input v-model="passwordForm.old_password" type="password" placeholder="请输入原密码" show-password />
        </el-form-item>
        <el-form-item label="新密码" prop="new_password">
          <el-input v-model="passwordForm.new_password" type="password" placeholder="请输入新密码" show-password />
        </el-form-item>
        <el-form-item label="确认密码" prop="confirm_password">
          <el-input v-model="passwordForm.confirm_password" type="password" placeholder="请确认新密码" show-password />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="passwordDialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="loading" @click="handleUpdatePassword">
          确定
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped>
.profile {
  padding: 20px;
}

.page-header {
  margin-bottom: 24px;
}

.page-header h2 {
  margin: 0;
  font-size: 24px;
  color: var(--text-primary);
}

:deep(.el-card) {
  border: none;
}

.profile-form {
  max-width: 600px;
}
</style>