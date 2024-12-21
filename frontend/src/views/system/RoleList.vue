<template>
  <div class="role-list">
    <div class="header">
      <el-form :inline="true" :model="searchForm" class="search-form">
        <el-form-item label="角色名称">
          <el-input v-model="searchForm.name" placeholder="请输入角色名称" clearable />
        </el-form-item>
        <el-form-item label="角色代码">
          <el-input v-model="searchForm.code" placeholder="请输入角色代码" clearable />
        </el-form-item>
        <el-form-item label="状态" class="status-item">
          <el-select v-model="searchForm.status" placeholder="请选择状态" clearable style="width: 120px;">
            <el-option label="启用" :value="1" />
            <el-option label="禁用" :value="0" />
          </el-select>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="handleSearch">搜索</el-button>
          <el-button @click="handleReset">重置</el-button>
        </el-form-item>
      </el-form>
      <el-button type="primary" @click="handleAdd">添加角色</el-button>
    </div>

    <el-table :data="roles" style="width: 100%">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="name" label="角色名称" width="180" />
      <el-table-column prop="code" label="角色代码" width="180" />
      <el-table-column prop="description" label="描述" />
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
          <el-button type="primary" link @click="handlePermissions(row)">权限</el-button>
          <el-button type="danger" link @click="handleDelete(row)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>

    <!-- 分页器 -->
    <div class="pagination">
      <el-pagination
        v-model:current-page="currentPage"
        v-model:page-size="pageSize"
        :page-sizes="[10, 20, 50, 100]"
        :total="total"
        layout="total, sizes, prev, pager, next"
        @size-change="handleSizeChange"
        @current-change="handlePageChange"
      />
    </div>

    <!-- 角色表单对话框 -->
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
        <el-form-item label="角色名称" prop="name">
          <el-input v-model="form.name" />
        </el-form-item>
        <el-form-item label="角色代码" prop="code">
          <el-input v-model="form.code" />
        </el-form-item>
        <el-form-item label="描述" prop="description">
          <el-input v-model="form.description" type="textarea" :rows="3" />
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

    <!-- 权限分配对话框 -->
    <el-dialog
      title="分配权限"
      v-model="permissionDialogVisible"
      width="600px"
    >
      <el-transfer
        v-model="selectedPermissions"
        :data="allPermissions"
        :titles="['可选权限', '已选权限']"
      />
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
const dialogVisible = ref(false)
const permissionDialogVisible = ref(false)
const dialogTitle = ref('')
const formRef = ref()
const currentRoleId = ref(null)

// 分页相关
const total = ref(0)
const currentPage = ref(1)
const pageSize = ref(10)

const form = ref({
  name: '',
  code: '',
  description: '',
  status: 1
})

const rules = {
  name: [
    { required: true, message: '请输入角色名称', trigger: 'blur' }
  ],
  code: [
    { required: true, message: '请输入角色代码', trigger: 'blur' }
  ]
}

// 权限相关
const allPermissions = ref([])
const selectedPermissions = ref([])

// 搜索表单
const searchForm = ref({
  name: '',
  code: '',
  status: ''
})

const fetchRoles = async () => {
  try {
    const response = await axios.get('http://localhost:3000/roles', {
      params: {
        page: currentPage.value,
        page_size: pageSize.value,
        ...searchForm.value
      }
    })
    roles.value = response.data.items
    total.value = response.data.total
  } catch (error) {
    ElMessage.error('获取角色列表失败')
  }
}

const fetchPermissions = async () => {
  try {
    const response = await axios.get('http://localhost:3000/permissions')
    allPermissions.value = response.data.map(permission => ({
      key: permission.id,
      label: permission.name
    }))
  } catch (error) {
    ElMessage.error('获取权限列表失败')
  }
}

const fetchRolePermissions = async (roleId) => {
  try {
    const response = await axios.get(`http://localhost:3000/roles/${roleId}/permissions`)
    selectedPermissions.value = response.data.map(permission => permission.id)
  } catch (error) {
    ElMessage.error('获取角色权限失败')
  }
}

const handleAdd = () => {
  dialogTitle.value = '添加角色'
  dialogVisible.value = true
  form.value = {
    name: '',
    code: '',
    description: '',
    status: 1
  }
}

const handleEdit = (row) => {
  dialogTitle.value = '编辑角色'
  dialogVisible.value = true
  form.value = { ...row }
}

const handlePermissions = async (row) => {
  currentRoleId.value = row.id
  permissionDialogVisible.value = true
  await fetchPermissions()
  await fetchRolePermissions(row.id)
}

const handleDelete = (row) => {
  ElMessageBox.confirm('确认删除该角色吗？', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(async () => {
    try {
      await axios.delete(`http://localhost:3000/roles/${row.id}`)
      ElMessage.success('删除成功')
      fetchRoles()
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
    name: '',
    code: '',
    description: '',
    status: 1
  }
}

const handleSubmit = async () => {
  if (!formRef.value) return
  
  await formRef.value.validate(async (valid) => {
    if (valid) {
      try {
        if (form.value.id) {
          await axios.put(`http://localhost:3000/roles/${form.value.id}`, form.value)
          ElMessage.success('更新成功')
        } else {
          await axios.post('http://localhost:3000/roles', form.value)
          ElMessage.success('添加成功')
        }
        dialogVisible.value = false
        fetchRoles()
      } catch (error) {
        ElMessage.error(form.value.id ? '更新失败' : '添加失败')
      }
    }
  })
}

const handlePermissionSubmit = async () => {
  try {
    await axios.put(`http://localhost:3000/roles/${currentRoleId.value}/permissions`, {
      permission_ids: selectedPermissions.value
    })
    ElMessage.success('权限分配成功')
    permissionDialogVisible.value = false
  } catch (error) {
    ElMessage.error('权限分配失败')
  }
}

const handlePageChange = (page) => {
  currentPage.value = page
  fetchRoles()
}

const handleSizeChange = (size) => {
  pageSize.value = size
  currentPage.value = 1
  fetchRoles()
}

const handleSearch = () => {
  currentPage.value = 1
  fetchRoles()
}

const handleReset = () => {
  searchForm.value = {
    name: '',
    code: '',
    status: ''
  }
  currentPage.value = 1
  fetchRoles()
}

onMounted(() => {
  fetchRoles()
})
</script>

<style scoped>
.role-list {
  padding: 20px;
}

.header {
  margin-bottom: 20px;
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}

.search-form {
  flex: 1;
  margin-right: 16px;
}

.search-form :deep(.el-form-item) {
  margin-bottom: 16px;
  margin-right: 16px;
}

.search-form :deep(.el-input),
.search-form :deep(.el-select) {
  width: 180px;
}

.pagination {
  margin-top: 20px;
  display: flex;
  justify-content: flex-end;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}
</style>