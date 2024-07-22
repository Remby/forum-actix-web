<template>
  <div class="flex items-center justify-center min-h-screen bg-gray-100">
    <div class="flex flex-col md:flex-row w-full max-w-4xl bg-white shadow-md">
      <div class="w-full md:w-1/2">
        <img src="/assets/sponge.jpg" alt="Car Image" class="w-full h-full object-cover" />
      </div>
      <div class="p-6 md:p-10 flex flex-col justify-center w-full md:w-1/2">
        <div class="flex items-center justify-center mb-4">
          <User class="w-6 h-6 mr-2" />
          <h2 class="text-2xl font-bold">登录</h2>
        </div>
        <el-form @submit.prevent="login">
          <el-form-item>
            <el-input v-model="username" placeholder="用户名">
              <template #prepend>
                <User />
              </template>
            </el-input>
          </el-form-item>
          <el-form-item>
            <el-input v-model="password" type="password" placeholder="登录密码">
              <template #prepend>
                <Lock />
              </template>
            </el-input>
          </el-form-item>
          <el-form-item>
            <div class="flex items-center">
              <el-input v-model="captchaInput" placeholder="验证码">
                <template #prepend>
                  <Key />
                </template>
              </el-input>
              <span class="ml-4 p-2 bg-gray-200 cursor-pointer select-none" @click="refreshCaptcha">{{ captchaCode }}</span>
            </div>
          </el-form-item>
          <el-form-item>
            <el-checkbox v-model="rememberMe">记住密码</el-checkbox>
          </el-form-item>
          <el-form-item>
            <el-button type="primary" @click="login" block>登录</el-button>
          </el-form-item>
        </el-form>
        <div class="text-center mt-4">
          <el-link @click="forgotPassword" class="block mb-4">忘记了密码？</el-link>
          <div class="flex justify-center mb-4">
            <el-button circle class="mx-2"><font-awesome-icon :icon="['fas', 'mobile-alt']" /></el-button>
            <el-button circle class="mx-2"><font-awesome-icon :icon="['fab', 'weixin']" /></el-button>
            <el-button circle class="mx-2"><font-awesome-icon :icon="['fab', 'qq']" /></el-button>
          </div>
          <p>没有账号？ <el-link @click="signup" class="font-medium text-blue-500">点击注册</el-link></p>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted } from 'vue';
import axios from '@/axiosConfig';
import { ElMessage } from 'element-plus';
import { User, Lock, Key } from '@element-plus/icons-vue';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
import { generateCaptcha } from '@/utils/captcha';

export default defineComponent({
  name: 'Login',
  components: {
    User,
    Lock,
    Key,
    FontAwesomeIcon,
  },
  data() {
    return {
      username: '',
      password: '',
      rememberMe: false,
      captchaCode: generateCaptcha(),
      captchaInput: '',
    };
  },
  methods: {
    async login() {
      if (!this.username || !this.password || !this.captchaInput) {
        ElMessage.error('请填写所有字段');
        return;
      }
      if (this.captchaInput.toUpperCase() !== this.captchaCode) {
        ElMessage.error('验证码不正确');
        this.refreshCaptcha();
        return;
      }

      try {
        const response = await axios.post('/login', {
          username: this.username,
          password: this.password,
        });

        if (response.status === 200) {
          if (this.rememberMe) {
            localStorage.setItem('username', this.username);
            localStorage.setItem('password', this.password);
          } else {
            localStorage.removeItem('username');
            localStorage.removeItem('password');
          }
          ElMessage.info("now to home")
          window.location.href = '/'
        } else {
          ElMessage.error(response.data.message);
        }
      } catch (error) {
        ElMessage.error('登录失败，请重试');
      }
    },
    refreshCaptcha() {
      this.captchaCode = generateCaptcha();
    },
    forgotPassword() {
      ElMessage.info('忘记密码功能尚未实现');
    },
    signup() {
      window.location.href = '/register'
    },
    loadRememberedCredentials() {
      const rememberedUsername = localStorage.getItem('username');
      const rememberedPassword = localStorage.getItem('password');
      if (rememberedUsername && rememberedPassword) {
        this.username = rememberedUsername;
        this.password = rememberedPassword;
        this.rememberMe = true;
      }
    }
  },
  mounted() {
    this.loadRememberedCredentials();
  }
});
</script>

<style scoped>
/* 已不再需要自定义 CSS，全都用 Tailwind CSS 类名来实现 */
</style>
