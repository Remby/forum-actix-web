<template>
  <div class="flex justify-center items-center min-h-screen bg-gray-100">
    <div v-if="user" class="w-full max-w-4xl">
      <div class="flex items-center justify-between bg-white p-4 shadow-lg rounded-t-lg">
        <img :src="getAvatarUrl(user.avatar)" alt="Avatar" class="w-20 h-20 rounded-full border-2 border-blue-500 shadow-md">
        <div class="ml-4 flex-1">
          <h2 class="text-3xl font-bold">{{ user.nickname }}</h2>
          <p class="text-sm text-gray-500">@{{ user.username }}</p>
        </div>
        <el-button type="primary" class="h-10">编辑资料</el-button>
      </div>
      <div class="bg-white p-4 shadow-lg rounded-b-lg mt-4">
        <div class="text-left">
          <p class="text-lg"><strong>性别:</strong> {{ user.gender }}</p>
          <p class="text-lg"><strong>生日:</strong> {{ user.birthdate }}</p>
          <p class="text-lg"><strong>年龄:</strong> {{ user.age }}</p>
          <p class="text-lg"><strong>签名:</strong> {{ user.bio }}</p>
        </div>
      </div>
    </div>
    <div v-else class="text-center text-gray-500 p-4">加载中...</div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import axios, {API_BASE_URL} from '@/axiosConfig';

interface User {
  id: number;
  username: string;
  nickname: string;
  gender: string | null;
  birthdate: string | null;
  age: number | null;
  avatar: string | null;
  bio: string | null;
}

export default defineComponent({
  name: 'UserProfile',
  setup() {
    const user = ref<User | null>(null);

    onMounted(async () => {
      try {
        const response = await axios.get('/user');
        user.value = response.data;
      } catch (error) {
        console.error("Error fetching user data:", error);
      }
    });

    const getAvatarUrl = (avatarPath: string | null) => {
      if (avatarPath) {
        return `${API_BASE_URL}/${avatarPath}`;
      }
      return '';
    };

    return {
      user,
      getAvatarUrl,
    };
  },
});
</script>

<style scoped>
</style>
