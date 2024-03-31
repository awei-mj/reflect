<script lang="ts" setup>
  import { Delete, ZoomIn } from '@element-plus/icons-vue'

  defineProps<{
    src: string,
    name: string,
    size: number
  }>()

  const convertSize = (size: number) => {
    if(size < 1024)
      return size.toFixed(2) + 'B';
    else if(size < 1048576)
      return (size / 1024).toFixed(2) + 'KiB';
    else
      return (size / 1048576).toFixed(2) + 'MiB';
  }
</script>

<template>
  <div>
    <img :src="src" :alt="name" />
    <br/>
    <span>{{ name }}</span>
    <br/>
    <span>{{ convertSize(size) }}</span>
    <br/>

    <span class="el-upload-list__item-actions">
      <span
        class="el-upload-list__item-preview"
        @click="$emit('handlePreview')"
      >
        <el-icon><zoom-in /></el-icon>
      </span>
      <span
        class="el-upload-list__item-preview"
        @click="$emit('handleRemove')"
      >
        <el-icon><Delete /></el-icon>
      </span>
    </span>
  </div>
</template>

<style scoped>
</style>