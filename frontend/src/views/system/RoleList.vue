<template>
  <div class="role-list">
    <div class="header">
      <el-button type="primary" @click="showCreateDialog">新建角色</el-button>
    </div>

    <el-table :data="roles" style="width: 100%" v-loading="loading">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="name" label="角色名称" width="180" />
      <el-table-column prop="code" label="角色编码" width="180" />
      <el-table-column prop="description" label="描述" />
      <el-table-column prop="status" label="状态" width="100">
        <template #default="{ row }">
          <el-tag :type="row.status === 1 ? 'success' : 'info'">
            {{ row.status === 1 ? '启用' : '禁用' }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column label="操作" width="250">
        <template #default="{ row }">
          <el-button type="primary" size="small" @click="showPermissionDialog(row)">权限设置</el-button>
          <el-button type="primary" size="small" @click="showEditDialog(row)">编辑</el-button>
          <el-button type="danger" size="small" @click="handleDelete(row)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>

    <!-- 创建/编辑角色对话框 -->
    <el-dialog :title="dialogType === 'create' ? '新建角色' : '编辑角色'" v-model="dialogVisible" width="500px">
      <el-form :model="roleForm" :rules="rules" ref="roleFormRef" label-width="100px">
        <el-form-item label="角色名称" prop="name">
          <el-input v-model="roleForm.name" />
        </el-form-item>
        <el-form-item label="角色编码" prop="code">
          <el-input v-model="roleForm.code" />
        </el-form-item>
        <el-form-item label="描述" prop="description">
          <el-input v-model="roleForm.description" type="textarea" />
        </el-form-item>
        <el-form-item label="状态" prop="status">
          <el-switch v-model="roleForm.status" :active-value="1" :inactive-value="0" active-text="启用"
            inactive-text="禁用" />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="dialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handleSubmit">确定</el-button>
        </span>
      </template>
    </el-dialog>

    <!-- 权限设置对话框 -->
    <el-dialog title="权限设置" v-model="permissionDialogVisible" width="500px">
      <el-tree ref="permissionTree" :data="permissions" show-checkbox node-key="id" :props="{ label: 'name' }"
        :default-checked-keys="selectedPermissions" />
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="permissionDialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handlePermissionSubmit">确定</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import axios from 'axios'

const roles = ref([])
const permissions = ref([])
const loading = ref(false)
const dialogVisible = ref(false)
const permissionDialogVisible = ref(false)
const dialogType = ref('create')
const selectedPermissions = ref([])
const currentRole = ref(null)
const permissionTree = ref(null)
const roleFormRef = ref(null)

const roleForm = ref({
  name: '',
  code: '',
  description: '',
  status: 1
})

const rules = {
  name: [{ required: true, message: '请输入角色名称', trigger: 'blur' }],
  code: [{ required: true, message: '请输入角色编码', trigger: 'blur' }]
}

// 获取角色列表
const fetchRoles = async () => {
  loading.value = true
  try {
    const response = await axios.get('http://localhost:3000/roles')
    roles.value = response.data
  } catch (error) {
    ElMessage.error('获取角色列表失败')
  } finally {
    loading.value = false
  }
}

// 获取权限列表
const fetchPermissions = async () => {
  try {
    const response = await axios.get('http://localhost:3000/permissions')
    permissions.value = response.data
  } catch (error) {
    ElMessage.error('获取权限列表失败')
  }
}

// 获取角色的权限
const fetchRolePermissions = async (roleId) => {
  try {
    const response = await axios.get(`http://localhost:3000/roles/${roleId}/permissions`)
    selectedPermissions.value = response.data
  } catch (error) {
    ElMessage.error('获取角色权限失败')
  }
}

// 显示创建对话框
const showCreateDialog = () => {
  dialogType.value = 'create'
  roleForm.value = {
    name: '',
    code: '',
    description: '',
    status: 1
  }
  dialogVisible.value = true
}

// 显示编辑对话框
const showEditDialog = (row) => {
  dialogType.value = 'edit'
  roleForm.value = { ...row }
  dialogVisible.value = true
}

// 显示权限设置对话框
const showPermissionDialog = async (row) => {
  currentRole.value = row
  await fetchRolePermissions(row.id)
  permissionDialogVisible.value = true
}

// 提交角色表单
const handleSubmit = async () => {
  if (!roleFormRef.value) return

  await roleFormRef.value.validate(async (valid) => {
    if (valid) {
      try {
        if (dialogType.value === 'create') {
          await axios.post('http://localhost:3000/roles', roleForm.value)
          ElMessage.success('创建成功')
        } else {
          await axios.put(`http://localhost:3000/roles/${roleForm.value.id}`, roleForm.value)
          ElMessage.success('更新成功')
        }
        dialogVisible.value = false
        fetchRoles()
      } catch (error) {
        ElMessage.error(dialogType.value === 'create' ? '创建失败' : '更新失败')
      }
    }
  })
}

// 删除角色
const handleDelete = (row) => {
  ElMessageBox.confirm('确认删除该角色吗？', '提示', {
    type: 'warning'
  })
    .then(async () => {
      try {
        await axios.delete(`http://localhost:3000/roles/${row.id}`)
        ElMessage.success('删除成功')
        fetchRoles()
      } catch (error) {
        ElMessage.error('删除失败')
      }
    })
    .catch(() => { })
}

// 提交权限设置
const handlePermissionSubmit = async () => {
  if (!currentRole.value || !permissionTree.value) return

  try {
    const checkedKeys = permissionTree.value.getCheckedKeys()
    await axios.put(`http://localhost:3000/roles/${currentRole.value.id}/permissions`, {
      role_id: currentRole.value.id,
      permission_ids: checkedKeys
    })
    ElMessage.success('权限设置成功')
    permissionDialogVisible.value = false
  } catch (error) {
    ElMessage.error('权限设置失败')
  }
}

onMounted(() => {
  fetchRoles()
  fetchPermissions()
})
</script>

<style scoped>
.role-list {
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