<template>
  <div>
    <div class="flex justify-between items-center p-4 bg-gray-100 shadow-md">
      <div class="flex items-center">
        <el-avatar :src="user.avatar" size="small"></el-avatar>
        <span class="ml-2">{{ user.nickname }}</span>
      </div>
      <router-link to="/markdown-editor">
        <el-button type="primary">发表</el-button>
      </router-link>
    </div>

    <div class="mt-4 px-4">
      <el-tabs v-model="activeTab">
        <el-tab-pane label="广场上的文章" name="square">
          <div class="grid grid-cols-1 gap-4">
            <el-card v-for="article in sortedSquareArticles" :key="article.id" class="min-h-32 transition duration-200 ease-in-out transform hover:shadow-lg hover:bg-gray-200">
              <div class="flex">
                <div class="flex-1">
                  <h3 class="text-lg font-bold">{{ article.title }}</h3>
                  <span class="text-sm text-gray-500">{{ formatDate(article.created_at) }}</span>
                  <div v-if="article.isExpanded">
                    <v-md-preview :text="article.content"></v-md-preview>
                  </div>
                  <div v-else>
                    <p>{{ article.preview }}</p>
                  </div>
                  <div class="text-right">
                    <el-button type="text" @click="togglePreview(article)">
                      {{ article.isExpanded ? '折叠' : '查看全文' }}
                    </el-button>
                  </div>
                </div>
              </div>
            </el-card>
          </div>
        </el-tab-pane>
        <el-tab-pane label="我的文章" name="myArticles">
          <div class="grid grid-cols-1 gap-4">
            <el-card v-for="article in sortedMyArticles" :key="article.id" class="min-h-32 transition duration-200 ease-in-out transform hover:shadow-lg hover:bg-gray-200">
              <div class="flex">
                <div class="flex-1">
                  <h3 class="text-lg font-bold">{{ article.title }}</h3>
                  <span class="text-sm text-gray-500">{{ formatDate(article.created_at) }}</span>
                  <div v-if="article.isExpanded">
                    <v-md-preview :text="article.content"></v-md-preview>
                  </div>
                  <div v-else>
                    <p>{{ article.preview }}</p>
                  </div>
                  <div class="text-right">
                    <el-button type="text" @click="togglePreview(article)">
                      {{ article.isExpanded ? '折叠' : '查看全文' }}
                    </el-button>
                  </div>
                </div>
              </div>
            </el-card>
          </div>
        </el-tab-pane>
      </el-tabs>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import axios from '@/axiosConfig';
import { API_BASE_URL } from '@/axiosConfig';

interface Article {
  id: number;
  title: string;
  content: string;
  thumbnail: string;
  preview: string;
  created_at: string;
  isExpanded?: boolean;
}

const user = ref({
  nickname: '',
  avatar: ''
});

const activeTab = ref('square');
const squareArticles = ref<Article[]>([]);
const myArticles = ref<Article[]>([]);

const fetchUser = async () => {
  try {
    const response = await axios.get(`/get-user-name-avatar`);
    response.data.avatar = response.data.avatar.replace('./', `${API_BASE_URL}/`);
    user.value = response.data;
  } catch (error) {
    console.error('Failed to fetch user:', error);
  }
};

const fetchSquareArticles = async () => {
  const response = await axios.get('/square-articles');
  squareArticles.value = response.data.map((article: any) => ({
    ...article,
    preview: article.content.slice(0, 60) + '...',
    isExpanded: false
  }));
};

const fetchMyArticles = async () => {
  const response = await axios.get('/my-articles');
  myArticles.value = response.data.map((article: any) => ({
    ...article,
    preview: article.content.slice(0, 100) + '...',
    isExpanded: false
  }));
};

const togglePreview = (article: Article) => {
  article.isExpanded = !article.isExpanded;
};

const formatDate = (dateString: string) => {
  const date = new Date(dateString);
  return date.toLocaleString("zh-CN", { timeZone: "Asia/Shanghai" });
};

const sortedSquareArticles = computed(() => {
  return squareArticles.value.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime());
});

const sortedMyArticles = computed(() => {
  return myArticles.value.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime());
});

onMounted(() => {
  fetchUser();
  fetchSquareArticles();
  fetchMyArticles();
});
</script>

<style scoped></style>
