<template>
    <div class="error-page">
        <div class="error-code">403</div>
        <div class="error-title">访问被拒绝</div>
        <div class="error-desc">抱歉，您没有权限访问此页面</div>
        <el-button type="primary" @click="goHome">返回首页</el-button>
    </div>
</template>

<script setup>
import { useRouter } from 'vue-router'

const router = useRouter()
const goHome = () => router.push('/')
</script>

<style scoped>
.error-page {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
    background: linear-gradient(135deg, var(--el-color-primary-light-7) 0%, var(--el-color-primary) 100%);
    color: white;
}

.error-code {
    font-size: 120px;
    font-weight: bold;
    margin-bottom: 20px;
    text-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}

.error-title {
    font-size: 32px;
    font-weight: bold;
    margin-bottom: 16px;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.error-desc {
    font-size: 18px;
    margin-bottom: 32px;
    opacity: 0.9;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
}

.el-button {
    padding: 12px 24px;
    font-size: 16px;
    background: white;
    color: var(--el-color-primary);
    border: none;
    transition: all 0.3s;
}

.el-button:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

.el-button:active {
    transform: translateY(0);
}
</style>