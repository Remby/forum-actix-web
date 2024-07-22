import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import ElementPlus from 'element-plus';
import 'element-plus/dist/index.css';
import './index.css'

// 导入 Font Awesome 图标库
import { library } from '@fortawesome/fontawesome-svg-core';
import { faMobileAlt } from '@fortawesome/free-solid-svg-icons';
import { faQq, faWeixin } from '@fortawesome/free-brands-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';

import plugins from './plugins';

import axios from '@/axiosConfig';

axios.defaults.withCredentials = true;

library.add(faMobileAlt, faQq, faWeixin);

const app = createApp(App);

app.use(router);
app.use(ElementPlus);
app.use(plugins)
app.component('font-awesome-icon', FontAwesomeIcon);

app.mount('#app');
