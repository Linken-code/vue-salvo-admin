<script setup>
import { ref, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import axios from 'axios'

const users = ref([])
const dialogVisible = ref(false)
const roleDialogVisible = ref(false)
const dialogTitle = ref('')
const formRef = ref()
const currentUserId = ref(null)

const form = ref({
  username: '',
  nickname: '',
  email: '',
  password: '',
  status: 1
})

const rules = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 3, max: 20, message: '长度在 3 到 20 个字符', trigger: 'blur' }
  ],
  nickname: [
    { required: true, message: '请输入昵称', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, message: '密码不能少于 6 个字符', trigger: 'blur' }
  ]
}

// 角色相关
const allRoles = ref([])
const selectedRoles = ref([])

const fetchUsers = async () => {
  try {
    const response = await axios.get('http://localhost:3000/users')
    users.value = response.data
  } catch (error) {
    ElMessage.error('获取用户列表失败')
  }
}

const fetchRoles = async () => {
  try {
    const response = await axios.get('http://localhost:3000/roles')
    allRoles.value = response.data.map(role => ({
      key: role.id,
      label: role.name
    }))
  } catch (error) {
    ElMessage.error('获取角色列表失败')
  }
}

const fetchUserRoles = async (userId) => {
  try {
    const response = await axios.get(`http://localhost:3000/users/${userId}/roles`)
    selectedRoles.value = response.data.map(role => role.id)
  } catch (error) {
    ElMessage.error('获取用户角色失败')
  }
}

const handleAdd = () => {
  dialogTitle.value = '添加用户'
  dialogVisible.value = true
  form.value = {
    username: '',
    nickname: '',
    email: '',
    password: '',
    status: 1
  }
}

const handleEdit = (row) => {
  dialogTitle.value = '编辑用户'
  dialogVisible.value = true
  form.value = { ...row }
}

const handleRoles = async (row) => {
  currentUserId.value = row.id
  roleDialogVisible.value = true
  await fetchRoles()
  await fetchUserRoles(row.id)
}

const handleDelete = (row) => {
  ElMessageBox.confirm('确认删除该用户吗？', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(async () => {
    try {
      await axios.delete(`http://localhost:3000/users/${row.id}`)
      ElMessage.success('删除成功')
      fetchUsers()
    } catch (error) {
      ElMessage.error('删除失败')
    }
  })
}

const resetForm = () => {
  if (formRef.value) {
    formRef.value.resetFields()
  }
  form.value = {
    username: '',
    nickname: '',
    email: '',
    password: '',
    status: 1
  }
}

const handleSubmit = async () => {
  if (!formRef.value) return
  
  await formRef.value.validate(async (valid) => {
    if (valid) {
      try {
        if (form.value.id) {
          await axios.put(`http://localhost:3000/users/${form.value.id}`, form.value)
          ElMessage.success('更新成功')
        } else {
          await axios.post('http://localhost:3000/users', form.value)
          ElMessage.success('添加成功')
        }
        dialogVisible.value = false
        fetchUsers()
      } catch (error) {
        ElMessage.error(form.value.id ? '更新失败' : '添加失败')
      }
    }
  })
}

const handleRoleSubmit = async () => {
  try {
    await axios.put(`http://localhost:3000/users/${currentUserId.value}/roles`, {
      role_ids: selectedRoles.value
    })
    ElMessage.success('角色分配成功')
    roleDialogVisible.value = false
  } catch (error) {
    ElMessage.error('角色分配失败')
  }
}

onMounted(() => {
  fetchUsers()
})
</script>

<template>
  <div class="user-list">
    <div class="header">
      <el-button type="primary" @click="handleAdd">添加用户</el-button>
    </div>

    <el-table :data="users" style="width: 100%">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="username" label="用户名" width="120" />
      <el-table-column prop="nickname" label="昵称" width="120" />
      <el-table-column prop="email" label="邮箱" width="180" />
      <el-table-column prop="status" label="状态" width="100">
        <template #default="{ row }">
          <el-tag :type="row.status === 1 ? 'success' : 'danger'">
            {{ row.status === 1 ? '启用' : '禁用' }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column prop="created_at" label="创建时间" width="180" />
      <el-table-column label="操作">
        <template #default="{ row }">
          <el-button type="primary" link @click="handleEdit(row)">编辑</el-button>
          <el-button type="primary" link @click="handleRoles(row)">角色</el-button>
          <el-button type="danger" link @click="handleDelete(row)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>

    <!-- 用户表单对话框 -->
    <el-dialog
      :title="dialogTitle"
      v-model="dialogVisible"
      width="500px"
      @close="resetForm"
    >
      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-width="80px"
        style="max-width: 460px"
      >
        <el-form-item label="用户名" prop="username">
          <el-input v-model="form.username" />
        </el-form-item>
        <el-form-item label="昵称" prop="nickname">
          <el-input v-model="form.nickname" />
        </el-form-item>
        <el-form-item label="邮箱" prop="email">
          <el-input v-model="form.email" />
        </el-form-item>
        <el-form-item label="密码" prop="password" v-if="!form.id">
          <el-input v-model="form.password" type="password" />
        </el-form-item>
        <el-form-item label="状态" prop="status">
          <el-switch
            v-model="form.status"
            :active-value="1"
            :inactive-value="0"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="dialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handleSubmit">确定</el-button>
        </span>
      </template>
    </el-dialog>

    <!-- 角色分配对话框 -->
    <el-dialog
      title="分配角色"
      v-model="roleDialogVisible"
      width="600px"
    >
      <el-transfer
        v-model="selectedRoles"
        :data="allRoles"
        :titles="['可选角色', '已选角色']"
      />
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="roleDialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handleRoleSubmit">确定</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped>
.user-list {
  padding: 20px;
}

.header {
  margin-bottom: 20px;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}
</style> 