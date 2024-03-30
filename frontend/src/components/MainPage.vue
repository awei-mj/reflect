<script setup lang="ts">
  import { ref } from 'vue'
  import { Delete, ZoomIn, UploadFilled } from '@element-plus/icons-vue'

  import type { UploadFile } from 'element-plus'

  const dialogImageUrl = ref('')
  const dialogVisible = ref(false)
  const disabled = ref(false)
  const fileList = ref()
  const handlePictureCardPreview = (file: UploadFile) => {
    dialogImageUrl.value = file.url!
    dialogVisible.value = true
  }
  const handleRemove = (file: UploadFile) => {
    let index = fileList.value.indexOf(file);
    fileList.value.splice(index, 1);
  }
</script>

<template>
  <el-upload drag multiple action="#" list-type="picture-card" v-model:file-list="fileList" :auto-upload="false">
    <el-icon class="el-icon--upload"><upload-filled /></el-icon>
    <div class="el-upload__text">
      Drop file here or <em>click to upload</em>
    </div>

    <template #file="{ file }">
      <div>
        <img class="el-upload-list__item-thumbnail" :src="file.url" alt="" />
        <span>{{ file.size }}</span>
        <span>{{ file.name }}</span>
        <span class="el-upload-list__item-actions">
          <span
            class="el-upload-list__item-preview"
            @click="handlePictureCardPreview(file)"
          >
            <el-icon><zoom-in /></el-icon>
          </span>
          <span
            v-if="!disabled"
            class="el-upload-list__item-delete"
            @click="handleRemove(file)"
          >
            <el-icon><Delete /></el-icon>
          </span>
        </span>
      </div>
    </template>

  </el-upload>

  <el-dialog v-model="dialogVisible">
    <img w-full :src="dialogImageUrl" alt="Preview Image" />
  </el-dialog>
</template>

<style scoped>
</style>
