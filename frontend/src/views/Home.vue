<template>
  <div class="flex flex-col max-h-screen overflow-hidden">
    <!-- Top Navigation Bar -->
    <div
      class="bg-white shadow-md flex justify-between items-center px-4 py-2 md:px-6"
    >
      <div class="flex items-center">
        <el-avatar src="/assets/sponge.jpg" size="small"></el-avatar>
        <h1 class="ml-2 md:ml-4 text-base md:text-lg font-bold">论坛</h1>
      </div>
      <div class="flex-grow max-w-2xl mx-4">
        <el-input placeholder="搜索" class="w-full"></el-input>
      </div>
      <div class="flex items-center space-x-2 md:space-x-4">
        <el-button link>
          <el-icon><ChatDotSquare /></el-icon>
        </el-button>
        <el-button link>
          <el-icon><Setting /></el-icon>
        </el-button>
        <el-button link @click="toggleFullScreen">
          <h1 v-if="isFullScreen">
            <el-icon><Open /></el-icon>
          </h1>
          <h1 v-else>
            <el-icon><TurnOff /></el-icon>
          </h1>
        </el-button>
      </div>
    </div>

    <div class="flex flex-1 overflow-hidden">
      <!-- Sidebar -->
      <aside
        v-if="!isFullScreen"
        class="bg-gray-100 w-20 md:w-64 py-4 overflow-y-auto scrollbar-hide flex flex-col justify-between"
      >
        <nav>
          <ul>
            <li class="mb-4 px-2 md:px-6">
              <el-button
                class="w-full flex flex-col md:flex-row items-center justify-center md:justify-start"
                @click="setView('SquareView')"
              >
                <el-icon><User /></el-icon>
                <span class="ml-0 md:ml-4 mt-2 md:mt-0 text-xs md:text-base"
                  >广场</span
                >
              </el-button>
            </li>
            <li class="mb-4 px-2 md:px-6">
              <el-button
                class="w-full flex flex-col md:flex-row items-center justify-center md:justify-start"
                @click="setView('FriendsView')"
              >
                <el-icon><User /></el-icon>
                <span class="ml-0 md:ml-4 mt-2 md:mt-0 text-xs md:text-base"
                  >朋友圈</span
                >
              </el-button>
            </li>
            <li class="mb-4 px-2 md:px-6">
              <el-button
                class="w-full flex flex-col md:flex-row items-center justify-center md:justify-start"
                @click="setView('AppsView')"
              >
                <el-icon><Star /></el-icon>
                <span class="ml-0 md:ml-4 mt-2 md:mt-0 text-xs md:text-base"
                  >应用</span
                >
              </el-button>
            </li>
            <li class="mb-4 px-2 md:px-6">
              <el-button
                class="w-full flex flex-col md:flex-row items-center justify-center md:justify-start"
                @click="setView('SettingsView')"
              >
                <el-icon><Star /></el-icon>
                <span class="ml-0 md:ml-4 mt-2 md:mt-0 text-xs md:text-base"
                  >设置</span
                >
              </el-button>
            </li>
          </ul>
        </nav>

        <!-- Right Sidebar content for mobile -->
        <div class="block md:hidden mt-8 px-2 md:px-6">
          <el-collapse>
            <el-collapse-item title="联系人" name="contact">
              <ul>
                <li class="mb-4 flex items-center">
                  <el-avatar
                    src="https://example.com/contact1.jpg"
                    size="small"
                  ></el-avatar>
                  <span class="ml-2 text-xs md:text-base">联系人1</span>
                </li>
              </ul>
            </el-collapse-item>
            <el-collapse-item title="群组" name="groups">
              <ul>
                <li class="mb-4 flex items-center">
                  <el-avatar
                    src="https://example.com/group1.jpg"
                    size="small"
                  ></el-avatar>
                  <span class="ml-2 text-xs md:text-base">群组1</span>
                </li>
              </ul>
            </el-collapse-item>
          </el-collapse>
        </div>
      </aside>

      <!-- Main Content Area -->
      <main class="flex-1 p-4 md:p-6 overflow-y-auto scrollbar-hide">
        <component
          :is="currentViewComponent"
          :posts="posts"
          @open-create-post-dialog="openCreatePostDialog"
        ></component>
        <el-button
          @click="loadMorePosts"
          type="primary"
          v-if="currentView === 'SquareView'"
          >加载更多</el-button
        >
      </main>

      <!-- Right Sidebar -->
      <aside
        v-if="!isFullScreen"
        class="bg-gray-100 w-20 md:w-64 py-4 px-2 md:px-6 overflow-y-auto scrollbar-hide hidden md:block"
      >
        <el-input
          v-model="searchQuery"
          placeholder="搜索联系人"
          @input="searchContacts"
          class="w-full mb-4"
        ></el-input>

        <ul v-if="searchResults.length > 0">
          <li
            v-for="user in searchResults"
            :key="user.username"
            class="flex items-center mb-4 p-2 border rounded-lg bg-white shadow-sm"
          >
            <el-avatar :src="user.avatar" size="default" class="mr-3"></el-avatar>
            <div class="flex-grow">
              <div class="font-semibold text-l">{{ user.username }}</div>
              <div class="text-gray-500 text-sm">{{ user.nickname }}</div>
            </div>
            <el-button
              type="primary"
              @click="addContact(user.username)"
              class="ml-4" size="default"
            >
              添加
            </el-button>
          </li>
        </ul>

        <el-dialog
          title="添加联系人"
          v-model="showAddContactDialog"
          width="30%"
          :visible.sync="showAddContactDialog"
        >
          <el-input
            v-model="newContactName"
            placeholder="输入联系人名字"
          ></el-input>
          <span slot="footer" class="dialog-footer">
            <el-button @click="showAddContactDialog = false">取消</el-button>
            <el-button type="primary" @click="confirmAddContact"
              >确定</el-button
            >
          </span>
        </el-dialog>

        <el-collapse>
          <el-collapse-item>
            <template #title>
              <el-icon class="mr-1"><User /></el-icon>联系人
            </template>
            <ul>
              <li class="mb-4 flex items-center">
                <el-avatar
                  src="https://example.com/contact1.jpg"
                  size="small"
                ></el-avatar>
                <span class="ml-2 text-xs md:text-base">联系人1</span>
              </li>
            </ul>
          </el-collapse-item>
          <el-collapse-item title="群组" name="groups">
            <ul>
              <li class="mb-4 flex items-center">
                <el-avatar
                  src="https://example.com/group1.jpg"
                  size="small"
                ></el-avatar>
                <span class="ml-2 text-xs md:text-base">群组1</span>
              </li>
            </ul>
          </el-collapse-item>
        </el-collapse>
      </aside>
    </div>

    <!-- Create Post Dialog -->
    <el-dialog
      title="创建帖子"
      v-model="showCreatePostDialog"
      width="90%"
      :visible.sync="showCreatePostDialog"
    >
      <CreatePost
        :visible="showCreatePostDialog"
        @close="closeCreatePostDialog"
      />
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import {
  ElButton,
  ElInput,
  ElAvatar,
  ElIcon,
  ElDialog,
  ElMessage,
  ElCollapse,
  ElCollapseItem,
  ElNotification,
} from "element-plus";
import {
  User,
  Star,
  ChatDotSquare,
  Setting,
  Open,
  TurnOff,
} from "@element-plus/icons-vue";
import SquareView from "@/components/SquareView.vue";
import FriendsView from "@/components/FriendsView.vue";
import AppsView from "@/components/AppsView.vue";
import SettingsView from "@/components/SettingsView.vue";
import CreatePost from "@/components/CreatePost.vue";
import axios, { API_BASE_URL } from "@/axiosConfig";
import { h } from "vue";
interface Post {
  id: number;
  user_id: number;
  nickname: string;
  avatar: string;
  title: string;
  content: string;
  images?: string[];
  likes: number;
}

const showCreatePostDialog = ref(false);
const posts = ref<Post[]>([]);
const currentView = ref("SquareView");
const isFullScreen = ref(false);

const viewComponents = {
  SquareView,
  FriendsView,
  AppsView,
  SettingsView,
};

const searchQuery = ref("");
const searchResults = ref([]);
const showAddContactDialog = ref(false);
const newContactName = ref("");

const currentViewComponent = computed(
  () => viewComponents[currentView.value as keyof typeof viewComponents]
);

const setView = (view: string) => {
  currentView.value = view;
};

const openCreatePostDialog = () => {
  showCreatePostDialog.value = true;
};

const closeCreatePostDialog = () => {
  showCreatePostDialog.value = false;
};

const loadPosts = async () => {
  try {
    const response = await axios.get("/posts", {
      params: { limit: 20 },
    });

    response.data.posts.map((post: Post) => {
      if (post.avatar) {
        post.avatar = post.avatar.replace("./", `${API_BASE_URL}/`);
      }
      if (post.images) {
        post.images = post.images.map((image) => {
          return image.replace("./", `${API_BASE_URL}/`);
        });
      }
      return post;
    });
    posts.value = response.data.posts;
  } catch (error) {
    console.error("Error loading posts:", error);
  }
};

const loadMorePosts = async () => {
  try {
    const response = await axios.get("/posts", {
      params: { limit: 20, offset: posts.value.length },
    });
    posts.value.push(...response.data.posts);
  } catch (error) {
    console.error("Error loading more posts:", error);
  }
};

const toggleFullScreen = () => {
  isFullScreen.value = !isFullScreen.value;
  ElNotification({
    title: isFullScreen.value ? "╭(●｀∀´●)╯" : "╰(●’◡’●)╮",
    position: "top-left",
    message: h(
      "i",
      { style: "color:teal" },
      isFullScreen.value
        ? "您已经关闭了边栏，再次点击打开"
        : "您已经打开了边栏，再次点击关闭"
    ),

    duration: 2000,
  });
};

const searchContacts = async () => {
  try {
    const response = await axios.get(`/search/users`, {
      params: { username: searchQuery.value },
    });
    if (response.data) {
      response.data.map(
        (info: any) =>
          (info.avatar = info.avatar.replace("./", `${API_BASE_URL}/`))
      );
    }
    searchResults.value = response.data;
    console.log(response.data);
  } catch (error) {
    ElMessage.error("搜索失败，请稍后再试");
    console.error(error);
  }
};

const addContact = async (contactId: number) => {
  try {
    const response = await axios.post("/contacts", { contactId });
    if (response.data.success) {
      ElMessage.success("联系人添加成功");
      // 这里可以刷新联系人列表或者做其他处理
    } else {
      ElMessage.error("添加联系人失败");
    }
  } catch (error) {
    ElMessage.error("添加联系人失败，请稍后再试");
    console.error(error);
  }
};

const confirmAddContact = async () => {
  // 确认添加联系人逻辑
};

onMounted(() => {
  loadPosts();
});
</script>

<style scoped>
/* 使用 Tailwind CSS 类名来实现样式 */
</style>
