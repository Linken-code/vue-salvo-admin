<template>
    <div class="permission-tree">
        <el-tree ref="treeRef" :data="permissionTree" show-checkbox node-key="id" :props="defaultProps"
            :default-checked-keys="checkedKeys" @check="handleCheck">
            <template #default="{ node, data }">
                <div class="custom-tree-node">
                    <span class="label">{{ data.name }}</span>
                    <el-tag size="small" :type="getTagType(data.type_name)" class="permission-type">
                        {{ data.type_name }}
                    </el-tag>
                    <el-tag size="small" :type="getActionType(data.action)" class="permission-action">
                        {{ data.action }}
                    </el-tag>
                    <el-tooltip v-if="data.description" :content="data.description" placement="top">
                        <el-icon class="info-icon">
                            <InfoFilled />
                        </el-icon>
                    </el-tooltip>
                </div>
            </template>
        </el-tree>
    </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { ElTree, ElTag, ElTooltip, ElMessage } from 'element-plus'
import { InfoFilled } from '@element-plus/icons-vue'
import request from '@/utils/request'

const props = defineProps({
    roleId: {
        type: [String, Number],
        required: true
    }
})

const emit = defineEmits(['update:modelValue', 'change'])

const treeRef = ref()
const permissionTree = ref([])
const checkedKeys = ref([])

const defaultProps = {
    children: 'children',
    label: 'name'
}

// 获取权限树数据
const loadPermissionTree = async () => {
    if (!props.roleId) {
        console.error('Role ID is required')
        return
    }

    try {
        const response = await request.get(`/roles/${props.roleId}/permissions`)
        if (response.data) {
            permissionTree.value = response.data.data || []
            checkedKeys.value = response.data.checkedKeys || []
            console.log('Permission tree data:', permissionTree.value)
            console.log('Checked keys:', checkedKeys.value)
        } else {
            console.error('Invalid response format:', response)
        }
    } catch (error) {
        console.error('Failed to load permission tree:', error)
        ElMessage.error('加载权限数据失败')
    }
}

// 处理权限选择变化
const handleCheck = (node, checkedInfo) => {
    const { checkedKeys } = checkedInfo
    emit('change', checkedKeys)
}

// 获取权限类型对应的Tag类型
const getTagType = (type) => {
    const typeMap = {
        'API': 'success',
        'PAGE': 'info'
    }
    return typeMap[type] || ''
}

// 获取操作类型对应的Tag类型
const getActionType = (action) => {
    const actionMap = {
        'GET': 'info',
        'POST': 'success',
        'PUT': 'warning',
        'DELETE': 'danger',
        'VIEW': 'primary'
    }
    return actionMap[action] || ''
}

// 获取当前选中的所有权限ID
const getCheckedPermissions = () => {
    if (!treeRef.value) return []
    return treeRef.value.getCheckedKeys()
}

// 对外暴露方法
defineExpose({
    getCheckedPermissions
})

onMounted(() => {
    loadPermissionTree()
})
</script>

<style scoped>
.permission-tree {
    padding: 20px;
}

.custom-tree-node {
    display: flex;
    align-items: center;
    gap: 8px;
}

.label {
    min-width: 120px;
}

.permission-type,
.permission-action {
    font-size: 11px;
}

.info-icon {
    font-size: 14px;
    color: #909399;
    cursor: help;
}
</style>