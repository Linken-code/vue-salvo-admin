<template>
  <div class="page-container">
    <div class="page-header">
      <div class="left">
        <div class="page-title">
          <el-icon>
            <UserFilled />
          </el-icon>
          角色管理
        </div>
        <div class="page-subtitle">管理系统角色及其权限</div>
      </div>
      <el-button class="btn-add" type="primary" @click="handleAdd">
        <el-icon>
          <Plus />
        </el-icon>
        添加角色
      </el-button>
    </div>

    <div class="content-wrapper">
      <div class="search-wrapper">
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
      </div>

      <div class="table-wrapper">
        <el-table :data="roles" class="menu-table" :highlight-current-row="true" :border="false">
          <el-table-column prop="id" label="ID" width="80">
            <template #default="{ row }">
              <span class="text-tag">{{ row.id }}</span>
            </template>
          </el-table-column>
          <el-table-column prop="name" label="角色名称" width="140">
            <template #default="{ row }">
              <span class="role-tag" :style="{ background: getRoleGradient(row) }">
                {{ row.name }}
              </span>
            </template>
          </el-table-column>
          <el-table-column prop="code" label="角色代码" width="140">
            <template #default="{ row }">
              <span class="text-tag">{{ row.code }}</span>
            </template>
          </el-table-column>
          <el-table-column prop="description" label="描述" min-width="200">
            <template #default="{ row }">
              <span class="text-tag">{{ row.description }}</span>
            </template>
          </el-table-column>
          <el-table-column prop="permissions" label="权限" min-width="300">
            <template #default="{ row }">
              <div class="permission-tags">
                <template v-if="row.permissions && row.permissions.length > 0">
                  <span v-for="permission in row.permissions" :key="permission.id" class="permission-tag"
                    :style="{ background: getPermissionGradient(permission) }">
                    {{ permission.name }}
                  </span>
                </template>
                <span v-else class="permission-tag no-permission">无权限</span>
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
                <span class="operation-tag permission" @click="handlePermissions(row)">
                  <el-icon>
                    <Key />
                  </el-icon>权限
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

    <!-- 角色表单对话框 -->
    <el-dialog :title="dialogTitle" v-model="dialogVisible" width="500px" @close="resetForm">
      <el-form ref="formRef" :model="form" :rules="rules" label-width="80px" style="max-width: 460px">
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
          <el-switch v-model="form.status" :active-value="1" :inactive-value="0" />
        </el-form-item>
        <el-form-item label="角色颜色">
          <div class="color-picker-wrapper">
            <div class="color-inputs">
              <el-color-picker v-model="form.color_start" show-alpha />
              <el-icon>
                <ArrowRight />
              </el-icon>
              <el-color-picker v-model="form.color_end" show-alpha />
            </div>
            <div class="color-preview" :style="{ background: getRoleGradient(form) }">
              预览效果
            </div>
            <div class="preset-colors">
              <div class="preset-title">预设方案：</div>
              <div class="preset-list">
                <div v-for="preset in presetColors" :key="preset.name" class="preset-item"
                  :style="{ background: `linear-gradient(135deg, ${preset.start} 0%, ${preset.end} 100%)` }"
                  @click="applyPresetColor(preset)">
                  {{ preset.name }}
                </div>
              </div>
            </div>
          </div>
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
    <el-dialog v-model="permissionDialogVisible" title="分配权限" width="50%" destroy-on-close>
      <permission-tree v-if="currentRole" ref="permissionTreeRef" :role-id="currentRole.id"
        @change="handlePermissionChange" />
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="permissionDialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handleSavePermissions">
            确定
          </el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import request from '../../utils/request'
import { formatDateTime } from '../../utils/format'
import { UserFilled, Plus, Edit, Delete, Key, ArrowRight } from '@element-plus/icons-vue'
import PermissionTree from '@/components/PermissionTree.vue'

const roles = ref([])
const dialogVisible = ref(false)
const permissionDialogVisible = ref(false)
const dialogTitle = ref('')
const formRef = ref()
const currentRole = ref(null)
const currentRoleId = ref(null)

// 分页相关
const total = ref(0)
const currentPage = ref(1)
const pageSize = ref(10)

const form = ref({
  name: '',
  code: '',
  description: '',
  status: 1,
  color_start: '', // 渐变开始颜色
  color_end: '' // 渐变结束颜色
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
const permissionTreeRef = ref()
const selectedPermissions = ref([])

// 搜索表单
const searchForm = ref({
  name: '',
  code: '',
  status: ''
})

// 角色色映射
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

// 预设的颜色方案
const presetColors = [
  { name: '紫色主题', start: '#9C27B0', end: '#BA68C8' },
  { name: '蓝色主题', start: '#1976D2', end: '#42A5F5' },
  { name: '绿色主题', start: '#388E3C', end: '#66BB6A' },
  { name: '橙色主题', start: '#FFA000', end: '#FFB74D' },
  { name: '青色主题', start: '#0097A7', end: '#26C6DA' },
  { name: '棕色主题', start: '#5D4037', end: '#8D6E63' },
  { name: '粉色主题', start: '#C2185B', end: '#EC407A' },
  { name: '灰色主题', start: '#757575', end: '#BDBDBD' }
]

// 应用预设颜色
const applyPresetColor = (preset) => {
  form.value.color_start = preset.start
  form.value.color_end = preset.end
}

// 获取角色的渐变色
const getRoleGradient = (role) => {
  if (role.color_start && role.color_end) {
    return `linear-gradient(135deg, ${role.color_start} 0%, ${role.color_end} 100%)`
  }
  // 如果没有设置颜色，使用默认颜色
  const colors = roleColorMap[role.code] || roleColorMap.default
  return `linear-gradient(135deg, ${colors[0]} 0%, ${colors[1]} 100%)`
}

// 获取权限的渐变色
const getPermissionGradient = (permission) => {
  if (permission.color_start && permission.color_end) {
    return `linear-gradient(135deg, ${permission.color_start} 0%, ${permission.color_end} 100%)`;
  }
  // 如果没有设置颜色，使用默认颜色
  return `linear-gradient(135deg, #757575 0%, #BDBDBD 100%)`;
};

const fetchRoles = async () => {
  try {
    const response = await request.get('/roles', {
      params: {
        page: currentPage.value,
        page_size: pageSize.value,
        ...searchForm.value
      }
    });

    roles.value = response.items;
    total.value = response.total;
  } catch (error) {
    ElMessage.error('获取角色列表失败');
  }
};

const fetchPermissions = async () => {
  try {
    const response = await request.get('/permissions', {
      params: {
        page: 1,
        page_size: 1000 // 获取所有权限
      }
    })
    allPermissions.value = response.items.map(permission => ({
      key: permission.id,
      label: permission.name,
      description: permission.description
    }))
  } catch (error) {
    ElMessage.error('获取权限列表失败')
  }
}

const fetchRolePermissions = async (roleId) => {
  try {
    const response = await request.get(`/roles/${roleId}/permissions`)
    selectedPermissions.value = response.permissions || []
  } catch (error) {
    console.error('获取角色权限失败:', error)
    ElMessage.error('获取角色权限失败')
    selectedPermissions.value = []
  }
}

const handleAdd = () => {
  dialogTitle.value = '添加角色'
  dialogVisible.value = true
  form.value = {
    name: '',
    code: '',
    description: '',
    status: 1,
    color_start: '', // 渐变开始颜色
    color_end: '' // 渐变结束颜色
  }
}

const handleEdit = (row) => {
  dialogTitle.value = '编辑角色'
  dialogVisible.value = true
  form.value = { ...row }
}

const handlePermissions = (row) => {
  currentRole.value = row
  permissionDialogVisible.value = true
}

const handleDelete = (row) => {
  // 禁止删除超级管理员角色
  if (row.code === 'super_admin') {
    ElMessage.warning('超级管理员角色不能删除')
    return
  }

  ElMessageBox.confirm('确认删除该角色吗？', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(async () => {
    try {
      await request.delete(`/roles/${row.id}`)
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
    status: 1,
    color_start: '', // 渐变开始颜色
    color_end: '' // 渐变结束颜色
  }
}

const handleSubmit = async () => {
  if (!formRef.value) return

  await formRef.value.validate(async (valid) => {
    if (valid) {
      try {
        if (form.value.id) {
          await request.put(`/roles/${form.value.id}`, form.value)
          ElMessage.success('更新成功')
        } else {
          await request.post('/roles', form.value)
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

const handlePermissionChange = (checkedKeys) => {
  console.log('Permission change:', checkedKeys)
  selectedPermissions.value = checkedKeys
}

const handleSavePermissions = async () => {
  if (!currentRole.value) return

  try {
    const checkedKeys = permissionTreeRef.value.getCheckedPermissions()
    console.log('Saving permissions:', checkedKeys)
    await request.put(`/roles/${currentRole.value.id}/permissions`, {
      permission_ids: checkedKeys
    })
    ElMessage.success('权限设置成功')
    permissionDialogVisible.value = false
    fetchRoles() // 刷新角色列表
  } catch (error) {
    console.error('Failed to save permissions:', error)
    ElMessage.error('权限设置失败')
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

const filterMethod = (query, item) => {
  return item.label.indexOf(query) > -1;
};

onMounted(() => {
  fetchRoles()
})
</script>

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

        &.permission {
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

.role-tag {
  display: inline-block;
  padding: 2px 8px;
  border-radius: var(--border-radius-base);
  text-align: center;
  font-weight: 500;
  font-size: 13px;
  color: white;
  white-space: nowrap;
}

.color-picker-wrapper {
  .color-inputs {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 12px;
  }

  .color-preview {
    height: 36px;
    border-radius: var(--border-radius-base);
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    font-weight: 500;
    margin-bottom: 16px;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
  }

  .preset-colors {
    .preset-title {
      font-size: 14px;
      color: var(--text-color-secondary);
      margin-bottom: 8px;
    }

    .preset-list {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
      gap: 8px;
    }

    .preset-item {
      height: 32px;
      border-radius: var(--border-radius-base);
      display: flex;
      align-items: center;
      justify-content: center;
      color: white;
      font-size: 12px;
      cursor: pointer;
      transition: all 0.3s;
      text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);

      &:hover {
        transform: translateY(-1px);
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
      }

      &:active {
        transform: translateY(0);
      }
    }
  }
}

.permission-tags {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
  justify-content: center;
}

.permission-tag {
  display: inline-block;
  padding: 2px 8px;
  border-radius: var(--border-radius-base);
  text-align: center;
  font-weight: 500;
  font-size: 13px;
  color: white;
  white-space: nowrap;

  &.no-permission {
    background: linear-gradient(135deg, #909399 0%, #A6A9AD 100%);
  }
}
</style>