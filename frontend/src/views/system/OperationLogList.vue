<template>
    <div class="operation-log-list">
        <el-card>
            <template #header>
                <div class="card-header">
                    <span>操作日志</span>
                    <el-button type="danger" @click="handleClear">清空日志</el-button>
                </div>
            </template>

            <el-form :model="queryForm" inline>
                <el-form-item label="用户名">
                    <el-input v-model="queryForm.username" placeholder="请输入用户名" clearable @keyup.enter="handleSearch" />
                </el-form-item>
                <el-form-item label="模块">
                    <el-input v-model="queryForm.module" placeholder="请输入模块" clearable @keyup.enter="handleSearch" />
                </el-form-item>
                <el-form-item label="操作">
                    <el-input v-model="queryForm.operation" placeholder="请输入操作" clearable @keyup.enter="handleSearch" />
                </el-form-item>
                <el-form-item label="状态">
                    <el-select v-model="queryForm.status" placeholder="请选择状态" clearable>
                        <el-option label="成功" :value="200" />
                        <el-option label="失败" :value="500" />
                    </el-select>
                </el-form-item>
                <el-form-item>
                    <el-button type="primary" @click="handleSearch">查询</el-button>
                    <el-button @click="handleReset">重置</el-button>
                </el-form-item>
            </el-form>

            <el-table v-loading="loading" :data="logList" border style="width: 100%">
                <el-table-column prop="username" label="用户名" width="120" />
                <el-table-column prop="module" label="模块" width="120" />
                <el-table-column prop="operation" label="操作" width="120" />
                <el-table-column prop="method" label="请求方法" width="100" />
                <el-table-column prop="ip" label="IP地址" width="120" />
                <el-table-column prop="status" label="状态" width="100">
                    <template #default="{ row }">
                        <el-tag :type="row.status >= 200 && row.status < 300 ? 'success' : 'danger'">
                            {{ row.status >= 200 && row.status < 300 ? '成功' : '失败' }} </el-tag>
                    </template>
                </el-table-column>
                <el-table-column prop="created_at" label="操作时间" width="180" />
                <el-table-column label="请求参数" min-width="200">
                    <template #default="{ row }">
                        <el-button v-if="row.params" link type="primary" @click="handleViewParams(row)">查看</el-button>
                        <span v-else>-</span>
                    </template>
                </el-table-column>
                <el-table-column label="错误信息" min-width="200">
                    <template #default="{ row }">
                        <el-button v-if="row.error" link type="primary" @click="handleViewError(row)">查看</el-button>
                        <span v-else>-</span>
                    </template>
                </el-table-column>
            </el-table>

            <div class="pagination">
                <el-pagination v-model:current-page="queryForm.page" v-model:page-size="queryForm.page_size"
                    :total="total" :page-sizes="[10, 20, 50, 100]" layout="total, sizes, prev, pager, next"
                    @size-change="handleSizeChange" @current-change="handleCurrentChange" />
            </div>
        </el-card>

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

<style scoped>
.operation-log-list {
    padding: 20px;
}

.card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.pagination {
    margin-top: 20px;
    display: flex;
    justify-content: flex-end;
}

.json-viewer {
    margin: 0;
    padding: 10px;
    background-color: #f5f7fa;
    border-radius: 4px;
    white-space: pre-wrap;
    word-wrap: break-word;
    font-family: monospace;
}
</style>