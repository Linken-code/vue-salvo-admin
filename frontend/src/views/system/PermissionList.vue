<template>
  <div class="permission-list">
    <div class="header">
      <el-button type="primary" @click="showCreateDialog">新建权限</el-button>
    </div>

    <el-table :data="permissions" style="width: 100%" v-loading="loading">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="name" label="权限名称" width="180" />
      <el-table-column prop="code" label="权限编码" width="180" />
      <el-table-column prop="description" label="描述" />
      <el-table-column label="操作" width="180">
        <template #default="{ row }">
          <el-button type="primary" size="small" @click="showEditDialog(row)">编辑</el-button>
          <el-button type="danger" size="small" @click="handleDelete(row)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>

    <!-- 创建/编辑权限对话框 -->
    <el-dialog :title="dialogType === 'create' ? '新建权限' : '编辑权限'" v-model="dialogVisible" width="500px">
      <el-form :model="permissionForm" :rules="rules" ref="permissionFormRef" label-width="100px">
        <el-form-item label="权限名称" prop="name">
          <el-input v-model="permissionForm.name" />
        </el-form-item>
        <el-form-item label="权限编码" prop="code">
          <el-input v-model="permissionForm.code" />
        </el-form-item>
        <el-form-item label="描述" prop="description">
          <el-input v-model="permissionForm.description" type="textarea" />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="dialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handleSubmit">确定</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import axios from 'axios'

const permissions = ref([])
const loading = ref(false)
const dialogVisible = ref(false)
const dialogType = ref('create')
const permissionFormRef = ref(null)

const permissionForm = ref({
  name: '',
  code: '',
  description: ''
})

const rules = {
  name: [{ required: true, message: '请输入权限名称', trigger: 'blur' }],
  code: [{ required: true, message: '请输入权限编码', trigger: 'blur' }]
}

// 获取权限列表
const fetchPermissions = async () => {
  loading.value = true
  try {
    const response = await axios.get('http://localhost:3000/permissions')
    permissions.value = response.data
  } catch (error) {
    ElMessage.error('获取权限列表失败')
  } finally {
    loading.value = false
  }
}

// 显示创建对话框
const showCreateDialog = () => {
  dialogType.value = 'create'
  permissionForm.value = {
    name: '',
    code: '',
    description: ''
  }
  dialogVisible.value = true
}

// 显示编辑对话框
const showEditDialog = (row) => {
  dialogType.value = 'edit'
  permissionForm.value = { ...row }
  dialogVisible.value = true
}

// 提交权限表单
const handleSubmit = async () => {
  if (!permissionFormRef.value) return

  await permissionFormRef.value.validate(async (valid) => {
    if (valid) {
      try {
        if (dialogType.value === 'create') {
          await axios.post('http://localhost:3000/permissions', permissionForm.value)
          ElMessage.success('创建成功')
        } else {
          await axios.put(`http://localhost:3000/permissions/${permissionForm.value.id}`, permissionForm.value)
          ElMessage.success('更新成功')
        }
        dialogVisible.value = false
        fetchPermissions()
      } catch (error) {
        ElMessage.error(dialogType.value === 'create' ? '创建失败' : '更新失败')
      }
    }
  })
}

// 删除权限
const handleDelete = (row) => {
  ElMessageBox.confirm('确认删除该权限吗？', '提示', {
    type: 'warning'
  })
    .then(async () => {
      try {
        await axios.delete(`http://localhost:3000/permissions/${row.id}`)
        ElMessage.success('删除成功')
        fetchPermissions()
      } catch (error) {
        ElMessage.error('删除失败')
      }
    })
    .catch(() => { })
}

onMounted(() => {
  fetchPermissions()
})
</script>

<style scoped>
.permission-list {
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