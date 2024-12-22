<script setup>
import { ref, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import request from '../../utils/request'
import { formatDateTime } from '../../utils/format'
import { User, Plus, Edit, Delete, UserFilled } from '@element-plus/icons-vue'

const users = ref([])
const dialogVisible = ref(false)
const roleDialogVisible = ref(false)
const dialogTitle = ref('')
const formRef = ref()
const currentUserId = ref(null)

// 分页相关
const total = ref(0)
const currentPage = ref(1)
const pageSize = ref(10)

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
  ],
  email: [
    { type: 'email', message: '请输入正确的邮箱地址', trigger: 'blur' }
  ]
}

// 角色相关
const allRoles = ref([])
const selectedRoles = ref([])

// 搜索表单
const searchForm = ref({
  username: '',
  nickname: '',
  email: '',
  status: ''
})

const fetchUsers = async () => {
  try {
    const response = await request.get('/users', {
      params: {
        page: currentPage.value,
        page_size: pageSize.value,
        ...searchForm.value
      }
    })

    // 获取每个用户的角色信息
    const usersWithRoles = await Promise.all(
      response.items.map(async (user) => {
        try {
          const roleResponse = await request.get(`/users/${user.id}/roles`)
          return {
            ...user,
            roles: roleResponse.roles || []
          }
        } catch (error) {
          console.error(`获取用户 ${user.id} 的角色失败:`, error)
          return {
            ...user,
            roles: []
          }
        }
      })
    )

    users.value = usersWithRoles
    total.value = response.total
  } catch (error) {
    ElMessage.error('获取用户列表失败')
  }
}

const fetchRoles = async () => {
  try {
    const response = await request.get('/roles', {
      params: {
        page: 1,
        page_size: 1000 // 获取所有角色
      }
    })
    allRoles.value = response.items.map(role => ({
      key: role.id,
      label: role.name
    }))
  } catch (error) {
    ElMessage.error('获取角色列表失败')
  }
}

const fetchUserRoles = async (userId) => {
  try {
    const response = await request.get(`/users/${userId}/roles`)
    selectedRoles.value = response.roles.map(role => role.id)
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
  // 禁止删除admin账号
  if (row.username === 'admin') {
    ElMessage.warning('系统管理员账号不能删除')
    return
  }

  // 获取当前登录用户信息
  const currentUser = JSON.parse(localStorage.getItem('user') || '{}')

  ElMessageBox.confirm('确认删除该用户吗？', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(async () => {
    try {
      await request.delete(`/users/${row.id}`)
      ElMessage.success('删除成功')
      fetchUsers()

      // 如果删除的是当前登录用户，则自动登出
      if (currentUser.id === row.id) {
        localStorage.removeItem('token')
        localStorage.removeItem('user')
        window.location.href = '/login'
      }
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
          await request.put(`/users/${form.value.id}`, form.value)
          ElMessage.success('���新成功')
        } else {
          await request.post('/users', form.value)
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
    await request.put(`/users/${currentUserId.value}/roles`, {
      role_ids: selectedRoles.value
    })
    ElMessage.success('角色分配成功')
    roleDialogVisible.value = false
  } catch (error) {
    ElMessage.error('角色分配失败')
  }
}

const handlePageChange = (page) => {
  currentPage.value = page
  fetchUsers()
}

const handleSizeChange = (size) => {
  pageSize.value = size
  currentPage.value = 1
  fetchUsers()
}

const handleSearch = () => {
  currentPage.value = 1
  fetchUsers()
}

const handleReset = () => {
  searchForm.value = {
    username: '',
    nickname: '',
    email: '',
    status: ''
  }
  currentPage.value = 1
  fetchUsers()
}

const filterMethod = (query, item) => {
  return item.label.indexOf(query) > -1;
};

// 角色颜色映射
const roleColorMap = {
  'super_admin': ['#9C27B0', '#BA68C8'], // 超级管理员：紫色
  'admin': ['#1976D2', '#42A5F5'], // 管理员：蓝色
  'user': ['#388E3C', '#66BB6A'], // 普通用户：绿色
  'guest': ['#FFA000', '#FFB74D'], // 访客：橙色
  'operator': ['#0097A7', '#26C6DA'], // 操作员：青色
  'viewer': ['#5D4037', '#8D6E63'], // 查看者：棕色
  'editor': ['#C2185B', '#EC407A'], // 编辑者：粉色
  'default': ['#757575', '#BDBDBD'] // 默认：灰色
}

// 获取角色的渐变色
const getRoleGradient = (roleCode) => {
  const colors = roleColorMap[roleCode] || roleColorMap.default
  return `linear-gradient(135deg, ${colors[0]} 0%, ${colors[1]} 100%)`
}

onMounted(() => {
  fetchUsers()
})
</script>

<template>
  <div class="page-container">
    <div class="page-header">
      <div class="left">
        <h2 class="page-title">
          <el-icon>
            <User />
          </el-icon>
          用户管理
        </h2>
        <div class="page-subtitle">管理系统的用户账号和权限</div>
      </div>
      <el-button type="primary" class="btn-add" @click="handleAdd">
        <el-icon>
          <Plus />
        </el-icon>新增用户
      </el-button>
    </div>

    <div class="content-wrapper">
      <div class="search-wrapper">
        <el-form :inline="true" :model="searchForm" class="search-form">
          <el-form-item label="用户名">
            <el-input v-model="searchForm.username" placeholder="请输入用户名" clearable />
          </el-form-item>
          <el-form-item label="昵称">
            <el-input v-model="searchForm.nickname" placeholder="请输入昵称" clearable />
          </el-form-item>
          <el-form-item label="邮箱">
            <el-input v-model="searchForm.email" placeholder="请输入邮箱" clearable />
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
      </div>

      <div class="table-wrapper">
        <el-table :data="users" class="menu-table" :highlight-current-row="true" :border="false">
          <el-table-column prop="id" label="ID" width="80">
            <template #default="{ row }">
              <span class="text-tag">{{ row.id }}</span>
            </template>
          </el-table-column>
          <el-table-column prop="username" label="用户名" width="120">
            <template #default="{ row }">
              <span class="text-tag">{{ row.username }}</span>
            </template>
          </el-table-column>
          <el-table-column prop="nickname" label="昵称" width="120">
            <template #default="{ row }">
              <span class="text-tag">{{ row.nickname }}</span>
            </template>
          </el-table-column>
          <el-table-column prop="email" label="邮箱" min-width="180">
            <template #default="{ row }">
              <span class="text-tag">{{ row.email }}</span>
            </template>
          </el-table-column>
          <el-table-column prop="roles" label="角色" min-width="200">
            <template #default="{ row }">
              <div class="role-tags">
                <template v-if="row.roles && row.roles.length > 0">
                  <span v-for="role in row.roles" :key="role.id" class="role-tag"
                    :style="{ background: getRoleGradient(role.code) }">
                    {{ role.name }}
                  </span>
                </template>
                <span v-else class="role-tag no-role">无角色</span>
              </div>
            </template>
          </el-table-column>
          <el-table-column prop="status" label="状态" width="80">
            <template #default="{ row }">
              <span class="status-tag" :class="{ 'success': row.status === 1, 'error': row.status === 0 }">
                {{ row.status === 1 ? '启用' : '禁用' }}
              </span>
            </template>
          </el-table-column>
          <el-table-column prop="created_at" label="创建时间" width="180">
            <template #default="{ row }">
              <span class="text-tag">{{ formatDateTime(row.created_at) }}</span>
            </template>
          </el-table-column>
          <el-table-column label="操作" width="280" fixed="right">
            <template #default="{ row }">
              <div class="action-buttons">
                <span class="operation-tag update" @click="handleEdit(row)">
                  <el-icon>
                    <Edit />
                  </el-icon>编辑
                </span>
                <span class="operation-tag role" @click="handleRoles(row)">
                  <el-icon>
                    <UserFilled />
                  </el-icon>角色
                </span>
                <span class="operation-tag delete" @click="handleDelete(row)">
                  <el-icon>
                    <Delete />
                  </el-icon>删除
                </span>
              </div>
            </template>
          </el-table-column>
        </el-table>
      </div>

      <div class="pagination-wrapper">
        <el-pagination v-model:current-page="currentPage" v-model:page-size="pageSize" :page-sizes="[10, 20, 50, 100]"
          :total="total" layout="total, sizes, prev, pager, next" @size-change="handleSizeChange"
          @current-change="handlePageChange" />
      </div>
    </div>

    <!-- 户表单对话框 -->
    <el-dialog :title="dialogTitle" v-model="dialogVisible" width="500px" @close="resetForm">
      <el-form ref="formRef" :model="form" :rules="rules" label-width="80px" style="max-width: 460px">
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
          <el-switch v-model="form.status" :active-value="1" :inactive-value="0" />
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
    <el-dialog title="分配角色" v-model="roleDialogVisible" width="800px">
      <div class="transfer-container">
        <el-transfer v-model="selectedRoles" :data="allRoles" :titles="['可选角色', '已选角色']" filterable
          :filter-method="filterMethod" filter-placeholder="请输入角色名称" />
      </div>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="roleDialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handleRoleSubmit">确定</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<style lang="scss" scoped>
.page-container {
  background-color: var(--bg-color);
  border-radius: var(--border-radius-base);
  box-shadow: var(--box-shadow-light);
  padding: 24px;
  min-height: calc(100vh - 120px);
  position: relative;
  overflow: hidden;

  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 200px;
    background: linear-gradient(135deg, var(--primary-light-7) 0%, var(--primary-color) 100%);
    opacity: 0.1;
    z-index: 0;
  }
}

.page-header {
  position: relative;
  z-index: 1;
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
  padding: 16px;
  background: linear-gradient(135deg, var(--primary-light-5) 0%, var(--primary-color) 100%);
  border-radius: var(--border-radius-base);
  box-shadow: var(--box-shadow-light);

  .left {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .page-title {
    font-size: 20px;
    font-weight: 600;
    color: white;
    display: flex;
    align-items: center;
    gap: 8px;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);

    .el-icon {
      font-size: 24px;
      color: white;
      filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
    }
  }

  .page-subtitle {
    font-size: 14px;
    color: rgba(255, 255, 255, 0.9);
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
  }

  .btn-add {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 8px 16px;
    font-weight: 500;
    background: white;
    color: var(--primary-color);
    border: none;
    transition: all 0.3s;

    &:hover {
      transform: translateY(-1px);
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
    }

    &:active {
      transform: translateY(0);
    }
  }
}

.content-wrapper {
  position: relative;
  z-index: 1;
  background-color: var(--bg-color);
  border-radius: var(--border-radius-base);
  box-shadow: var(--box-shadow-light);
  padding: 20px;
  overflow-x: auto;

  .search-wrapper {
    margin-bottom: 20px;
    padding: 16px;
    background: linear-gradient(135deg, var(--primary-light-9) 0%, rgba(255, 255, 255, 0.95) 100%);
    border-radius: var(--border-radius-base);
    backdrop-filter: blur(10px);

    .search-form {
      display: flex;
      flex-wrap: wrap;
      gap: 16px;

      .el-form-item {
        margin-bottom: 0;
        margin-right: 0;
      }
    }
  }

  .table-wrapper {
    overflow-x: auto;

    .menu-table {
      width: 100%;
      margin-bottom: 20px;
      min-width: 900px;

      :deep(.el-table__row) {
        transition: all 0.3s;

        &:hover {
          background-color: var(--primary-light-9);
        }
      }

      :deep(.el-table__header) {
        th {
          background-color: var(--primary-light-9);
          color: var(--primary-color);
          font-weight: 600;
          text-align: center !important;

          .cell {
            justify-content: center;
          }
        }
      }

      :deep(.el-table__cell) {
        .cell {
          display: flex;
          align-items: center;
          justify-content: center;
          min-height: 40px;
        }
      }

      .text-tag {
        display: inline-block;
        padding: 2px 8px;
        border-radius: var(--border-radius-base);
        text-align: center;
        font-weight: 500;
        font-size: 13px;
        color: var(--primary-color);
        border: 1px solid var(--primary-color);
        background-color: rgba(64, 158, 255, 0.05);
      }

      .role-tags {
        display: flex;
        gap: 4px;
        flex-wrap: wrap;
        justify-content: center;
      }

      .role-tag {
        display: inline-block;
        padding: 2px 8px;
        border-radius: var(--border-radius-base);
        text-align: center;
        font-weight: 500;
        font-size: 13px;
        color: white;
        white-space: nowrap;

        &.no-role {
          background: linear-gradient(135deg, #909399 0%, #A6A9AD 100%);
        }
      }

      .status-tag {
        display: inline-block;
        padding: 2px 8px;
        border-radius: var(--border-radius-base);
        text-align: center;
        font-weight: 500;
        font-size: 13px;
        color: white;

        &.success {
          background: linear-gradient(135deg, #67C23A 0%, #85CE61 100%);
        }

        &.error {
          background: linear-gradient(135deg, #F56C6C 0%, #F78989 100%);
        }
      }

      .operation-tag {
        display: inline-flex;
        align-items: center;
        gap: 4px;
        padding: 2px 8px;
        border-radius: var(--border-radius-base);
        text-align: center;
        font-weight: 500;
        font-size: 13px;
        color: white;
        cursor: pointer;
        transition: all 0.3s;

        &.update {
          background: linear-gradient(135deg, #409EFF 0%, #53A8FF 100%);
        }

        &.role {
          background: linear-gradient(135deg, #9C27B0 0%, #BA68C8 100%);
        }

        &.delete {
          background: linear-gradient(135deg, #F56C6C 0%, #F78989 100%);
        }

        &:hover {
          transform: translateY(-1px);
          box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
        }

        &:active {
          transform: translateY(0);
        }

        .el-icon {
          font-size: 14px;
        }
      }
    }

    .action-buttons {
      display: flex;
      gap: 8px;
      flex-wrap: nowrap;
      justify-content: flex-start;

      .path-tag,
      .component-tag {
        cursor: pointer;
        display: inline-flex;
        align-items: center;
        gap: 4px;
        position: relative;
        overflow: hidden;
        white-space: nowrap;
        flex-shrink: 0;

        .el-icon {
          margin-right: 4px;
          font-size: 14px;
        }

        &::before {
          content: '';
          position: absolute;
          top: 0;
          left: 0;
          width: 100%;
          height: 100%;
          background: linear-gradient(135deg, var(--primary-light-7) 0%, var(--primary-color) 100%);
          opacity: 0;
          transition: opacity 0.3s;
          z-index: -1;
        }

        &:hover {
          color: white;
          background-color: transparent;

          &::before {
            opacity: 1;
          }
        }
      }

      .component-tag.empty:hover::before {
        background: linear-gradient(135deg, #f56c6c 0%, #e41749 100%);
      }
    }
  }

  .pagination-wrapper {
    margin-top: 20px;
    display: flex;
    justify-content: flex-end;
  }
}

:deep(.el-dialog) {
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);

  .el-dialog__header {
    margin: 0;
    padding: 20px 24px;
    background: linear-gradient(135deg, var(--primary-light-5) 0%, var(--primary-color) 100%);
    border-bottom: none;

    .el-dialog__title {
      color: white;
      font-weight: 600;
      font-size: 18px;
    }

    .el-dialog__close {
      color: white;
      font-size: 18px;

      &:hover {
        color: white;
        transform: rotate(90deg);
      }
    }
  }

  .el-dialog__body {
    padding: 24px;
  }

  .el-dialog__footer {
    padding: 16px 24px;
    margin: 0;
    border-top: 1px solid var(--border-light);

    .el-button--primary {
      background: linear-gradient(to right, var(--primary-color), var(--primary-dark-2));
      border: none;
      padding: 8px 20px;

      &:hover {
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(64, 158, 255, 0.4);
      }

      &:active {
        transform: translateY(0);
      }
    }
  }
}

@media screen and (max-width: 768px) {
  .page-container {
    padding: 16px;
  }

  .page-header {
    flex-direction: column;
    gap: 16px;
    align-items: flex-start;

    .btn-add {
      width: 100%;
      justify-content: center;
    }
  }

  .content-wrapper {
    padding: 12px;
    margin: 0 -16px;
    border-radius: 0;

    .search-wrapper {
      padding: 12px;

      .search-form {
        .el-form-item {
          width: 100%;

          .el-input,
          .el-select {
            width: 100%;
          }
        }
      }
    }

    .table-wrapper {
      padding: 12px;
      margin: 0 -16px;
      border-radius: 0;

      .menu-table {
        min-width: 800px;
      }

      .action-buttons {
        flex-wrap: nowrap;

        .el-button {
          padding: 4px 6px;
          font-size: 13px;

          .el-icon {
            font-size: 14px;
          }
        }
      }
    }
  }
}

.transfer-container {
  padding: 20px 0;

  :deep(.el-transfer) {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 50px;

    .el-transfer-panel {
      width: 300px;

      &__body {
        height: 400px;
      }

      .el-transfer-panel__list {
        height: 350px;
      }
    }

    .el-transfer__buttons {
      display: flex;
      flex-direction: column;
      padding: 0;
      gap: 12px;

      .el-button {
        padding: 8px;
        width: 40px;
        height: 40px;
        border-radius: var(--border-radius-base);
        display: flex;
        align-items: center;
        justify-content: center;

        .el-icon {
          font-size: 20px;
        }
      }
    }
  }
}
</style>