<template>
  <div class="page-container">
    <div class="page-header">
      <div class="left">
        <div class="page-title">
          <el-icon>
            <Lock />
          </el-icon>
          权限管理
        </div>
        <div class="page-subtitle">管理系统权限及其配置</div>
      </div>
      <el-button class="btn-add" type="primary" @click="handleAdd">
        <el-icon>
          <Plus />
        </el-icon>
        添加权限
      </el-button>
    </div>

    <div class="content-wrapper">
      <div class="search-wrapper">
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
      </div>

      <div class="table-wrapper">
        <el-table :data="permissions" class="menu-table" :highlight-current-row="true" :border="false">
          <el-table-column prop="id" label="ID" width="80">
            <template #default="{ row }">
              <span class="text-tag">{{ row.id }}</span>
            </template>
          </el-table-column>
          <el-table-column prop="name" label="权限名称" width="140">
            <template #default="{ row }">
              <span class="permission-tag" :style="{ background: getPermissionGradient(row) }">
                {{ row.name }}
              </span>
            </template>
          </el-table-column>
          <el-table-column prop="code" label="权限代码" width="140">
            <template #default="{ row }">
              <span class="text-tag">{{ row.code }}</span>
            </template>
          </el-table-column>
          <el-table-column prop="description" label="描述" min-width="200">
            <template #default="{ row }">
              <span class="text-tag">{{ row.description }}</span>
            </template>
          </el-table-column>
          <el-table-column prop="created_at" label="创建时间" width="180">
            <template #default="{ row }">
              <span class="text-tag">{{ formatDateTime(row.created_at) }}</span>
            </template>
          </el-table-column>
          <el-table-column label="操作" width="180" fixed="right">
            <template #default="{ row }">
              <div class="action-buttons">
                <span class="operation-tag update" @click="handleEdit(row)">
                  <el-icon>
                    <Edit />
                  </el-icon>编辑
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
        <el-form-item label="权限颜色">
          <div class="color-picker-wrapper">
            <div class="color-inputs">
              <el-color-picker v-model="form.color_start" show-alpha />
              <el-icon>
                <ArrowRight />
              </el-icon>
              <el-color-picker v-model="form.color_end" show-alpha />
            </div>
            <div class="color-preview" :style="{ background: getPermissionGradient(form) }">
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
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import request from '../../utils/request'
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
  description: '',
  color_start: '', // 渐变开始颜色
  color_end: '' // 渐变结束颜色
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

// 预设的颜色方案
const presetColors = [
  { name: '系统管理', start: '#9C27B0', end: '#BA68C8' },
  { name: '用户管理', start: '#1976D2', end: '#42A5F5' },
  { name: '角色管理', start: '#388E3C', end: '#66BB6A' },
  { name: '菜单管理', start: '#FFA000', end: '#FFB74D' },
  { name: '权限管理', start: '#0097A7', end: '#26C6DA' },
  { name: '日志管理', start: '#5D4037', end: '#8D6E63' },
  { name: '操作管理', start: '#C2185B', end: '#EC407A' },
  { name: '默认主题', start: '#757575', end: '#BDBDBD' }
]

// 应用预设颜色
const applyPresetColor = (preset) => {
  form.value.color_start = preset.start
  form.value.color_end = preset.end
}

// 获取权限的渐变色
const getPermissionGradient = (permission) => {
  if (permission.color_start && permission.color_end) {
    return `linear-gradient(135deg, ${permission.color_start} 0%, ${permission.color_end} 100%)`
  }
  // 如果没有设置颜色，使用默认颜色
  return `linear-gradient(135deg, #757575 0%, #BDBDBD 100%)`
}

const fetchPermissions = async () => {
  try {
    const response = await request.get('/permissions', {
      params: {
        page: currentPage.value,
        page_size: pageSize.value,
        ...searchForm.value
      }
    })
    permissions.value = response.items
    total.value = response.total
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
    description: '',
    color_start: '', // 渐变开始颜色
    color_end: '' // 渐变结束颜色
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
      await request.delete(`/permissions/${row.id}`)
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
    description: '',
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
          await request.put(`/permissions/${form.value.id}`, form.value)
          ElMessage.success('更新成功')
        } else {
          await request.post('/permissions', form.value)
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

.permission-tag {
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
</style>