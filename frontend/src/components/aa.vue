<template>
    <div>
      hello
      <!-- <SphereWithGrid :users="users" /> -->
      <div v-for="user in users" :key="user.id">
        <img :src="user.avatarUrl" alt="User Avatar">
      </div>
    </div>
  </template>
  
  <script>
  import SphereWithGrid from '../components/sphere.vue'; // 确保路径正确
  
  export default {
    components: {
      SphereWithGrid
    },
    data() {
      return {
        users: [
          { id: 1, avatarUrl: '/assets/sponge.jpg' },
          { id: 2, avatarUrl: '/assets/sponge.jpg' },
          { id: 3, avatarUrl: '/assets/sponge.jpg' },
          { id: 4, avatarUrl: '/assets/sponge.jpg' },
          { id: 5, avatarUrl: '/assets/sponge.jpg' },
          { id: 6, avatarUrl: '/assets/sponge.jpg' },
          { id: 7, avatarUrl: '/assets/sponge.jpg' },
          // 更多用户数据
        ]
      };
    }
  };
  </script>
  
  <style>
  body {
    margin: 0;
  }
  </style>
  