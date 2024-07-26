<template>
  <div class="flex h-screen">
    <!-- Sidebar with user list and room options -->
    <aside class="w-1/4 bg-gray-800 text-white p-4">
      <el-menu :default-active="activeMenu" @select="handleSelect">
        <el-menu-item index="1">Public Room</el-menu-item>
        <el-menu-item index="2">Private Chat</el-menu-item>
      </el-menu>
      <div class="mt-4">
        <h3 class="text-lg">Users</h3>
        <ul>
          <li v-for="user in users" :key="user.id">{{ user.name }}</li>
        </ul>
      </div>
    </aside>
    <!-- Main chat area -->
    <main class="w-3/4 p-4 flex flex-col">
      <div class="flex-1 overflow-y-auto bg-white p-4 border rounded">
        <div v-for="msg in messages"  class="mb-2">
            {{msg }}
          <!-- <strong>{{ msg.from }}:</strong> {{ msg.content }} -->
        </div>
      </div>
      <div class="mt-4">
        <el-input
          v-model="newMessage"
          placeholder="Type your message"
          @keyup.enter="sendMessage"
        />
        <el-button type="primary" class="mt-2" @click="sendMessage">
          Send
        </el-button>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { ElMessage } from "element-plus";

// 定义用户接口
interface User {
  id: number;
  name: string;
}

// 定义消息接口
interface ChatMessage {
//   id: number;
//   from: string;
  content: string;
}

// 声明并初始化状态
const activeMenu = ref("1");
const users = ref<User[]>([]);
const messages = ref<string[]>([]);
const newMessage = ref("");

// 创建 WebSocket 连接
const socket = new WebSocket("ws://localhost:8080/ws");

// WebSocket 事件监听
socket.onopen = () => {
  console.log("Connected to server");
};

socket.onmessage = (event) => {
  console.log(event.data);
  const message = event.data;
  messages.value.push(message);
  if (message.type === "userList") {
    users.value = message.data;
  } else if (message.type === "chatMessage") {
    messages.value.push(message.data);
  }
};

socket.onclose = () => {
  console.log("Disconnected from server");
};

socket.onerror = (error) => {
  console.error("WebSocket error:", error);
};

// 处理菜单选择
const handleSelect = (index: string) => {
  console.log(`Selected menu item: ${index}`);
  if (index === "1") {
    ElMessage.info("Public room selected.");
    // 逻辑：加入公共聊天室
  } else if (index === "2") {
    ElMessage.info("Private chat selected.");
    // 逻辑：私人聊天
  }
};

// 发送消息
const sendMessage = () => {
  if (newMessage.value.trim() !== "") {
    const message = {
      type: "chatMessage",
      from: "User1", // 替换为实际的用户名
      content: newMessage.value,
    };
    socket.send(newMessage.value);
    newMessage.value = "";
  }
};
</script>

<style scoped>
/* 自定义样式 */
</style>
