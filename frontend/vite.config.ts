import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import path from 'path';
import prismjs from 'vite-plugin-prismjs';

export default defineConfig({
  base: './',
  optimizeDeps: {
    include: ['@kangc/v-md-editor/lib/theme/vuepress.js'],
  },
  plugins: [vue(),
      prismjs({
      languages: ['json', 'rust', 'html', 'c','cpp','java','python'],
    }),

  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
    },
  },
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:8080', // 确保这是您的后端服务地址
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ''),
        timeout: 60000, // 设置代理请求超时时间为60秒,
      },
    },
  },
});
