<template>
  <vue-draggable-resizable
    class="border-none" 
    :x="initialX"
    :y="initialY"
    :w="initialWidth"
    :h="500"
    @dragging="onDragging"
    @resizing="onResizing"
  >
    <div class="bg-white shadow-md rounded-lg flex flex-col h-full">
      <div class="bg-gray-100 p-4 flex justify-between items-center">
        <span class="text-lg font-bold">Chat with {{ chatWith }}</span>
        <el-button
          @click="closeChat"
          link
          class="text-gray-500">
          <el-icon><CircleCloseFilled /></el-icon>
        </el-button>
      </div>
      <div class="flex-1 p-4 overflow-y-auto">
        <div v-for="(message, index) in messages" :key="index" class="mb-2">
          <strong>{{ message.user }}:</strong> {{ message.text }}
        </div>
      </div>
      <div class="p-4 border-t border-gray-200 flex">
        <el-input
          v-model="newMessage"
          @keyup.enter="sendMessage"
          placeholder="Type your message..."
          class="flex-grow"
        />
        <el-button type="primary" @click="sendMessage" class="ml-2">Send</el-button>
      </div>
    </div>
  </vue-draggable-resizable>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import VueDraggableResizable from 'vue-draggable-resizable';
import 'vue-draggable-resizable/style.css';

import {
  ElButton,
  ElInput,
  ElIcon,
} from "element-plus";

import {
  CircleCloseFilled
} from "@element-plus/icons-vue";

const props = defineProps<{
  chatWith: string;
  chatId: number;
  initialX: number;
  initialY: number;
  initialWidth: number,
}>();

const emit = defineEmits(['close']);

const messages = ref([
  { user: 'User1', text: 'Hello!' },
  { user: 'User2', text: 'Hi there!' }
]);

const newMessage = ref('');

const closeChat = () => {
  socket.close(1000, "Client requested disconnect");
  emit('close');
};

const sendMessage = () => {
  if (newMessage.value.trim() !== '') {
    messages.value.push({ user: 'You', text: newMessage.value });
    socket.send(newMessage.value);
    newMessage.value = '';
  }
};

const onDragging = (x:number, y:number) => {
  console.log(`Dragging to: ${x}, ${y}`);
};

const onResizing = (w: number, h:number) => {
  console.log(`Resizing to: ${w}, ${h}`);
};


const receiver_username = props.chatWith;
const socket = new WebSocket(`ws://localhost:8080/ws?receiver_username=${receiver_username}`);


socket.onmessage = (event) => {
  console.log(event.data);
};

</script>
