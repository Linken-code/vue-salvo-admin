<script setup>
import { ref, computed, onMounted } from 'vue'
import { Plus, Edit, Delete } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import IconSelect from '../../components/IconSelect.vue'
import request from '../../utils/request'

const menus = ref([])
const dialogVisible = ref(false)
const dialogType = ref('add')
const loading = ref(false)
const formRef = ref(null)

const form = ref({
    parent_id: null,
    title: '',
    name: '',
    path: '',
    component: '',
    icon: '',
    sort: 0,
    is_hidden: false
})

const rules = {
    title: [{ required: true, message: '请输入菜单名称', trigger: 'blur' }],
    name: [{ required: true, message: '请输入路由名称', trigger: 'blur' }],
    path: [{ required: true, message: '请输入路由路径', trigger: 'blur' }],
    component: [{ required: true, message: '请输入组件路径', trigger: 'blur' }]
}

const fetchMenus = async () => {
    loading.value = true;
    try {
        const response = await request.get('/menus');
        if (Array.isArray(response)) {
            menus.value = buildMenuTree(response);
        } else {
            console.error('Invalid menu data:', response);
            menus.value = [];
        }
    } catch (error) {
        console.error('Failed to fetch menus:', error);
        ElMessage.error('获取菜单列表失败');
        menus.value = [];
    } finally {
        loading.value = false;
    }
};

const buildMenuTree = (items, parentId = null) => {
    if (!Array.isArray(items)) return []
    return items
        .filter(item => item.parent_id === parentId)
        .sort((a, b) => a.sort - b.sort)
        .map(item => ({
            ...item,
            children: buildMenuTree(items, item.id)
        }))
}

const menuOptions = computed(() => {
    return [
        {
            id: null,
            title: '顶级菜单',
            children: menus.value
        }
    ]
})

const handleAdd = (row) => {
    dialogType.value = 'add'
    form.value = {
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

const handleEdit = (row) => {
    dialogType.value = 'edit'
    form.value = { ...row }
    dialogVisible.value = true
}

const handleDelete = async (row) => {
    try {
        await ElMessageBox.confirm('确认删除该菜单吗？', '提示', {
            confirmButtonText: '确定',
            cancelButtonText: '取消',
            type: 'warning'
        });

        await request.delete(`/menus/${row.id}`);
        ElMessage.success('删除成功');
        fetchMenus();
    } catch (error) {
        if (error !== 'cancel') {
            console.error('Failed to delete menu:', error);
            ElMessage.error('删除失败');
        }
    }
};

const submitForm = async () => {
    if (!formRef.value) return;

    try {
        await formRef.value.validate();
        const isEdit = form.value.id !== undefined;

        if (isEdit) {
            await request.put(`/menus/${form.value.id}`, form.value);
        } else {
            await request.post('/menus', form.value);
        }

        ElMessage.success('保存成功');
        dialogVisible.value = false;
        fetchMenus();
    } catch (error) {
        console.error('Failed to submit form:', error);
        ElMessage.error('保存失败');
    }
};

onMounted(() => {
    fetchMenus()
})
</script>

<template>
    <div class="menu-list">
        <div class="page-header">
            <h2>菜单管理</h2>
            <el-button type="primary" @click="handleAdd()">
                <el-icon>
                    <Plus />
                </el-icon>新增菜单
            </el-button>
        </div>

        <el-card shadow="never">
            <el-table :data="menus" style="width: 100%" v-loading="loading" row-key="id" default-expand-all
                :tree-props="{ children: 'children', hasChildren: 'hasChildren' }">
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
                            <el-icon>
                                <Plus />
                            </el-icon>新增子菜单
                        </el-button>
                        <el-button type="primary" link @click="handleEdit(row)">
                            <el-icon>
                                <Edit />
                            </el-icon>编辑
                        </el-button>
                        <el-button type="danger" link @click="handleDelete(row)">
                            <el-icon>
                                <Delete />
                            </el-icon>删除
                        </el-button>
                    </template>
                </el-table-column>
            </el-table>
        </el-card>

        <!-- 新增/编辑菜单对话框 -->
        <el-dialog v-model="dialogVisible" :title="dialogType === 'add' ? '新增菜单' : '编辑菜单'" width="600px"
            destroy-on-close>
            <el-form ref="formRef" :model="form" :rules="rules" label-width="100px">
                <el-form-item label="上级菜单" prop="parent_id">
                    <el-tree-select v-model="form.parent_id" :data="menuOptions" node-key="id"
                        :props="{ label: 'title', value: 'id' }" placeholder="请选择上级菜单" clearable />
                </el-form-item>
                <el-form-item label="路由名称" prop="name">
                    <el-input v-model="form.name" placeholder="请输入路由名称" />
                </el-form-item>
                <el-form-item label="路由路径" prop="path">
                    <el-input v-model="form.path" placeholder="请输入路由路径" />
                </el-form-item>
                <el-form-item label="组件路径" prop="component">
                    <el-input v-model="form.component" placeholder="请输入组件路径" />
                </el-form-item>
                <el-form-item label="显示名称" prop="title">
                    <el-input v-model="form.title" placeholder="请输入显示名称" />
                </el-form-item>
                <el-form-item label="图标" prop="icon">
                    <icon-select v-model="form.icon" />
                </el-form-item>
                <el-form-item label="排序" prop="sort">
                    <el-input-number v-model="form.sort" :min="0" />
                </el-form-item>
                <el-form-item label="显示状态" prop="is_hidden">
                    <el-switch v-model="form.is_hidden" active-text="隐藏" inactive-text="显示" :active-value="true"
                        :inactive-value="false" />
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