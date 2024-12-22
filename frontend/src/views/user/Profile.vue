<template>
  <div class="profile-container">
    <el-card class="profile-card">
      <template #header>
        <div class="card-header">
          <span>个人信息</span>
          <el-button type="primary" @click="handleSave" :loading="loading">保存修改</el-button>
        </div>
      </template>

      <el-form :model="form" label-width="100px" class="profile-form">
        <el-form-item label="头像">
          <el-upload class="avatar-uploader" action="http://localhost:3000/upload"
            :headers="{ Authorization: `Bearer ${token}` }" name="file" :show-file-list="false"
            :on-success="handleAvatarSuccess" :on-error="handleAvatarError" :before-upload="beforeAvatarUpload">
            <el-avatar v-if="form.avatar" :size="100" :src="getAvatarUrl(form.avatar)" />
            <el-avatar v-else :size="100">{{ form.nickname?.charAt(0) }}</el-avatar>
            <div class="upload-tip">点击上传</div>
          </el-upload>
        </el-form-item>

        <el-form-item label="用户名">
          <el-input v-model="form.username" disabled />
        </el-form-item>

        <el-form-item label="昵称">
          <el-input v-model="form.nickname" />
        </el-form-item>

        <el-form-item label="邮箱">
          <el-input v-model="form.email" />
        </el-form-item>
      </el-form>
    </el-card>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import request, { handleRequestError } from '../../utils/request'
import { updateUserInfo } from '../../utils/user'

const form = ref({
  username: '',
  nickname: '',
  email: '',
  avatar: ''
})

const loading = ref(false)
const token = localStorage.getItem('token')

// 获取用户信息
const fetchUserInfo = async () => {
  try {
    const response = await request.get('/auth/current-user')
    const { user } = response
    form.value = {
      username: user.username,
      nickname: user.nickname,
      email: user.email,
      avatar: user.avatar
    }
  } catch (error) {
    handleRequestError(error)
  }
}

// 保存个人信息
const handleSave = async () => {
  try {
    loading.value = true
    const data = {
      nickname: form.value.nickname,
      email: form.value.email
    }
    if (form.value.avatar) {
      data.avatar = form.value.avatar
    }
    const response = await request.patch('/profile', data)
    if (response.user) {
      ElMessage.success('保存成功')
      updateUserInfo(response.user)
    } else {
      throw new Error('更新失败：服务器返回数据格式不正确')
    }
  } catch (error) {
    handleRequestError(error)
  } finally {
    loading.value = false
  }
}

// 头像上传前的验证
const beforeAvatarUpload = (file) => {
  const isImage = file.type.startsWith('image/')
  const isLt2M = file.size / 1024 / 1024 < 2

  if (!isImage) {
    ElMessage.error('只能上传图片文件!')
    return false
  }
  if (!isLt2M) {
    ElMessage.error('图片大小不能超过 2MB!')
    return false
  }
  return true
}

// 获取头像完整URL
const getAvatarUrl = (avatar) => {
  if (!avatar) return ''
  return avatar.startsWith('http') ? avatar : `http://localhost:3000${avatar}`
}

// 头像上传成功的回调
const handleAvatarSuccess = async (response) => {
  try {
    // 先更新本地表单的头像
    form.value.avatar = response.url
    // 构造更新请求数据，只包含头像字段
    const data = {
      avatar: response.url
    }
    const result = await request.patch('/profile', data)

    if (result.user) {
      ElMessage.success('头像更新成功')
      updateUserInfo(result.user)
    } else {
      throw new Error('服务器返回数据格式不正确')
    }
  } catch (error) {
    handleRequestError(error)
    // 如果更新失败，恢复原来的头像
    form.value.avatar = form.value.avatar || ''
  }
}

// 头像上传失败的回调
const handleAvatarError = (error) => {
  handleRequestError(error)
}

onMounted(() => {
  fetchUserInfo()
})
</script>

<style scoped>
.profile-container {
  padding: 20px;
}

.profile-card {
  max-width: 600px;
  margin: 0 auto;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.profile-form {
  max-width: 400px;
  margin: 0 auto;
}

.avatar-uploader {
  text-align: center;
  cursor: pointer;
}

.upload-tip {
  margin-top: 8px;
  color: var(--el-text-color-secondary);
  font-size: 14px;
}

.el-avatar {
  display: block;
  margin: 0 auto;
}
</style>