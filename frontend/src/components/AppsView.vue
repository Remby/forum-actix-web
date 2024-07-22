<template>
  <div>
    <h2 class="text-lg font-bold mb-4">应用内容</h2>
    <div class="grid grid-cols-2 gap-4">
      <el-card
        v-for="app in apps"
        :key="app.name"
        shadow="always"
        @click="navigateTo(app.path)"
        class="cursor-pointer transition duration-300 transform hover:scale-105"
        :style="{ backgroundColor: app.color }"
      >
        <h3 class="text-base font-bold">{{ app.name }}</h3>
        <p>{{ app.description }}</p>
      </el-card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';

const router = useRouter();

const navigateTo = (path: string) => {
  router.push({ name: path });
};

const apps = [
  { name: 'Markdown', description: '点击跳转到Markdown展示页面', path: 'Markdown', color: '#dcd7c9' },
  { name: '3D可视化', description: '3D可视化功能展示', path: 'visualization', color: '#a7adba' },
  { name: '留言板', description: '查看和发布留言', path: 'message-board', color: '#9ca3af' },
  { name: '实时弹幕', description: '实时弹幕展示', path: 'live-barrage', color: '#848484' },
  { name: '听音乐', description: '在线听音乐', path: 'music', color: '#b3b3b3' },
  { name: '一起看视频', description: '在线一起看视频', path: 'video', color: '#8a8a8a' },
  { name: '图库', description: '图片展示', path: 'gallery', color: '#c5c6c7' },
  { name: '管理员系统', description: '系统管理功能', path: 'admin-system', color: '#a9a9a9' },
];
</script>

<style scoped>
</style>
