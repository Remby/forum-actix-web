<!-- <template>
  <div ref="container" class="sphere-container"></div>
</template>

<script>
import * as THREE from 'three';
import { onMounted, ref } from 'vue';

export default {
  name: 'SphereWithGrid',
  props: {
    users: {
      type: Array,
      required: true
    }
  },
  setup(props) {
    const container = ref(null);
    let isDragging = false;
    let previousMousePosition = { x: 0, y: 0 };
    let rotationSpeed = 0.01;

    onMounted(() => {
      const scene = new THREE.Scene();
      const camera = new THREE.PerspectiveCamera(75, container.value.clientWidth / container.value.clientHeight, 0.1, 1000);
      const renderer = new THREE.WebGLRenderer({ alpha: true });
      renderer.setSize(container.value.clientWidth, container.value.clientHeight);
      container.value.appendChild(renderer.domElement);

      // 创建带有经纬线的透明球体
      const geometry = new THREE.SphereGeometry(5, 32, 32);
      const wireframe = new THREE.WireframeGeometry(geometry);
      const lineMaterial = new THREE.LineBasicMaterial({ color: 0xffffff, opacity: 0.5, transparent: true });
      const wireframeMesh = new THREE.LineSegments(wireframe, lineMaterial);
      scene.add(wireframeMesh);

      // 添加用户头像
      const textureLoader = new THREE.TextureLoader();
      const avatars = new THREE.Group(); // 创建一个组来包含所有头像
      props.users.forEach(user => {
        textureLoader.load(user.avatarUrl, (texture) => {
          const avatarMaterial = new THREE.MeshBasicMaterial({ map: texture, side: THREE.DoubleSide });
          const avatarGeometry = new THREE.PlaneGeometry(1, 1);
          const avatarMesh = new THREE.Mesh(avatarGeometry, avatarMaterial);

          // 随机分布用户头像在球体上
          const phi = Math.acos(2 * Math.random() - 1);
          const theta = 2 * Math.PI * Math.random();

          avatarMesh.position.set(
            5 * Math.sin(phi) * Math.cos(theta),
            5 * Math.sin(phi) * Math.sin(theta),
            5 * Math.cos(phi)
          );

          avatarMesh.lookAt(0, 0, 0);
          avatarMesh.userData = { userId: user.id };
          avatars.add(avatarMesh); // 将头像添加到组中
        });
      });
      scene.add(avatars); // 将组添加到场景中

      // 设置相机位置
      camera.position.z = 15;
      camera.position.y = 5; // 调整相机位置，使球体稍微倾斜
      camera.lookAt(0, 0, 0);

      // 动画函数
      const animate = () => {
        requestAnimationFrame(animate);
        if (!isDragging) {
          wireframeMesh.rotation.y += rotationSpeed; // 自动缓慢旋转
          avatars.rotation.y += rotationSpeed; // 确保头像跟随球体旋转
        }
        renderer.render(scene, camera);
      };
      animate();

      // 添加交互事件监听器
      container.value.addEventListener('mousedown', (event) => {
        isDragging = true;
        previousMousePosition = { x: event.clientX, y: event.clientY };
      });

      container.value.addEventListener('mousemove', (event) => {
        if (isDragging) {
          const deltaMove = { x: event.clientX - previousMousePosition.x };
          wireframeMesh.rotation.y += deltaMove.x * 0.005; // 只在Yaw方向旋转
          avatars.rotation.y += deltaMove.x * 0.005; // 确保头像跟随球体旋转
          previousMousePosition = { x: event.clientX, y: event.clientY };
        }
      });

      container.value.addEventListener('mouseup', () => { isDragging = false; });
      container.value.addEventListener('mouseleave', () => { isDragging = false; });

      // 添加点击事件监听器
      container.value.addEventListener('click', (event) => {
        const mouse = new THREE.Vector2();
        mouse.x = (event.clientX / container.value.clientWidth) * 2 - 1;
        mouse.y = -(event.clientY / container.value.clientHeight) * 2 + 1;

        const raycaster = new THREE.Raycaster();
        raycaster.setFromCamera(mouse, camera);

        const intersects = raycaster.intersectObjects(scene.children, true);
        if (intersects.length > 0) {
          const intersectedObject = intersects[0].object;
          if (intersectedObject.userData.userId) {
            window.location.href = `/user/${intersectedObject.userData.userId}`;
          }
        }
      });
    });

    return { container };
  }
};
</script>

<style>
.sphere-container {
  width: 100%;
  height: 100vh;
  overflow: hidden;
  cursor: grab;
}

.sphere-container:active {
  cursor: grabbing;
}
</style> -->
