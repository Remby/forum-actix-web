import { createRouter, createWebHistory } from 'vue-router';
import Home from '@/views/Home.vue';
import Login from '@/views/Login.vue';
import Register from '@/views/Register.vue';
import  UserProfile from '@/views/UserProfile.vue';
import MarkdownEditor from '@/components/MarkdownEditor.vue';
import Markdown from '@/views/Markdown.vue';
import axios from '@/axiosConfig';
// import Three from '@/views/Three.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home,
  },
  {
    path: '/login',
    name: 'Login',
    component: Login,
  },
    {
    path: '/register',
    name: 'Register',
    component: Register,
  },
    {
    path: '/user',
    name: 'UserProfile',
    component: UserProfile,
  },
    {
    path: '/markdown-editor',
    name: 'MarkdownEditor',
    component: MarkdownEditor,
  },
  {
    path: '/markdown',
    name: 'Markdown',
    component: Markdown,
  }
  //   {
  //   path: '/three',
  //   name: 'Three',
  //   component: Three,
  // },
  
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

router.beforeEach(async (to, _from, next) => {
    try {
    const response = await axios.get('/auth');
    console.log(response.status);

    // Allow access to login and register routes without authentication
    if (to.name === 'Login' || to.name === 'Register') {
      next();
    } else if (!response.data.auth) {
      next({ name: 'Login' });
    } else {
      next();
    }
  } catch (error) {
    console.error("Network Error:", error);

    // Allow access to login and register routes even if there is a network error
    if (to.name === 'Login' || to.name === 'Register') {
      next();
    } else {
      next({ name: 'Login' });
    }
  }
});


export default router;