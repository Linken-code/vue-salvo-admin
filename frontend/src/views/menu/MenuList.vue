<script setup>
import { ref, onMounted } from 'vue'
import { Plus, Edit, Delete } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import IconSelect from '../../components/IconSelect.vue'

const tableData = ref([])
const loading = ref(false)
const dialogVisible = ref(false)
const dialogTitle = ref('新增菜单')
const formRef = ref(null)

const formData = ref({
    parent_id: null,
    name: '',
    path: '',
    component: '',
    title: '',
    icon: '',
    sort: 0,
    is_hidden: false
})

const rules = {
    name: [{ required: true, message: '请输入路由名称', trigger: 'blur' }],
    path: [{ required: true, message: '请输入路由路径', trigger: 'blur' }],
    component: [{ required: true, message: '请输入组件路径', trigger: 'blur' }],
    title: [{ required: true, message: '请输入显示名称', trigger: 'blur' }]
}

// 获取菜单列表
const fetchMenus = async () => {
    loading.value = true
    try {
        const response = await fetch('http://localhost:3000/menus')
        const data = await response.json()
        tableData.value = buildMenuTree(data)
    } catch (error) {
        ElMessage.error('获取菜单列表失败')
        console.error('Error:', error)
    } finally {
        loading.value = false
    }
}

// 递归构建树形数据
const buildMenuTree = (items, parentId = null) => {
    return items
        .filter(item => item.parent_id === parentId)
        .sort((a, b) => a.sort - b.sort)
        .map(item => ({
            ...item,
            children: buildMenuTree(items, item.id)
        }))
}

// 添加菜单
const handleAdd = (row) => {
    dialogTitle.value = '新增菜单'
    formData.value = {
        parent_id: row?.id || null,
        name: '',
        path: '',
        component: '',
        title: '',
        icon: '',
        sort: 0,
        is_hidden: false
    }
    dialogVisible.value = true
}

// 编辑菜单
const handleEdit = (row) => {
    dialogTitle.value = '编辑菜单'
    formData.value = { ...row }
    dialogVisible.value = true
}

// 删除菜单
const handleDelete = (row) => {
    ElMessageBox.confirm('确认删除该菜单吗？', '提示', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
    }).then(async () => {
        try {
            const response = await fetch(`http://localhost:3000/menus/${row.id}`, {
                method: 'DELETE'
            })

            if (response.ok) {
                ElMessage.success('删除成功')
                fetchMenus()
            } else {
                const data = await response.json()
                throw new Error(data.message || '删除失败')
            }
        } catch (error) {
            ElMessage.error(error.message || '删除失败')
            console.error('Error:', error)
        }
    }).catch(() => {})
}

// 提交表单
const submitForm = async () => {
    if (!formRef.value) return

    await formRef.value.validate(async (valid) => {
        if (valid) {
            try {
                const isEdit = formData.value.id !== undefined
                const url = isEdit
                    ? `http://localhost:3000/menus/${formData.value.id}`
                    : 'http://localhost:3000/menus'

                const response = await fetch(url, {
                    method: isEdit ? 'PUT' : 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify(formData.value)
                })

                if (response.ok) {
                    ElMessage.success('保存成功')
                    dialogVisible.value = false
                    fetchMenus()
                } else {
                    throw new Error('保存失败')
                }
            } catch (error) {
                ElMessage.error('保存失败')
                console.error('Error:', error)
            }
        }
    })
}

onMounted(() => {
    fetchMenus()
})
</script>

<template>
    <div class="menu-list">
        <div class="page-header">
            <h2>菜单管理</h2>
            <el-button type="primary" @click="handleAdd()">
                <el-icon><Plus /></el-icon>新增菜单
            </el-button>
        </div>

        <el-card shadow="never">
            <el-table
                :data="tableData"
                style="width: 100%"
                v-loading="loading"
                row-key="id"
                default-expand-all
                :tree-props="{ children: 'children', hasChildren: 'hasChildren' }"
            >
                <el-table-column prop="title" label="菜单名称" min-width="200">
                    <template #default="{ row }">
                        <el-icon v-if="row.icon" class="menu-icon">
                            <component :is="row.icon" />
                        </el-icon>
                        <span>{{ row.title }}</span>
                    </template>
                </el-table-column>
                <el-table-column prop="name" label="路由名称" min-width="150" />
                <el-table-column prop="path" label="路由路径" min-width="150" />
                <el-table-column prop="component" label="组件路径" min-width="200" />
                <el-table-column prop="sort" label="排序" width="80" align="center" />
                <el-table-column prop="is_hidden" label="显示状态" width="100" align="center">
                    <template #default="{ row }">
                        <el-tag :type="row.is_hidden ? 'info' : 'success'">
                            {{ row.is_hidden ? '隐藏' : '显示' }}
                        </el-tag>
                    </template>
                </el-table-column>
                <el-table-column label="操作" width="250" align="center" fixed="right">
                    <template #default="{ row }">
                        <el-button link @click="handleAdd(row)">
                            <el-icon><Plus /></el-icon>新增子菜单
                        </el-button>
                        <el-button type="primary" link @click="handleEdit(row)">
                            <el-icon><Edit /></el-icon>编辑
                        </el-button>
                        <el-button type="danger" link @click="handleDelete(row)">
                            <el-icon><Delete /></el-icon>删除
                        </el-button>
                    </template>
                </el-table-column>
            </el-table>
        </el-card>

        <!-- 新增/编辑菜单对话框 -->
        <el-dialog v-model="dialogVisible" :title="dialogTitle" width="600px" destroy-on-close>
            <el-form ref="formRef" :model="formData" :rules="rules" label-width="100px">
                <el-form-item label="上级菜单" prop="parent_id">
                    <el-tree-select
                        v-model="formData.parent_id"
                        :data="tableData"
                        node-key="id"
                        :props="{ label: 'title', value: 'id' }"
                        placeholder="请选择上级菜单"
                        clearable
                    />
                </el-form-item>
                <el-form-item label="路由名称" prop="name">
                    <el-input v-model="formData.name" placeholder="请输入路由名称" />
                </el-form-item>
                <el-form-item label="路由路径" prop="path">
                    <el-input v-model="formData.path" placeholder="请输入路由路径" />
                </el-form-item>
                <el-form-item label="组件路径" prop="component">
                    <el-input v-model="formData.component" placeholder="请输入组件路径" />
                </el-form-item>
                <el-form-item label="显示名称" prop="title">
                    <el-input v-model="formData.title" placeholder="请输入显示名称" />
                </el-form-item>
                <el-form-item label="图标" prop="icon">
                    <icon-select v-model="formData.icon" />
                </el-form-item>
                <el-form-item label="排序" prop="sort">
                    <el-input-number v-model="formData.sort" :min="0" />
                </el-form-item>
                <el-form-item label="显示状态" prop="is_hidden">
                    <el-switch
                        v-model="formData.is_hidden"
                        active-text="隐藏"
                        inactive-text="显示"
                        :active-value="true"
                        :inactive-value="false"
                    />
                </el-form-item>
            </el-form>
            <template #footer>
                <el-button @click="dialogVisible = false">取消</el-button>
                <el-button type="primary" @click="submitForm">确定</el-button>
            </template>
        </el-dialog>
    </div>
</template>

<style scoped>
.menu-list {
    padding: 20px;
}

.page-header {
    margin-bottom: 24px;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.page-header h2 {
    margin: 0;
    font-size: 24px;
    color: var(--text-primary);
}

:deep(.el-card) {
    border: none;
}

:deep(.el-table) {
    border-radius: 4px;
}

:deep(.el-button.is-link) {
    padding: 0 4px;
}

:deep(.el-dialog__body) {
    padding-top: 0;
}

.menu-icon {
    margin-right: 8px;
    font-size: 16px;
    vertical-align: middle;
}
</style>