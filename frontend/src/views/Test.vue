<!-- <template>
    <div>
      <input type="file" @change="uploadFile" />
    </div>
  </template>
  
  <script>
  import axios from 'axios';
  
  export default {
    methods: {
      async uploadFile(event) {
        const file = event.target.files[0];
        const formData = new FormData();
        formData.append('file', file);
  
        try {
          const response = await axios.post('/api/uploads', formData, {
            headers: {
              'Content-Type': 'multipart/form-data',
            },
          });
          console.log(response.data);
        } catch (error) {
          console.error(error);
        }
      },
    },
  };
  </script>
   -->