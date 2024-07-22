<template>
  <el-form @submit.prevent="submitPost">
    <el-form-item label="标题">
      <el-input v-model="title" placeholder="请输入标题"></el-input>
    </el-form-item>
    <el-form-item label="内容" class>
      <el-input type="textarea" v-model="content" autosize placeholder="分享你的新鲜事"></el-input>
    </el-form-item>
    <el-form-item label="图片">
      <el-upload
        :file-list="fileList"
        list-type="picture-card"
        :limit="9"
        :auto-upload="false"
        @change="handleAvatarChange"
      >
        <i v-if="fileList.length < 9" class="el-icon-plus"></i>
      </el-upload>
      <el-dialog :visible.sync="dialogVisible">
        <img width="100%" :src="dialogImageUrl" alt="">
      </el-dialog>
    </el-form-item>
    <el-form-item>
      <el-button type="primary" @click="submitPost">发布</el-button>
      <el-button @click="$emit('close')">取消</el-button>
    </el-form-item>
  </el-form>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import axios from '@/axiosConfig';
import { ElMessage, UploadFile } from 'element-plus';

export default defineComponent({
  name: 'CreatePost',
  setup(_, { emit }) {
    const title = ref('');
    const content = ref('');
    const dialogImageUrl = ref('');
    const dialogVisible = ref(false);
    const fileList = ref<UploadFile[]>([]);

    const handleAvatarChange = (_file: UploadFile, fileListArray: UploadFile[]) => {
      fileList.value = [...fileListArray];
    };

    const submitPost = async () => {
      if (!title.value || !content.value) {
        ElMessage.error('请填写所有字段');
        return;
      }

      const formData = new FormData();
      formData.append('title', title.value);
      formData.append('content', content.value);
      fileList.value.forEach((file) => {
        if (file.raw) {
          formData.append('images', file.raw);
        }
      });

      try {
        const response = await axios.post('/submit-post', formData, {
          headers: {
            'Content-Type': 'multipart/form-data',
          },
          withCredentials: true,  // 确保带上cookie
        });
        if (response.data.success) {
          ElMessage.success('发布成功');
          emit('close');
        } else {
          ElMessage.error(response.data.message);
        }
      } catch (error) {
        ElMessage.error('发布失败，请重试');
      }
    };

    return {
      title,
      content,
      dialogImageUrl,
      dialogVisible,
      fileList,
      submitPost,
      handleAvatarChange,
    };
  },
});
</script>

<style scoped>
/* 其他自定义样式 */
</style>
