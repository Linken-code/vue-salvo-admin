<template>
  <div class="permission-list">
    <div class="header">
      <el-form :inline="true" :model="searchForm" class="search-form">
        <el-form-item label="权限名称">
          <el-input v-model="searchForm.name" placeholder="请输入权限名称" clearable />
        </el-form-item>
        <el-form-item label="权限代码">
          <el-input v-model="searchForm.code" placeholder="请输入权限代码" clearable />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="handleSearch">搜索</el-button>
          <el-button @click="handleReset">重置</el-button>
        </el-form-item>
      </el-form>
      <el-button type="primary" @click="handleAdd">添加权限</el-button>
    </div>

    <el-table :data="permissions" style="width: 100%">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="name" label="权限名称" width="180" />
      <el-table-column prop="code" label="权限代码" width="180" />
      <el-table-column prop="description" label="描述" />
      <el-table-column prop="created_at" label="创建时间" width="180">
        <template #default="{ row }">
          {{ formatDateTime(row.created_at) }}
        </template>
      </el-table-column>
      <el-table-column label="操作" width="180">
        <template #default="{ row }">
          <el-button type="primary" link @click="handleEdit(row)">编辑</el-button>
          <el-button type="danger" link @click="handleDelete(row)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>

    <!-- 分页器 -->
    <div class="pagination">
      <el-pagination v-model:current-page="currentPage" v-model:page-size="pageSize" :page-sizes="[10, 20, 50, 100]"
        :total="total" layout="total, sizes, prev, pager, next" @size-change="handleSizeChange"
        @current-change="handlePageChange" />
    </div>

    <!-- 权限表单对话框 -->
    <el-dialog :title="dialogTitle" v-model="dialogVisible" width="500px" @close="resetForm">
      <el-form ref="formRef" :model="form" :rules="rules" label-width="80px" style="max-width: 460px">
        <el-form-item label="权限名称" prop="name">
          <el-input v-model="form.name" />
        </el-form-item>
        <el-form-item label="权限代码" prop="code">
          <el-input v-model="form.code" />
        </el-form-item>
        <el-form-item label="描述" prop="description">
          <el-input v-model="form.description" type="textarea" :rows="3" />
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
import { formatDateTime } from '../../utils/format'

const permissions = ref([])
const dialogVisible = ref(false)
const dialogTitle = ref('')
const formRef = ref()

// 分页相关
const total = ref(0)
const currentPage = ref(1)
const pageSize = ref(10)

const form = ref({
  name: '',
  code: '',
  description: ''
})

const rules = {
  name: [
    { required: true, message: '请输入权限名称', trigger: 'blur' }
  ],
  code: [
    { required: true, message: '请输入权限代码', trigger: 'blur' }
  ]
}

// 搜索表单
const searchForm = ref({
  name: '',
  code: ''
})

const fetchPermissions = async () => {
  try {
    const response = await axios.get('http://localhost:3000/permissions', {
      params: {
        page: currentPage.value,
        page_size: pageSize.value,
        ...searchForm.value
      }
    })
    permissions.value = response.data.items
    total.value = response.data.total
  } catch (error) {
    ElMessage.error('获取权限列表失败')
  }
}

const handleAdd = () => {
  dialogTitle.value = '添加权限'
  dialogVisible.value = true
  form.value = {
    name: '',
    code: '',
    description: ''
  }
}

const handleEdit = (row) => {
  dialogTitle.value = '编辑权限'
  dialogVisible.value = true
  form.value = { ...row }
}

const handleDelete = (row) => {
  ElMessageBox.confirm('确认删除该权限吗？', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(async () => {
    try {
      await axios.delete(`http://localhost:3000/permissions/${row.id}`)
      ElMessage.success('删除成功')
      fetchPermissions()
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
    description: ''
  }
}

const handleSubmit = async () => {
  if (!formRef.value) return

  await formRef.value.validate(async (valid) => {
    if (valid) {
      try {
        if (form.value.id) {
          await axios.put(`http://localhost:3000/permissions/${form.value.id}`, form.value)
          ElMessage.success('更新成功')
        } else {
          await axios.post('http://localhost:3000/permissions', form.value)
          ElMessage.success('添加成功')
        }
        dialogVisible.value = false
        fetchPermissions()
      } catch (error) {
        ElMessage.error(form.value.id ? '更新失败' : '添加失败')
      }
    }
  })
}

const handlePageChange = (page) => {
  currentPage.value = page
  fetchPermissions()
}

const handleSizeChange = (size) => {
  pageSize.value = size
  currentPage.value = 1
  fetchPermissions()
}

const handleSearch = () => {
  currentPage.value = 1
  fetchPermissions()
}

const handleReset = () => {
  searchForm.value = {
    name: '',
    code: ''
  }
  currentPage.value = 1
  fetchPermissions()
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
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}

.search-form {
  flex: 1;
  margin-right: 16px;
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