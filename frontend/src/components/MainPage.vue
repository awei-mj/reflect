<script lang="ts" setup>
  import PicCard from './PicCard.vue'
  import { ref, Ref } from 'vue';
  import { UploadFilled } from '@element-plus/icons-vue'
  import { Md5 } from 'ts-md5'

  import type { UploadUserFile } from 'element-plus'

  type UserFile = UploadUserFile & {md5: string};

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
  const handleChange = async () => {
    const input = document.getElementById("input") as HTMLInputElement;
    files.value = input.files!;
    for (let i = 0; i < files.value.length; i++) {
      let file = files.value[i];
      let md5 = Md5.hashStr(await file.text())
      // 判断重复
      if(fileList.value.every((file)=>{
        return file.md5 != md5
      })) {
        fileList.value.push({
          name: file.name,
          size: file.size,
          status: 'ready',
          uid: uid.value,
          url: URL.createObjectURL(file),
          md5: md5
        });
        uid.value++;
      }
    }
  }
  const handleClear = () => {
    fileList.value.splice(0, fileList.value.length);
  }
</script>

<template>
  <!-- TODO: input做成按钮 以及drag区域 -->
  <el-icon><upload-filled /></el-icon>
  <input multiple type="file" accept="image/*" id="input" @change="handleChange"/>

  <ul>
    <li v-for="file in fileList">
      <pic-card :src="file.url!" :size="file.size!" :name="file.name" @handle-preview="handlePictureCardPreview(file)" @handle-remove="handleRemove(file)"/>
      <span>{{ file.md5 }}</span>
    </li>
  </ul>

  <el-button @click="handleClear">清空</el-button>
  <el-button>上传图片</el-button>
  <el-button @click="">选择图片</el-button>

  <el-dialog v-model="dialogVisible">
    <img w-full :src="dialogImageUrl" alt="Preview Image" />
  </el-dialog>
</template>

<style scoped>
</style>
