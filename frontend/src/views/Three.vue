<!-- <template>
  <div ref="container" class="three-container"></div>
</template>

<script lang="ts">
import { defineComponent, onMounted, ref } from 'vue';
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';

export default defineComponent({
  name: 'WatercolorEffect',
  setup() {
    const container = ref<HTMLDivElement | null>(null);

    const vertexShader = (): string => {
      return `
        varying vec2 vUv;
        void main() {
          vUv = uv;
          gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
        }
      `;
    };

    const fragmentShader = (): string => {
      return `
        uniform float time;
        uniform vec2 resolution;
        varying vec2 vUv;
        
        void main() {
          vec2 st = vUv;
          st.x *= resolution.x / resolution.y;
          
          float color = 0.0;
          color += sin(st.x * 80.0) * sin(st.y * 10.0);
          color += sin(st.x * 1.0 + time) * 0.5;
          color += cos(st.y * 60.0 + time) * 0.5;

          gl_FragColor = vec4(vec3(color), 1.0);
        }
      `;
    };

    const initThreeJS = () => {
      if (!container.value) return;

      const scene = new THREE.Scene();
      const camera = new THREE.PerspectiveCamera(75, container.value.clientWidth / container.value.clientHeight, 0.1, 1000);
      camera.position.z = 5;

      const renderer = new THREE.WebGLRenderer();
      renderer.setSize(container.value.clientWidth, container.value.clientHeight);
      container.value.appendChild(renderer.domElement);

      const controls = new OrbitControls(camera, renderer.domElement);

      const geometry = new THREE.BoxGeometry();
      const material = new THREE.ShaderMaterial({
        uniforms: {
          time: { value: 1.0 },
          resolution: { value: new THREE.Vector2(container.value.clientWidth, container.value.clientHeight) }
        },
        vertexShader: vertexShader(),
        fragmentShader: fragmentShader()
      });

      const cube = new THREE.Mesh(geometry, material);
      scene.add(cube);

      window.addEventListener('resize', () => onWindowResize(camera, renderer, container.value), false);
      onWindowResize(camera, renderer, container.value);

      const animate = () => {
        requestAnimationFrame(animate);
        material.uniforms.time.value += 0.05;
        controls.update();
        renderer.render(scene, camera);
      };
      animate();
    };

    const onWindowResize = (camera: THREE.PerspectiveCamera, renderer: THREE.WebGLRenderer, container: HTMLDivElement | null) => {
      if (!container) return;
      camera.aspect = container.clientWidth / container.clientHeight;
      camera.updateProjectionMatrix();
      renderer.setSize(container.clientWidth, container.clientHeight);
    };

    onMounted(() => {
      initThreeJS();
    });

    return {
      container
    };
  }
});
</script>

<style scoped>
.three-container {
  width: 100%;
  height: 100vh;
  overflow: hidden;
}
</style> -->
