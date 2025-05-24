<script lang="ts" setup>
  import PicCard from './PicCard.vue'
  import { ref, Ref } from 'vue';
  import { Delete, Picture, Upload } from '@element-plus/icons-vue'

  import type { UploadUserFile } from 'element-plus'

  type UserFile = UploadUserFile;

  const dialogImageUrl = ref('')
  const dialogVisible = ref(false)
  //const disabled = ref(false)
  const files : Ref<FileList | null> = ref(null)
  const fileList = ref(new Array<UserFile>)
  const uid = ref(0)

  const handlePictureCardPreview = (file: UserFile) => {
    dialogImageUrl.value = file.url!
    dialogVisible.value = true
  }
  const handleRemove = (file: UserFile) => {
    let index = fileList.value.indexOf(file);
    fileList.value.splice(index, 1);
  }
  const handleRename = (uid: number, newName: string) => {
    const file = fileList.value.find(f => f.uid === uid)
    if (file) file.name = newName
  }
  const handleChange = async () => {
    const input = document.getElementById("input") as HTMLInputElement;
    files.value = input.files!;
    for (let i = 0; i < files.value.length; i++) {
      let file = files.value[i];

      fileList.value.push({
        name: file.name,
        size: file.size,
        status: 'ready',
        uid: uid.value,
        url: URL.createObjectURL(file),
      });
      uid.value++;
    }
  }
  const handleClear = () => {
    fileList.value.splice(0, fileList.value.length);
  }
</script>

<template>
  <!-- TODO: input做成按钮 以及drag区域 -->

  <el-icon><upload-filled /></el-icon>

  <ul>
    <li v-for="file in fileList" :key="file.uid">
      <pic-card :src="file.url!" :size="file.size!" :name="file.name" :uid="file.uid!"
        @handle-preview="handlePictureCardPreview(file)"
        @handle-remove="handleRemove(file)"
        @handle-rename="handleRename"
      />
    </li>
  </ul>

  <el-button-group>
    <el-button type="danger" :icon="Delete" @click="handleClear">清空</el-button>
    <el-button type="primary" :icon="Upload" color="#17a2b8">上传图片</el-button>
    <el-button type="primary" :icon="Picture" @click="">
      <input multiple type="file" accept="image/*" id="input" title="选择图片" @change="handleChange" />
    </el-button>
  </el-button-group>

  <el-dialog v-model="dialogVisible">
    <img w-full :src="dialogImageUrl" alt="Preview Image" />
  </el-dialog>
</template>

<style lang="less" scoped>
</style>
