<script setup lang="ts">
import MainPage from './components/MainPage.vue'
</script>

<template>
  <div class="common-layout">
    <el-container>
      <el-header>
        <span>載せ</span>
      </el-header>
      <el-main>
        <main-page />
      </el-main>
      <el-footer>Footer</el-footer>
    </el-container>
  </div>
</template>

<style lang="less" scoped>
  .el-main {
    height:1000px;
  }
</style>
