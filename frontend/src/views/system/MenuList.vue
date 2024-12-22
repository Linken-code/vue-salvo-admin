<template>
    <div class="page-container">
        <div class="page-header">
            <div class="left">
                <h2 class="page-title">
                    <el-icon>
                        <Menu />
                    </el-icon>
                    菜单管理
                </h2>
                <div class="page-subtitle">管理系统的导航菜单结构</div>
            </div>
            <el-button type="primary" class="btn-add" @click="handleAdd()">
                <el-icon>
                    <Plus />
                </el-icon>新增菜单
            </el-button>
        </div>

        <div class="menu-content">
            <el-table :data="menus" row-key="id" :tree-props="{ children: 'children', hasChildren: 'hasChildren' }"
                class="menu-table" :highlight-current-row="true" :border="false">
                <el-table-column prop="title" label="菜单名称" min-width="200">
                    <template #default="{ row }">
                        <div class="menu-item">
                            <el-icon v-if="row.icon" :class="['menu-icon', row.icon]">
                                <component :is="row.icon" />
                            </el-icon>
                            <span :class="{ 'menu-hidden': row.is_hidden }">{{ row.title }}</span>
                            <span v-if="row.is_hidden" class="hidden-tag">隐藏</span>
                        </div>
                    </template>
                </el-table-column>

                <el-table-column prop="path" label="路由路径" min-width="180">
                    <template #default="{ row }">
                        <span class="path-tag">{{ row.path }}</span>
                    </template>
                </el-table-column>

                <el-table-column prop="component" label="组件路径" min-width="180">
                    <template #default="{ row }">
                        <span class="component-tag" v-if="row.component">{{ row.component }}</span>
                        <span class="component-tag empty" v-else>无组件</span>
                    </template>
                </el-table-column>

                <el-table-column prop="sort" label="排序" width="100" align="center">
                    <template #default="{ row }">
                        <span class="sort-value">{{ row.sort }}</span>
                    </template>
                </el-table-column>

                <el-table-column label="操作" width="200" fixed="right">
                    <template #default="{ row }">
                        <div class="action-buttons">
                            <el-button type="primary" link @click="handleAdd(row)">
                                <el-icon>
                                    <Plus />
                                </el-icon>添加子菜单
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
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>

        <!-- 菜单表单对话框 -->
        <el-dialog :title="dialogType === 'add' ? '新增菜单' : '编辑菜单'" v-model="dialogVisible" width="600px"
            destroy-on-close>
            <el-form ref="formRef" :model="form" :rules="rules" label-width="100px" class="menu-form">
                <el-form-item label="上级菜单">
                    <el-tree-select v-model="form.parent_id" :data="menuTreeData"
                        :props="{ label: 'title', value: 'id' }" placeholder="请选择上级菜单" clearable
                        class="menu-tree-select" />
                </el-form-item>

                <el-form-item label="菜单名称" prop="title">
                    <el-input v-model="form.title" placeholder="请输入菜单名称" />
                </el-form-item>

                <el-form-item label="路由路径" prop="path">
                    <el-input v-model="form.path" placeholder="请输入路由路径">
                        <template #prefix>/</template>
                    </el-input>
                </el-form-item>

                <el-form-item label="组件路径" prop="component">
                    <el-input v-model="form.component" placeholder="请输入组件路径" />
                </el-form-item>

                <el-form-item label="菜单图标" prop="icon">
                    <el-input v-model="form.icon" placeholder="请选择图标">
                        <template #prefix>
                            <el-icon v-if="form.icon">
                                <component :is="form.icon" />
                            </el-icon>
                        </template>
                        <template #append>
                            <el-button @click="showIconPicker = true">选择图标</el-button>
                        </template>
                    </el-input>
                </el-form-item>

                <el-form-item label="排序" prop="sort">
                    <el-input-number v-model="form.sort" :min="0" :max="999" />
                </el-form-item>

                <el-form-item label="是否隐藏">
                    <el-switch v-model="form.is_hidden" />
                </el-form-item>
            </el-form>

            <template #footer>
                <div class="dialog-footer">
                    <el-button @click="dialogVisible = false">取消</el-button>
                    <el-button type="primary" @click="submitForm">确定</el-button>
                </div>
            </template>
        </el-dialog>

        <!-- 图标选择器 -->
        <el-dialog title="选择图标" v-model="showIconPicker" width="800px" destroy-on-close class="icon-picker-dialog">
            <div class="icon-list">
                <div v-for="icon in iconList" :key="icon" class="icon-item" :class="{ active: form.icon === icon }"
                    @click="selectIcon(icon)">
                    <el-icon>
                        <component :is="icon" />
                    </el-icon>
                    <span class="icon-name">{{ icon }}</span>
                </div>
            </div>
        </el-dialog>
    </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import * as ElementPlusIconsVue from '@element-plus/icons-vue';
import request from '../../utils/request';
import { Menu, Plus, Edit, Delete } from '@element-plus/icons-vue';

const menus = ref([]);
const dialogVisible = ref(false);
const dialogType = ref('add');
const formRef = ref();
const showIconPicker = ref(false);

// 图标列表
const iconList = Object.keys(ElementPlusIconsVue);

const form = ref({
    parent_id: null,
    title: '',
    path: '',
    component: '',
    icon: '',
    sort: 0,
    is_hidden: false
});

const rules = {
    title: [
        { required: true, message: '请输入菜单名称', trigger: 'blur' }
    ],
    path: [
        { required: true, message: '请输入路由路径', trigger: 'blur' }
    ]
};

// 用于树形选择器的数据
const menuTreeData = ref([]);

const fetchMenus = async () => {
    try {
        const response = await request.get('/menus');
        menus.value = buildTree(response);
        menuTreeData.value = [
            {
                id: null,
                title: '顶级菜单',
                children: buildTree(response)
            }
        ];
    } catch (error) {
        console.error('Failed to fetch menus:', error);
        ElMessage.error('获取菜单列表失败');
    }
};

// 构建树形结构
const buildTree = (items, parentId = null) => {
    const result = [];
    items.forEach(item => {
        if (item.parent_id === parentId) {
            const children = buildTree(items, item.id);
            if (children.length) {
                item.children = children;
            }
            result.push(item);
        }
    });
    return result.sort((a, b) => a.sort - b.sort);
};

const handleAdd = (row) => {
    dialogType.value = 'add';
    form.value = {
        parent_id: row?.id || null,
        title: '',
        path: '',
        component: '',
        icon: '',
        sort: 0,
        is_hidden: false
    };
    dialogVisible.value = true;
};

const handleEdit = (row) => {
    dialogType.value = 'edit';
    form.value = { ...row };
    dialogVisible.value = true;
};

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

const selectIcon = (icon) => {
    form.value.icon = icon;
    showIconPicker.value = false;
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
    fetchMenus();
});
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

.menu-content {
    position: relative;
    z-index: 1;
    background-color: var(--bg-color);
    border-radius: var(--border-radius-base);
    box-shadow: var(--box-shadow-light);
    padding: 20px;
    overflow-x: auto;

    .menu-table {
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
            }
        }

        :deep(.el-table__cell) {
            &.is-leaf {
                &:nth-child(1) {
                    min-width: 220px;
                }

                &:nth-child(2) {
                    min-width: 160px;
                }

                &:nth-child(3) {
                    min-width: 200px;
                }

                &:nth-child(4) {
                    min-width: 80px;
                }

                &:nth-child(5) {
                    min-width: 240px;
                }
            }

            .el-table__expand-icon {
                display: inline-flex;
                align-items: center;
                height: 24px;
                margin-right: 4px;
                vertical-align: middle;

                .el-icon {
                    transition: transform 0.2s;
                }

                &.el-table__expand-icon--expanded {
                    transform: none;

                    .el-icon {
                        transform: rotate(90deg);
                    }
                }
            }

            .cell {
                display: flex;
                align-items: center;
                min-height: 40px;
            }
        }

        .tag-base {
            display: inline-block;
            padding: 2px 8px;
            border-radius: var(--border-radius-base);
            text-align: center;
            font-weight: 500;
            font-size: 13px;
        }

        .path-tag {
            @extend .tag-base;
            background-color: var(--primary-light-9);
            color: var(--primary-color);
        }

        .component-tag {
            @extend .tag-base;
            background-color: rgba(64, 158, 255, 0.1);
            color: var(--primary-color);

            &.empty {
                background-color: rgba(64, 158, 255, 0.05);
                color: var(--primary-color);
                opacity: 0.6;
            }
        }

        .sort-value {
            @extend .tag-base;
            min-width: 40px;
            background-color: rgba(64, 158, 255, 0.15);
            color: var(--primary-color);
        }

        .hidden-tag {
            @extend .tag-base;
            background-color: rgba(64, 158, 255, 0.05);
            color: var(--primary-color);
            opacity: 0.6;
            margin-left: 8px;
            flex-shrink: 0;
            vertical-align: middle;
        }
    }

    .menu-item {
        display: inline-flex;
        align-items: center;
        gap: 8px;
        vertical-align: middle;
        height: 24px;
        line-height: 24px;

        .menu-icon {
            font-size: 16px;
            color: var(--primary-color);
            transition: all 0.3s;
            flex-shrink: 0;
            display: inline-flex;
            align-items: center;
            justify-content: center;
            width: 20px;
            height: 20px;
        }

        span {
            display: inline-block;
            vertical-align: middle;
        }

        .menu-hidden {
            color: var(--text-secondary);
            text-decoration: line-through;
            flex: 1;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
            line-height: inherit;
        }

        .hidden-tag {
            margin-left: 8px;
            flex-shrink: 0;
            vertical-align: middle;
        }
    }

    .action-buttons {
        display: flex;
        gap: 8px;
        flex-wrap: nowrap;
        justify-content: flex-start;

        .el-button {
            display: inline-flex;
            align-items: center;
            gap: 4px;
            padding: 4px 8px;
            position: relative;
            overflow: hidden;
            white-space: nowrap;
            flex-shrink: 0;

            .el-icon {
                margin-right: 2px;
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

            &.el-button--danger:hover::before {
                background: linear-gradient(135deg, #f56c6c 0%, #e41749 100%);
            }
        }
    }
}

.menu-form {
    .menu-tree-select {
        width: 100%;
    }

    :deep(.el-form-item) {
        margin-bottom: 24px;

        .el-form-item__label {
            font-weight: 500;
            color: var(--text-primary);
        }

        .el-input__wrapper,
        .el-input-number {
            box-shadow: none;
            border: 1px solid var(--border-color);
            transition: all 0.3s;

            &:hover {
                border-color: var(--primary-light-3);
            }

            &.is-focus {
                border-color: var(--primary-color);
                box-shadow: 0 0 0 2px var(--primary-light-8);
            }
        }

        .el-input-number__decrease,
        .el-input-number__increase {
            border: none;
            background-color: var(--primary-light-9);
            color: var(--primary-color);

            &:hover {
                color: white;
                background-color: var(--primary-color);
            }
        }
    }
}

.icon-picker-dialog {
    .icon-list {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
        gap: 16px;
        padding: 16px;

        .icon-item {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 8px;
            padding: 16px;
            border-radius: var(--border-radius-base);
            cursor: pointer;
            transition: all 0.3s;
            border: 1px solid transparent;

            &:hover {
                border-color: var(--primary-color);
                background-color: var(--primary-light-9);
                transform: translateY(-2px);
            }

            &.active {
                background: linear-gradient(135deg, var(--primary-light-7) 0%, var(--primary-color) 100%);
                color: white;
                border: none;
                transform: translateY(-2px);
                box-shadow: 0 4px 12px rgba(64, 158, 255, 0.4);

                .icon-name {
                    color: white;
                }
            }

            .el-icon {
                font-size: 24px;
            }

            .icon-name {
                font-size: 12px;
                color: var(--text-secondary);
            }
        }
    }
}

.dialog-footer {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    padding-top: 20px;
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

    .menu-content {
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
</style>