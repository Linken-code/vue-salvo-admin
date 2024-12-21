<script setup>
import { ref, onMounted } from 'vue'
import { Histogram, DataLine, Coin, User } from '@element-plus/icons-vue'
import { markRaw } from 'vue'
import * as echarts from 'echarts'

const statistics = ref([
    { title: '总访问量', value: '1,234', icon: markRaw(Histogram), color: '#409EFF' },
    { title: '今日访问', value: '123', icon: markRaw(DataLine), color: '#67C23A' },
    { title: '总收入', value: '¥12,345', icon: markRaw(Coin), color: '#E6A23C' },
    { title: '活跃用户', value: '345', icon: markRaw(User), color: '#F56C6C' }
])

const trendChart = ref(null)
const userDistChart = ref(null)

const trendOption = {
    tooltip: {
        trigger: 'axis'
    },
    grid: {
        left: '3%',
        right: '4%',
        bottom: '3%',
        containLabel: true,
        show: true,
        borderColor: 'transparent',
        backgroundColor: 'transparent',
        splitLine: {
            lineStyle: {
                color: 'rgba(128, 128, 128, 0.1)'
            }
        }
    },
    xAxis: {
        type: 'category',
        boundaryGap: false,
        data: ['周一', '周二', '周三', '���四', '周五', '周六', '周日'],
        axisLine: {
            lineStyle: {
                color: 'rgba(128, 128, 128, 0.2)'
            }
        },
        axisLabel: {
            color: 'var(--text-secondary)'
        }
    },
    yAxis: {
        type: 'value',
        axisLine: {
            show: false
        },
        axisLabel: {
            color: 'var(--text-secondary)'
        }
    },
    series: [
        {
            name: '访问量',
            type: 'line',
            smooth: true,
            data: [120, 132, 101, 134, 90, 230, 210],
            itemStyle: {
                color: '#409EFF'
            },
            areaStyle: {
                color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
                    {
                        offset: 0,
                        color: 'rgba(64, 158, 255, 0.3)'
                    },
                    {
                        offset: 1,
                        color: 'rgba(64, 158, 255, 0.1)'
                    }
                ])
            }
        }
    ]
}

const userDistOption = {
    tooltip: {
        trigger: 'item'
    },
    grid: {
        left: '3%',
        right: '4%',
        bottom: '3%',
        containLabel: true,
        show: true,
        borderColor: 'transparent',
        backgroundColor: 'transparent',
        splitLine: {
            lineStyle: {
                color: 'rgba(128, 128, 128, 0.1)'
            }
        }
    },
    series: [
        {
            name: '用户分布',
            type: 'pie',
            radius: ['40%', '70%'],
            center: ['50%', '50%'],
            avoidLabelOverlap: false,
            itemStyle: {
                borderRadius: 10,
                borderColor: 'var(--el-bg-color)',
                borderWidth: 2
            },
            label: {
                show: true,
                position: 'outside',
                color: 'var(--text-primary)',
                formatter: '{b}: {c} ({d}%)'
            },
            emphasis: {
                label: {
                    show: true,
                    fontSize: 16,
                    fontWeight: 'bold'
                }
            },
            data: [
                { value: 1048, name: '移动端' },
                { value: 735, name: '桌面端' },
                { value: 580, name: '平板' },
                { value: 484, name: '其他' }
            ]
        }
    ]
}

onMounted(() => {
    // 初始化访问趋势图表
    const trendDom = document.getElementById('trend-chart')
    const trendChartInstance = echarts.init(trendDom)
    trendChartInstance.setOption(trendOption)
    trendChart.value = trendChartInstance

    // 初始化用户分布图表
    const userDistDom = document.getElementById('user-dist-chart')
    const userDistChartInstance = echarts.init(userDistDom)
    userDistChartInstance.setOption(userDistOption)
    userDistChart.value = userDistChartInstance

    // 监听窗口大小变化
    window.addEventListener('resize', () => {
        trendChartInstance.resize()
        userDistChartInstance.resize()
    })
})
</script>

<template>
    <div class="dashboard">
        <div class="page-header">
            <h2>仪表盘</h2>
        </div>

        <el-row :gutter="20" class="statistics">
            <el-col :span="6" v-for="item in statistics" :key="item.title">
                <el-card shadow="hover" class="statistic-card">
                    <div class="statistic-content">
                        <div class="statistic-icon" :style="{ backgroundColor: item.color }">
                            <el-icon>
                                <component :is="item.icon" />
                            </el-icon>
                        </div>
                        <div class="statistic-info">
                            <div class="statistic-title">{{ item.title }}</div>
                            <div class="statistic-value">{{ item.value }}</div>
                        </div>
                    </div>
                </el-card>
            </el-col>
        </el-row>

        <el-row :gutter="20" class="charts">
            <el-col :span="12">
                <el-card shadow="hover">
                    <template #header>
                        <div class="card-header">
                            <span>访问趋势</span>
                        </div>
                    </template>
                    <div id="trend-chart" class="chart"></div>
                </el-card>
            </el-col>

            <el-col :span="12">
                <el-card shadow="hover">
                    <template #header>
                        <div class="card-header">
                            <span>用户分布</span>
                        </div>
                    </template>
                    <div id="user-dist-chart" class="chart"></div>
                </el-card>
            </el-col>
        </el-row>
    </div>
</template>

<style scoped>
.dashboard {
    padding: 20px;
}

.page-header {
    margin-bottom: 24px;
}

.page-header h2 {
    margin: 0;
    font-size: 24px;
    color: var(--text-primary);
}

.statistics {
    margin-bottom: 20px;
}

.statistic-card {
    height: 108px;
    transition: transform 0.3s;
}

.statistic-card:hover {
    transform: translateY(-4px);
}

.statistic-content {
    display: flex;
    align-items: center;
    height: 100%;
}

.statistic-icon {
    width: 48px;
    height: 48px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-right: 16px;
    transition: transform 0.3s;
}

.statistic-card:hover .statistic-icon {
    transform: scale(1.1);
}

.statistic-icon .el-icon {
    font-size: 24px;
    color: #fff;
}

.statistic-info {
    flex: 1;
}

.statistic-title {
    font-size: 14px;
    color: var(--text-secondary);
    margin-bottom: 8px;
}

.statistic-value {
    font-size: 24px;
    font-weight: bold;
    color: var(--text-primary);
}

.charts {
    margin-top: 20px;
}

.chart {
    height: 300px;
}

.card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    color: var(--text-primary);
}

:deep(.el-card) {
    border: none;
    transition: box-shadow 0.3s;
}

:deep(.el-card:hover) {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

:deep(.el-card__header) {
    border-bottom-color: var(--border-color);
    padding: 12px 20px;
}
</style>