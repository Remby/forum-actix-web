<template>
    <div>
      <div class="bg-white shadow-md p-4 rounded-md mb-6" @click="$emit('open-create-post-dialog')">
        <el-input placeholder="分享你的新鲜事" type="textarea"></el-input>
        <div class="flex justify-end mt-2">
          <el-button type="primary">发布</el-button>
        </div>
      </div>
  
      <!-- Posts -->
      <div v-for="post in posts" :key="post.id" class="bg-white shadow-md p-4 rounded-md mb-6">
        <PostComponent :post="post"></PostComponent>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import PostComponent from "@/components/PostComponent.vue";
  
  defineProps<{ posts: any[] }>();
  </script>
  