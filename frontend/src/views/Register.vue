<template>
  <div class="flex items-center justify-center min-h-screen bg-gray-100">
    <div class="flex flex-col md:flex-row w-4/5 max-w-4xl bg-white shadow-md relative">
      <div class="w-full md:w-3/5 flex items-center justify-center">
        <img src="/assets/register.jpg" alt="Register Image" class="w-full h-full object-cover" />
      </div>
      <div class="w-full md:w-2/5 p-6 md:p-10 flex flex-col justify-center">
        <div class="flex items-center justify-center mb-1">
          <User class="w-6 h-6 mr-2" />
          <h2 class="text-2xl font-bold">注册</h2>
        </div>
        <el-form @submit.prevent="openModal">
          <el-form-item>
            <el-input v-model="username" placeholder="用户名" @blur="checkUsername">
              <template #prepend>
                <User class="w-6 h-6" />
              </template>
            </el-input>
            <p :class="{
                'text-green-500': usernameError === '用户名可用',
                'text-red-500': usernameError === '用户名已存在'
              }">
              {{ usernameError }}
            </p>
          </el-form-item>
          <el-form-item>
            <el-input v-model="password" type="password" placeholder="密码">
              <template #prepend>
                <Lock class="w-6 h-6" />
              </template>
            </el-input>
          </el-form-item>
          <el-form-item>
            <el-input v-model="confirmPassword" type="password" placeholder="确认密码">
              <template #prepend>
                <Lock class="w-6 h-6" />
              </template>
            </el-input>
          </el-form-item>
          <el-form-item>
            <el-button type="primary" @click="openModal" block>注册</el-button>
          </el-form-item>
        </el-form>
        <div class="text-center mt-4">
          <el-link @click="navigateToLogin" class="block mb-4">已有账号？点击登录</el-link>
        </div>
      </div>
    </div>
    <!-- 注册信息模态框 -->
    <el-dialog v-model="showModal" title="填写详细信息" width="80%" :modal="true" :close-on-click-modal="false">
      <el-form @submit.prevent="submitRegistration">
        <el-form-item label="昵称">
          <el-input v-model="nickname" />
        </el-form-item>
        <el-form-item label="性别">
          <el-select v-model="gender" placeholder="请选择性别">
            <el-option label="男" value="male"></el-option>
            <el-option label="女" value="female"></el-option>
            <el-option label="自定义" value="custom"></el-option>
          </el-select>
        </el-form-item>
        <el-form-item label="生日">
          <el-date-picker v-model="birthdate" type="date" placeholder="选择生日" @change="calculateAge" />
        </el-form-item>
        <el-form-item label="年龄">
          <el-input v-model="age" :readonly="true" />
        </el-form-item>
        <el-form-item label="头像">
          <el-upload 
            list-type="picture-card" 
            :limit="1" 
            :on-exceed="handleAvatarExceed" 
            :auto-upload="false" 
            :file-list="avatarList" 
            @change="handleAvatarChange">
            <i class="el-icon-plus"></i>
          </el-upload>
        </el-form-item>
        <el-form-item label="签名">
          <el-input v-model="bio" type="textarea" />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="submitRegistration">提交</el-button>
          <el-button @click="showModal = false">取消</el-button>
        </el-form-item>
      </el-form>
    </el-dialog>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import axios from '@/axiosConfig';
import { ElMessage } from 'element-plus';
import { User, Lock } from '@element-plus/icons-vue';
import moment from 'moment';

export default defineComponent({
  name: 'Register',
  components: {
    User,
    Lock,
  },
  setup() {
    const username = ref('');
    const password = ref('');
    const confirmPassword = ref('');
    const usernameError = ref('');
    const showModal = ref(false);
    const nickname = ref('');
    const gender = ref('');
    const birthdate = ref('');
    const age = ref(0);
    const bio = ref('');
    const avatarList = ref<any[]>([]);

    const checkUsername = async () => {
      if (!username.value) return;
      try {
        const response = await axios.post('/check-name', { username: username.value });
        if (!response.data.available) {
          usernameError.value = '用户名已存在';
        } else {
          usernameError.value = '用户名可用';
        }
      } catch (error) {
        usernameError.value = '验证用户名时出错，请稍后再试';
      }
    };

    const openModal = () => {
      if (!username.value || !password.value || !confirmPassword.value) {
        ElMessage.error('请填写所有字段');
        return;
      }
      if (password.value !== confirmPassword.value) {
        ElMessage.error('两次输入的密码不一致');
        return;
      }
      if (usernameError.value !== '用户名可用') {
        ElMessage.error('请更换用户名');
        return;
      }
      showModal.value = true;
    };

    const calculateAge = () => {
      if (birthdate.value) {
        const birthYear = new Date(birthdate.value).getFullYear();
        const currentYear = new Date().getFullYear();
        age.value = currentYear - birthYear;
      }
    };

    const handleAvatarChange = (_file: any, fileList: any[]) => {
      avatarList.value = [...fileList];
    };

    const handleAvatarExceed = () => {
      ElMessage.warning('只能上传一张头像');
    };

    const submitRegistration = async () => {
      if (!nickname.value || !gender.value || !birthdate.value || !age.value) {
        ElMessage.error('请填写所有字段');
        return;
      }
      try {
        const avatarFile = avatarList.value[0]?.raw;
        const formData = new FormData();
        formData.append('file', avatarFile);

        const avatarResponse = await axios.post('/upload-avatar', formData, {
          headers: {
            'Content-Type': 'multipart/form-data',
          },
        });

        const response = await axios.post('/register', {
          username: username.value,
          password: password.value,
          nickname: nickname.value,
          gender: gender.value,
          birthdate: moment(birthdate.value).format('YYYY-MM-DD'),
          age: age.value,
          bio: bio.value,
          avatar: avatarResponse.data.url,
        });

        if (response.data.success) {
          ElMessage.success('注册成功');
          window.location.href = '/login';
        } else {
          ElMessage.error(response.data.message);
        }
      } catch (error) {
        ElMessage.error('注册失败，请重试');
      }
    };

    const navigateToLogin = () => {
      window.location.href = '/login';
    };

    return {
      username,
      password,
      confirmPassword,
      usernameError,
      showModal,
      nickname,
      gender,
      birthdate,
      age,
      bio,
      avatarList,
      checkUsername,
      openModal,
      calculateAge,
      handleAvatarChange,
      handleAvatarExceed,
      submitRegistration,
      navigateToLogin,
    };
  },
});
</script>

<style scoped>
/* 使用 Tailwind CSS 类名来实现样式 */
</style>
