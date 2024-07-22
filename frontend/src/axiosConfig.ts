// src/axiosConfig.ts
import axios from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;

axios.defaults.baseURL = API_BASE_URL;
axios.defaults.withCredentials = true; 

export { API_BASE_URL };
export default axios;
