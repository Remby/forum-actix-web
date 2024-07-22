import axios from 'axios';

const apiClient = axios.create({
  baseURL: '/api',
  headers: {
    'Content-Type': 'application/json',
  },
});

export default {
  async getHello() {
    const response = await apiClient.get('/');
    return response.data;
  },
};
