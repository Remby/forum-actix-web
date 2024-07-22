<template>
  <div class="border-b pb-2 ml-4">
    <p class="text-gray-700">{{ comment.comment.content }}</p>
    <span class="text-sm text-gray-500"
      >由 {{ comment.comment.nickname }} 发表于
      {{ formatDate(comment.comment.created_at) }}</span
    >
    <el-button link type="primary" size="default" class=" m-0.5" @click="toggleReply">回复</el-button>
    <div v-if="showReplyInput" class="mt-2">
      <el-input
        v-model="replyContent"
        placeholder="输入回复"
        type="textarea"
        rows="2"
      ></el-input>
      <el-button type="primary" size="small" @click="submitReply" class="mt-2"
        >提交</el-button
      >
    </div>
    <div
      v-if="comment.children && comment.children.length"
      class="mt-2 space-y-2"
    >
      <Comment
        v-for="child in comment.children"
        :key="child.comment.id"
        :comment="child"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import axios from "@/axiosConfig";
import { ElMessage } from "element-plus";

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

const props = defineProps<{ comment: Comment }>();

const showReplyInput = ref(false);
const replyContent = ref("");

const toggleReply = () => {
  showReplyInput.value = !showReplyInput.value;
};

const submitReply = async () => {
  if (!replyContent.value.trim()) {
    ElMessage.error("回复内容不能为空");
    return;
  }

  try {
    const response = await axios.post("/reply-comment", {
      post_id: props.comment.comment.post_id,
      content: replyContent.value,
      parent_comment_id: props.comment.comment.id,
    });

    const newComment: Comment = {
      comment: {
        id: response.data.id,
        post_id: props.comment.comment.post_id,
        user_id: response.data.user_id,
        content: replyContent.value,
        parent_comment_id: props.comment.comment.id,
        created_at: new Date().toLocaleString(),
        nickname: response.data.nickname,
      },
      children: [],
    };

    if (!props.comment.children) {
      props.comment.children = [];
    }
    props.comment.children.push(newComment);

    replyContent.value = "";
    showReplyInput.value = false;
    ElMessage.success("回复成功");
  } catch (error) {
    ElMessage.error("提交回复失败，请重试");
  }
};

const formatDate = (dateString: string) => {
  console.log(dateString)
  const date = new Date(dateString);
  return date.toLocaleString("zh-CN", { timeZone: "Asia/Shanghai" });
};
</script>

<style scoped>
/* 使用Tailwind CSS进行样式 */
</style>
