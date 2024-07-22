module.exports = {
    devServer: {
      // host: '0.0.0.0',
      // allowedHosts: 'all',
      proxy: {
        '/api': {
          target: 'http://localhost:8080',
          changeOrigin: true,
          pathRewrite: { '^/api': '/' },
          timeout: 60000, // 设置代理请求超时时间为60秒
        },
      },
    },
  }
  