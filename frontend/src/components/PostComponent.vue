<template>
  <div class="max-w-6xl mx-auto bg-white shadow-md rounded-lg p-6 my-4">
    <div class="flex items-center space-x-4 mb-4">
      <el-avatar :src="post.avatar" size="large"></el-avatar>
      <div>
        <h2 class="text-xl font-bold">{{ post.title }}</h2>
        <p class="text-gray-700">{{ post.nickname }}</p>
      </div>
    </div>

    <p class="text-gray-700 mb-4">{{ post.content }}</p>

    <div v-if="post.images && post.images.length" class="mb-4 grid grid-cols-3 gap-4">
      <div v-for="image in post.images" :key="image" class="mb-2">
        <img :src="image" alt="Post Image" class="w-full h-auto object-cover rounded-lg">
      </div>
    </div>

    <div class="flex justify-between items-center mb-4">
      <div class="flex items-center space-x-4">
        <!-- <el-button link @click="likePost" class="text-gray-600 hover:text-blue-500">
          <el-icon :class="{ 'text-red-500': liked, 'text-gray-400': !liked }">
            <Pointer />
          </el-icon>
          <span>{{ likes }}</span>
        </el-button> -->
        <span class=" ml-2 text-sm text-gray-500">
        {{formatDate(post.created_at)}}
        </span>
      </div>
      <el-button  link type="primary"  @click="toggleComments">
        {{ showComments ? '折叠评论' : '展开评论' }}
      </el-button>
    </div>

    <el-divider></el-divider>

    <div v-if="showComments">
      <el-form @submit.prevent="submitComment">
        <el-form-item>
          <el-input v-model="newComment" type="textarea" placeholder="添加评论" rows="2">
          </el-input>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="submitComment">评论</el-button>
        </el-form-item>
      </el-form>

      <div class="mt-4 space-y-4">
        <Comment v-for="comment in comments" :key="comment.comment.id" :comment="comment" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted} from 'vue';
import axios from '@/axiosConfig';
import { ElMessage } from 'element-plus';
// import { Pointer } from '@element-plus/icons-vue';
import Comment from '@/components/Comment.vue';

interface Comment {
  comment: {
    id: number;
    post_id: number;
    user_id: number;
    content: string;
    parent_comment_id?: number | null;
    created_at: string;
    nickname: string;
  };
  children: Comment[];
}

const props = defineProps<{ post: { id: number; title: string; content: string; images?: string[]; likes: number; avatar: string; nickname: string ;created_at:string}; }>();

const { post } = props;
const likes = ref(post.likes);
const liked = ref(false);
const comments = ref<Comment[]>([]);
const newComment = ref('');
const showComments = ref(false);

const toggleComments = async () => {
  showComments.value = !showComments.value;
  if (showComments.value && comments.value.length === 0) {
    await fetchComments();
  }
};

const fetchComments = async () => {
  try {
    const response = await axios.get(`/posts/${post.id}/comments`);
    comments.value = response.data;
    
  } catch (error) {
    ElMessage.error('获取评论失败，请重试');
  }
};

const likePost = async () => {
  if (!liked.value) {
    try {
      const response = await axios.post(`/posts/${post.id}/like`);
      if (response.status === 200) {
        likes.value++;
        liked.value = true;
        ElMessage.success('点赞成功');
      } else {
        ElMessage.error(response.data.message);
      }
    } catch (error) {
      ElMessage.error('点赞失败，请重试');
    }
  } else {
    ElMessage.warning('您已经点赞过了');
  }
};

const submitComment = async () => {
  if (!newComment.value.trim()) {
    ElMessage.error('评论内容不能为空');
    return;
  }

  try {
    const response = await axios.post('/submit-comment', {
      post_id: post.id,
      content: newComment.value,
    });

    const newCommentObj: Comment = {
      comment: {
        id: response.data.id,
        post_id: post.id,
        user_id: response.data.user_id,
        content: newComment.value,
        parent_comment_id: null,
        created_at: new Date().toLocaleString(),
        nickname: response.data.nickname,
      },
      children: [],
    };

    comments.value.push(newCommentObj);

    newComment.value = '';
    ElMessage.success('评论成功');
  } catch (error) {
    ElMessage.error('提交评论失败，请重试');
  }
};

const formatDate = (dateString:string) => {
  const date = new Date(dateString);
  const now = new Date();
  const diff = now.getTime() - date.getTime(); // 差值以毫秒为单位

  // 转换为秒、分钟、小时和天
  const seconds = Math.floor(diff / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);

  // 根据差值返回相应的时间格式
  if (seconds < 60) {
    return `${seconds} 秒前`;
  } else if (minutes < 60) {
    return `${minutes} 分钟前`;
  } else if (hours < 24) {
    return `${hours} 小时前`;
  } else if (days < 7) {
    return `${days} 天前`;
  } else {
    // 直接返回格式化的日期时间
    return date.toLocaleString("zh-CN", { timeZone: "Asia/Shanghai" });
  }
};

onMounted(async () => {
  showComments.value = true;
  await fetchComments();
});
</script>

<style scoped>
/* 使用Tailwind CSS进行样式 */
.text-red-500 {
  color: #f56565;
}
.text-gray-400 {
  color: #cbd5e0;
}
</style>
