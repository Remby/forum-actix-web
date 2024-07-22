<template>
  <div class="p-4">
    <el-input v-model="title" placeholder="请输入标题" class="mb-4" />
    <v-md-editor
      v-model="content"
      left-toolbar="undo redo | image"
      :disabled-menus="[]"
      @upload-image="handleUploadImage"
      height="700px"
    />
    <el-button type="primary" @click="submitPost" class="mt-4">提交</el-button>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import axios ,{ API_BASE_URL} from "@/axiosConfig";
import { ElMessage } from "element-plus";

const title = ref("");
const content = ref("");
const upload_urls = ref("");
interface InsertImageFunction {
  (data: { url: string; desc?: string; width?: string; height?: string }): void;
}

const handleUploadImage = async (
  _event: Event,
  insertImage: InsertImageFunction,
  files: FileList
) => {
  const formData = new FormData();
  Array.from(files).forEach((file) => {
    console.log("Adding file:", file);
    formData.append("images", file);
  });

  // 打印 FormData 的内容
  for (let pair of formData.entries()) {
    console.log(pair[0] + ", " + pair[1]);
  }
  try {
    const response = await axios.post("/submit-md-imgs", formData, {
      headers: {
        "Content-Type": "multipart/form-data",
      },
    });
    console.log(response.data);
    upload_urls.value = response.data.urls;
    response.data.urls.forEach((url: string) => {
      console.log(url);
      insertImage({
        url: url.replace("./", `${API_BASE_URL}/`),
        desc: "上传的图片",
      });
    });
  } catch (error) {
    console.error("Failed to upload image:", error);
  }
};

const submitPost = async () => {
  try {
    let response = await axios.post("/submit-md", {
      title: title.value,
      content: content.value,
      images: upload_urls.value.length > 0 ? upload_urls.value : null,
    });
    console.log(response.status);
    if (response.status != 200) {
      ElMessage.error("error");
    } else {
      ElMessage.info("提交成功");
    }
    // title.value = "";
    // content.value = "";
  } catch (error) {
    console.error("Failed to submit post:", error);
  }
};
</script>

<style scoped>
/* Tailwind CSS classes can be used directly in模板 */
</style>
