<script setup>
import { ref, computed } from 'vue'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import { markRaw } from 'vue'
import { Search } from '@element-plus/icons-vue'

const props = defineProps({
  modelValue: String
})

const emit = defineEmits(['update:modelValue'])

const dialogVisible = ref(false)
const searchText = ref('')
const inputValue = ref(props.modelValue)

// 将所有图标转换为数组
const icons = Object.entries(ElementPlusIconsVue).map(([name, component]) => ({
  name,
  component: markRaw(component)
}))

// 根据搜索文本过滤图标
const filteredIcons = computed(() => {
  const text = searchText.value.toLowerCase()
  return icons.filter(icon => icon.name.toLowerCase().includes(text))
})

// 选择图标
const selectIcon = (iconName) => {
  inputValue.value = iconName
  emit('update:modelValue', iconName)
  dialogVisible.value = false
}

// 清除图标
const clearIcon = () => {
  inputValue.value = ''
  emit('update:modelValue', '')
}
</script>

<template>
  <div class="icon-select">
    <el-input
      :model-value="modelValue"
      placeholder="请选择图标"
      readonly
      @click="dialogVisible = true"
      clearable
      @clear="clearIcon"
    >
      <template #prefix>
        <el-icon v-if="modelValue">
          <component :is="modelValue" />
        </el-icon>
      </template>
      <template #append>
        <el-button @click="dialogVisible = true">
          选择图标
        </el-button>
      </template>
    </el-input>

    <el-dialog
      v-model="dialogVisible"
      title="选择图标"
      width="800px"
      destroy-on-close
    >
      <el-input
        v-model="searchText"
        placeholder="搜索图标"
        clearable
        class="search-input"
      >
        <template #prefix>
          <el-icon><Search /></el-icon>
        </template>
      </el-input>

      <div class="icons-container">
        <div
          v-for="icon in filteredIcons"
          :key="icon.name"
          class="icon-item"
          @click="selectIcon(icon.name)"
          :class="{ active: modelValue === icon.name }"
        >
          <el-icon>
            <component :is="icon.component" />
          </el-icon>
          <span class="icon-name">{{ icon.name }}</span>
        </div>
      </div>
    </el-dialog>
  </div>
</template>

<style scoped>
.icon-select {
  width: 100%;
}

.search-input {
  margin-bottom: 20px;
}

.icons-container {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 12px;
  max-height: 400px;
  overflow-y: auto;
  padding: 12px;
}

.icon-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 12px;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.3s;
  border: 1px solid var(--header-border);
}

.icon-item:hover {
  background-color: var(--primary-bg);
  transform: translateY(-2px);
}

.icon-item.active {
  border-color: var(--menu-active);
  color: var(--menu-active);
}

.icon-item .el-icon {
  font-size: 24px;
  margin-bottom: 8px;
}

.icon-name {
  font-size: 12px;
  color: var(--text-regular);
  text-align: center;
  word-break: break-all;
}

:deep(.el-dialog__body) {
  padding-top: 0;
}
</style> 