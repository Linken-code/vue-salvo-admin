<template>
    <div class="password-container">
        <el-card class="password-card">
            <template #header>
                <div class="card-header">
                    <span>修改密码</span>
                    <el-button type="primary" @click="handleUpdatePassword" :loading="loading">更新密码</el-button>
                </div>
            </template>

            <el-form :model="form" label-width="100px" class="password-form" :rules="rules" ref="formRef">
                <el-form-item label="旧密码" prop="oldPassword">
                    <el-input v-model="form.oldPassword" type="password" show-password />
                </el-form-item>

                <el-form-item label="新密码" prop="newPassword">
                    <el-input v-model="form.newPassword" type="password" show-password />
                </el-form-item>

                <el-form-item label="确认密码" prop="confirmPassword">
                    <el-input v-model="form.confirmPassword" type="password" show-password />
                </el-form-item>
            </el-form>
        </el-card>
    </div>
</template>

<script setup>
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import request from '../../utils/request'

const formRef = ref(null)
const loading = ref(false)

const form = ref({
    oldPassword: '',
    newPassword: '',
    confirmPassword: ''
})

const validateConfirmPassword = (rule, value, callback) => {
    if (value !== form.value.newPassword) {
        callback(new Error('两次输入的密码不一致'))
    } else {
        callback()
    }
}

const rules = {
    oldPassword: [
        { required: true, message: '请输入旧密码', trigger: 'blur' },
        { min: 6, message: '密码长度不能小于6位', trigger: 'blur' }
    ],
    newPassword: [
        { required: true, message: '请输入新密码', trigger: 'blur' },
        { min: 6, message: '密码长度不能小于6位', trigger: 'blur' }
    ],
    confirmPassword: [
        { required: true, message: '请确认新密码', trigger: 'blur' },
        { validator: validateConfirmPassword, trigger: 'blur' }
    ]
}

const handleUpdatePassword = async () => {
    if (!formRef.value) return

    try {
        await formRef.value.validate()
        loading.value = true
        await request.patch('/profile/password', {
            old_password: form.value.oldPassword,
            new_password: form.value.newPassword
        })
        ElMessage.success('密码更新成功')
        form.value = {
            oldPassword: '',
            newPassword: '',
            confirmPassword: ''
        }
    } catch (error) {
        if (error.response?.data?.message) {
            ElMessage.error(error.response.data.message)
        } else if (error.message) {
            ElMessage.error(error.message)
        } else {
            ElMessage.error('密码更新失败')
        }
    } finally {
        loading.value = false
    }
}
</script>

<style scoped>
.password-container {
    padding: 20px;
}

.password-card {
    max-width: 600px;
    margin: 0 auto;
}

.card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.password-form {
    max-width: 400px;
    margin: 0 auto;
}
</style>