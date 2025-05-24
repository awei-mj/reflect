<script lang="ts" setup>
  import { ref, defineEmits } from 'vue'
  import { Delete, ZoomIn } from '@element-plus/icons-vue'

  const props = defineProps<{
    src: string,
    name: string,
    size: number,
    uid: number,
  }>()
  const emit = defineEmits(['handlePreview', 'handleRemove', 'handleRename'])

  const width = ref<number | null>(null)
  const height = ref<number | null>(null)
  const editing = ref(false)
  const newName = ref(props.name)

  const convertSize = (size: number) => {
    if(size < 1024)
      return size.toFixed(2) + 'B';
    else if(size < 1048576)
      return (size / 1024).toFixed(2) + 'KiB';
    else
      return (size / 1048576).toFixed(2) + 'MiB';
  }

  const onImgLoad = (e: Event) => {
    const img = e.target as HTMLImageElement
    width.value = img.naturalWidth
    height.value = img.naturalHeight
  }

  const startEdit = () => {
    editing.value = true
    newName.value = props.name
    // 下一个 tick 聚焦输入框
    setTimeout(() => {
      const input = document.getElementById('rename-input') as HTMLInputElement
      input?.focus()
      input?.select()
    }, 0)
  }
  const finishEdit = () => {
    editing.value = false
    if (newName.value !== props.name && newName.value.trim() !== '') {
      emit('handleRename', props.uid, newName.value.trim()) // 传递uid和新名字
    }
  }
</script>

<template>
  <el-card class="pic-card" style="width: 250px; height: 350px; margin: 10px;">
    <el-image
      :src="src"
      :alt="name"
      fit="contain"
      @load="onImgLoad"/>
    <br/>
    <span v-if="!editing" @click="startEdit" style="cursor: pointer;">
      {{ name }}
      <el-icon style="font-size: 14px; margin-left: 4px; vertical-align: middle;"><el-icon-edit /></el-icon>
    </span>
    <input
      v-else
      id="rename-input"
      v-model="newName"
      @keyup.enter="finishEdit"
      @blur="finishEdit"
      style="width: 80%; font-size: 1em;"
    />
    <br/>
    <span>{{ convertSize(size) }}</span>
    <br/>
    <span v-if="width && height">{{ width }} * {{ height }}</span>
    <br/>
    <span class="el-upload-list__item-actions">
      <el-button-group>
        <el-button type="primary" :icon="ZoomIn" @click="$emit('handlePreview')"></el-button>
        <el-button type="danger" :icon="Delete" @click="$emit('handleRemove')"></el-button>
      </el-button-group>
    </span>
  </el-card>
</template>

<style lang="less" scoped>
.el-upload-list__item-actions {
  display: flex;
  gap: 8px;
  margin-top: 8px;
}
input#rename-input {
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  padding: 2px 6px;
}
</style>