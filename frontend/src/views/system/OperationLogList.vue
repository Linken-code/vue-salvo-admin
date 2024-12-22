<template>
    <div class="page-container">
        <div class="page-header">
            <div class="left">
                <div class="page-title">
                    <el-icon>
                        <Document />
                    </el-icon>
                    操作日志
                </div>
                <div class="page-subtitle">系统操作日志记录</div>
            </div>
            <el-button class="btn-add btn-danger" type="danger" @click="handleClear">
                <el-icon>
                    <Delete />
                </el-icon>
                清空日志
            </el-button>
        </div>

        <div class="content-wrapper">
            <div class="search-wrapper">
                <el-form :model="queryForm" class="search-form" inline>
                    <el-form-item label="用户名">
                        <el-input v-model="queryForm.username" placeholder="请输入用户名" clearable
                            @keyup.enter="handleSearch" />
                    </el-form-item>
                    <el-form-item label="模块">
                        <el-input v-model="queryForm.module" placeholder="请输入模块" clearable
                            @keyup.enter="handleSearch" />
                    </el-form-item>
                    <el-form-item label="操作">
                        <el-input v-model="queryForm.operation" placeholder="请输入操作" clearable
                            @keyup.enter="handleSearch" />
                    </el-form-item>
                    <el-form-item label="状态">
                        <el-select v-model="queryForm.status" placeholder="请选择状态" clearable style="width: 160px;">
                            <el-option label="成功" :value="200" />
                            <el-option label="失败" :value="500" />
                        </el-select>
                    </el-form-item>
                    <el-form-item>
                        <el-button type="primary" @click="handleSearch">查询</el-button>
                        <el-button @click="handleReset">重置</el-button>
                    </el-form-item>
                </el-form>
            </div>

            <div class="table-wrapper">
                <el-table v-loading="loading" :data="logList" class="menu-table" :highlight-current-row="true"
                    :border="false">
                    <el-table-column prop="username" label="用户名" width="120">
                        <template #default="{ row }">
                            <span class="text-tag">{{ row.username }}</span>
                        </template>
                    </el-table-column>
                    <el-table-column prop="module" label="模块" width="140">
                        <template #default="{ row }">
                            <span class="module-tag" :class="{
                                'user': row.module.includes('用户'),
                                'role': row.module.includes('角色'),
                                'permission': row.module.includes('权限'),
                                'menu': row.module.includes('菜单'),
                                'system': row.module.includes('系统'),
                                'log': row.module.includes('日志')
                            }">{{ row.module }}</span>
                        </template>
                    </el-table-column>
                    <el-table-column prop="operation" label="操作" min-width="160">
                        <template #default="{ row }">
                            <span class="operation-tag" :class="{
                                'create': row.operation.includes('创建') || row.operation.includes('添加'),
                                'update': row.operation.includes('更新') || row.operation.includes('修改'),
                                'delete': row.operation.includes('删除'),
                                'query': row.operation.includes('查询') || row.operation.includes('获取'),
                                'login': row.operation.includes('登录'),
                                'logout': row.operation.includes('登出'),
                                'upload': row.operation.includes('上传'),
                                'download': row.operation.includes('下载'),
                                'import': row.operation.includes('导入'),
                                'export': row.operation.includes('导出')
                            }">{{ row.operation }}</span>
                        </template>
                    </el-table-column>
                    <el-table-column prop="method" label="请求方法" width="100">
                        <template #default="{ row }">
                            <span class="method-tag" :class="{
                                'get': row.method === 'GET',
                                'post': row.method === 'POST',
                                'put': row.method === 'PUT',
                                'delete': row.method === 'DELETE'
                            }">{{ row.method }}</span>
                        </template>
                    </el-table-column>
                    <el-table-column prop="ip" label="IP地址" width="200">
                        <template #default="{ row }">
                            <span class="text-tag">{{ row.ip }}</span>
                        </template>
                    </el-table-column>
                    <el-table-column prop="status" label="状态" width="80">
                        <template #default="{ row }">
                            <span class="status-tag" :class="{
                                'success': row.status >= 200 && row.status < 300,
                                'error': row.status >= 500,
                                'warning': row.status >= 400 && row.status < 500,
                                'info': row.status >= 300 && row.status < 400
                            }">
                                {{ row.status >= 200 && row.status < 300 ? '成功' : '失败' }} </span>
                        </template>
                    </el-table-column>
                    <el-table-column prop="created_at" label="操作时间" width="180">
                        <template #default="{ row }">
                            <span class="text-tag">{{ row.created_at }}</span>
                        </template>
                    </el-table-column>
                    <el-table-column label="请求参数" width="100">
                        <template #default="{ row }">
                            <div class="action-buttons" v-if="row.params">
                                <span class="path-tag" @click="handleViewParams(row)">
                                    <el-icon>
                                        <View />
                                    </el-icon>查看
                                </span>
                            </div>
                            <span v-else>-</span>
                        </template>
                    </el-table-column>
                    <el-table-column label="错误信息" width="100">
                        <template #default="{ row }">
                            <div class="action-buttons" v-if="row.error">
                                <span class="component-tag empty" @click="handleViewError(row)">
                                    <el-icon>
                                        <Warning />
                                    </el-icon>查看
                                </span>
                            </div>
                            <span v-else>-</span>
                        </template>
                    </el-table-column>
                </el-table>
            </div>

            <div class="pagination-wrapper">
                <el-pagination v-model:current-page="queryForm.page" v-model:page-size="queryForm.page_size"
                    :total="total" :page-sizes="[10, 20, 50, 100]" layout="total, sizes, prev, pager, next"
                    @size-change="handleSizeChange" @current-change="handleCurrentChange" />
            </div>
        </div>

        <el-dialog v-model="dialogVisible" :title="dialogTitle" width="600px">
            <pre class="json-viewer">{{ dialogContent }}</pre>
        </el-dialog>
    </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { ElMessageBox, ElMessage } from 'element-plus'
import { getOperationLogs, deleteOperationLogs } from '@/api/operation-log.js'

const loading = ref(false)
const logList = ref([])
const total = ref(0)
const queryForm = reactive({
    username: '',
    module: '',
    operation: '',
    status: undefined,
    page: 1,
    page_size: 10
})

const dialogVisible = ref(false)
const dialogTitle = ref('')
const dialogContent = ref('')

const fetchData = async () => {
    loading.value = true
    try {
        const res = await getOperationLogs(queryForm)
        logList.value = res.items
        total.value = res.total
    } catch (error) {
        console.error('获取操作日志失败:', error)
    } finally {
        loading.value = false
    }
}

const handleSearch = () => {
    queryForm.page = 1
    fetchData()
}

const handleReset = () => {
    queryForm.username = ''
    queryForm.module = ''
    queryForm.operation = ''
    queryForm.status = undefined
    queryForm.page = 1
    fetchData()
}

const handleSizeChange = (val) => {
    queryForm.page_size = val
    fetchData()
}

const handleCurrentChange = (val) => {
    queryForm.page = val
    fetchData()
}

const handleClear = () => {
    ElMessageBox.confirm('确定要清空所有操作日志吗？', '警告', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
    })
        .then(async () => {
            try {
                await deleteOperationLogs()
                ElMessage.success('清空操作日志成功')
                fetchData()
            } catch (error) {
                console.error('清空操作日志失败:', error)
            }
        })
        .catch(() => { })
}

const handleViewParams = (row) => {
    dialogTitle.value = '请求参数'
    try {
        const params = JSON.parse(row.params || '{}')
        dialogContent.value = JSON.stringify(params, null, 2)
    } catch (error) {
        dialogContent.value = row.params || ''
    }
    dialogVisible.value = true
}

const handleViewError = (row) => {
    dialogTitle.value = '错误信息'
    try {
        const error = JSON.parse(row.error || '{}')
        dialogContent.value = JSON.stringify(error, null, 2)
    } catch (error) {
        dialogContent.value = row.error || ''
    }
    dialogVisible.value = true
}

onMounted(() => {
    fetchData()
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
        border: none;
        transition: all 0.3s;

        &:hover {
            transform: translateY(-1px);
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
        }

        &:active {
            transform: translateY(0);
        }

        &.btn-danger {
            background: linear-gradient(135deg, rgba(245, 108, 108, 0.9) 0%, rgba(228, 23, 73, 0.9) 100%);
            color: rgba(255, 255, 255, 0.95);

            &:hover {
                background: linear-gradient(135deg, #F56C6C 0%, #e41749 100%);
                color: white;
            }
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

            .method-tag {
                display: inline-block;
                padding: 2px 8px;
                border-radius: var(--border-radius-base);
                text-align: center;
                font-weight: 500;
                font-size: 13px;
                color: white;

                &.get {
                    background: linear-gradient(135deg, #67C23A 0%, #85CE61 100%);
                }

                &.post {
                    background: linear-gradient(135deg, #409EFF 0%, #53A8FF 100%);
                }

                &.put {
                    background: linear-gradient(135deg, #E6A23C 0%, #EBBD5A 100%);
                }

                &.delete {
                    background: linear-gradient(135deg, #F56C6C 0%, #F78989 100%);
                }
            }

            .module-tag {
                display: inline-block;
                padding: 2px 8px;
                border-radius: var(--border-radius-base);
                text-align: center;
                font-weight: 500;
                font-size: 13px;
                color: white;

                &.user {
                    background: linear-gradient(135deg, #409EFF 0%, #53A8FF 100%);
                }

                &.role {
                    background: linear-gradient(135deg, #67C23A 0%, #85CE61 100%);
                }

                &.permission {
                    background: linear-gradient(135deg, #E6A23C 0%, #EBBD5A 100%);
                }

                &.menu {
                    background: linear-gradient(135deg, #9C27B0 0%, #BA68C8 100%);
                }

                &.system {
                    background: linear-gradient(135deg, #795548 0%, #8D6E63 100%);
                }

                &.log {
                    background: linear-gradient(135deg, #607D8B 0%, #78909C 100%);
                }
            }

            .operation-tag {
                display: inline-block;
                padding: 2px 8px;
                border-radius: var(--border-radius-base);
                text-align: center;
                font-weight: 500;
                font-size: 13px;
                color: white;

                &.create {
                    background: linear-gradient(135deg, #67C23A 0%, #85CE61 100%);
                }

                &.update {
                    background: linear-gradient(135deg, #409EFF 0%, #53A8FF 100%);
                }

                &.delete {
                    background: linear-gradient(135deg, #F56C6C 0%, #F78989 100%);
                }

                &.query {
                    background: linear-gradient(135deg, #909399 0%, #A6A9AD 100%);
                }

                &.login {
                    background: linear-gradient(135deg, #9C27B0 0%, #BA68C8 100%);
                }

                &.logout {
                    background: linear-gradient(135deg, #795548 0%, #8D6E63 100%);
                }

                &.upload {
                    background: linear-gradient(135deg, #00BCD4 0%, #26C6DA 100%);
                }

                &.download {
                    background: linear-gradient(135deg, #009688 0%, #26A69A 100%);
                }

                &.import {
                    background: linear-gradient(135deg, #3F51B5 0%, #5C6BC0 100%);
                }

                &.export {
                    background: linear-gradient(135deg, #2196F3 0%, #42A5F5 100%);
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

                &.warning {
                    background: linear-gradient(135deg, #E6A23C 0%, #EBBD5A 100%);
                }

                &.info {
                    background: linear-gradient(135deg, #909399 0%, #A6A9AD 100%);
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

.json-viewer {
    margin: 0;
    padding: 16px;
    background: linear-gradient(135deg, var(--primary-light-9) 0%, rgba(255, 255, 255, 0.95) 100%);
    border-radius: var(--border-radius-base);
    white-space: pre-wrap;
    word-wrap: break-word;
    font-family: monospace;
    font-size: 14px;
    line-height: 1.5;
    color: var(--text-color-primary);
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
</style>