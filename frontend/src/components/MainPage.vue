<script lang="ts" setup>
  import { ref, watch, Ref } from 'vue';
  import { Delete, ZoomIn, UploadFilled } from '@element-plus/icons-vue'

  import type { UploadFile, UploadUserFile } from 'element-plus'

  const dialogImageUrl = ref('')
  const dialogVisible = ref(false)
  //const disabled = ref(false)
  const files : Ref<FileList | null> = ref(null)
  const fileList = ref(new Array<UploadUserFile>)
  const handlePictureCardPreview = (file: UploadFile) => {
    dialogImageUrl.value = file.url!
    dialogVisible.value = true
  }
  const handleRemove = (file: UploadFile) => {
    let index = fileList.value.indexOf(file);
    fileList.value.splice(index, 1);
  }
  const getFileSize = (file: UploadFile) => {
    if(file.size !== undefined) {
      if(file.size < 1024)
        return file.size.toFixed(2) + 'B';
      else if(file.size < 1048576)
        return (file.size / 1024).toFixed(2) + 'KiB';
      else
        return (file.size / 1048576).toFixed(2) + 'MiB';
    }
    return undefined;
  }
  const handleChange = () => {
    const input = document.getElementById("input") as HTMLInputElement;
    files.value = input.files!;
    fileList.value.splice(0, fileList.value.length);
    for (let i = 0; i < files.value.length; i++) {
      let file = files.value[i];
      fileList.value.push({
        name: file.name,
        size: file.size,
        status: 'ready',
        uid: i,
        url: URL.createObjectURL(file)
      })
    }
  }
  watch(files, (newList) => {
    (document.getElementById("input") as HTMLInputElement).files = newList
    fileList.value.splice(0, fileList.value.length);
    for (let i = 0; i < files.value!.length; i++) {
      let file = files.value![i];
      fileList.value.push({
        name: file.name,
        size: file.size,
        status: 'ready',
        uid: i,
        url: URL.createObjectURL(file)
      })
    }
  })
</script>

<template>
  <input multiple type="file" accept="image/*" id="input" @change="handleChange"/>

  <ul>
    <li v-for="file in fileList">
      <img :src="file.url" :alt="file.name" />
    </li>
  </ul>

  <!-- <el-upload drag multiple action="#" list-type="picture" v-model:file-list="fileList" :auto-upload="false">
    <el-icon class="el-icon--upload"><upload-filled /></el-icon>
    <div class="el-upload__text">
      Drop file here or <em>click to upload</em>
    </div>

    <template #file="{ file }">
      <div style="width: 500px;">
        <img class="el-upload-list__item-thumbnail" :src="file.url" alt="" />
        <span class="el-upload-list__item-thumbnail">{{ getFileSize(file) }}</span>
        <span class="el-upload-list__item-thumbnail">{{ file.name }}</span>
        <span class="el-upload-list__item-actions">
          <span
            class="el-upload-list__item-preview"
            @click="handlePictureCardPreview(file)"
          >
            <el-icon><zoom-in /></el-icon>
          </span>
          <span
            v-if="!disabled"
            class="el-upload-list__item-preview"
            @click="handleRemove(file)"
          >
            <el-icon><Delete /></el-icon>
          </span>
        </span>
      </div>
    </template>
  </el-upload> -->

  <el-button>上传图片</el-button>

  <el-dialog v-model="dialogVisible">
    <img w-full :src="dialogImageUrl" alt="Preview Image" />
  </el-dialog>
</template>

<style scoped>
</style>
