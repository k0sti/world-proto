import * as THREE from 'three';
import { PointerLockControls } from 'three/examples/jsm/controls/PointerLockControls.js';

export class SceneManager {
  private scene: THREE.Scene;
  private camera: THREE.PerspectiveCamera;
  private renderer: THREE.WebGLRenderer;
  private controls: PointerLockControls;
  private geometryMesh: THREE.Mesh | null = null;
  private keys: { [key: string]: boolean } = {};
  private moveSpeed: number = 0.1;
  private velocity: THREE.Vector3 = new THREE.Vector3();
  private direction: THREE.Vector3 = new THREE.Vector3();

  constructor() {
    this.scene = new THREE.Scene();
    this.camera = new THREE.PerspectiveCamera(
      75,
      window.innerWidth / window.innerHeight,
      0.1,
      1000
    );
    this.renderer = new THREE.WebGLRenderer();
    this.controls = new PointerLockControls(this.camera, document.body);
  }

  initialize(canvas: HTMLCanvasElement): void {
    this.renderer = new THREE.WebGLRenderer({ 
      canvas, 
      antialias: true,
      alpha: true 
    });
    this.renderer.setSize(window.innerWidth, window.innerHeight);
    this.renderer.setPixelRatio(window.devicePixelRatio);
    this.renderer.shadowMap.enabled = true;
    this.renderer.shadowMap.type = THREE.PCFSoftShadowMap;

    this.camera.position.set(5, 5, 5);
    this.camera.lookAt(0, 0, 0);

    this.controls = new PointerLockControls(this.camera, this.renderer.domElement);
    this.scene.add(this.controls.getObject());
    
    this.renderer.domElement.addEventListener('click', () => {
      this.controls.lock();
    });
    
    this.controls.addEventListener('lock', () => {
      const instructions = document.getElementById('instructions');
      if (instructions) instructions.style.display = 'none';
    });
    
    this.controls.addEventListener('unlock', () => {
      const instructions = document.getElementById('instructions');
      if (instructions) instructions.style.display = 'block';
    });

    this.scene.background = new THREE.Color(0x1a1a2e);
    this.scene.fog = new THREE.Fog(0x1a1a2e, 10, 50);

    this.setupLights();
    this.setupKeyboardControls();

    const gridHelper = new THREE.GridHelper(10, 10, 0x444444, 0x222222);
    this.scene.add(gridHelper);
  }

  private setupLights(): void {
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.4);
    this.scene.add(ambientLight);

    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
    directionalLight.position.set(5, 10, 5);
    directionalLight.castShadow = true;
    directionalLight.shadow.camera.near = 0.1;
    directionalLight.shadow.camera.far = 50;
    directionalLight.shadow.camera.left = -10;
    directionalLight.shadow.camera.right = 10;
    directionalLight.shadow.camera.top = 10;
    directionalLight.shadow.camera.bottom = -10;
    this.scene.add(directionalLight);

    const pointLight = new THREE.PointLight(0x00ff88, 0.5, 100);
    pointLight.position.set(-5, 5, -5);
    this.scene.add(pointLight);
  }

  private setupKeyboardControls(): void {
    window.addEventListener('keydown', (event) => {
      this.keys[event.key.toLowerCase()] = true;
    });

    window.addEventListener('keyup', (event) => {
      this.keys[event.key.toLowerCase()] = false;
    });
  }

  private updateMovement(): void {
    if (!this.controls.isLocked) return;
    
    const forward = new THREE.Vector3();
    const right = new THREE.Vector3();
    
    this.camera.getWorldDirection(forward);
    
    right.crossVectors(forward, new THREE.Vector3(0, 1, 0));
    right.normalize();
    
    if (this.keys['w']) {
      this.camera.position.addScaledVector(forward, this.moveSpeed);
    }
    if (this.keys['s']) {
      this.camera.position.addScaledVector(forward, -this.moveSpeed);
    }
    if (this.keys['a']) {
      this.camera.position.addScaledVector(right, -this.moveSpeed);
    }
    if (this.keys['d']) {
      this.camera.position.addScaledVector(right, this.moveSpeed);
    }
    if (this.keys[' ']) {
      this.camera.position.y += this.moveSpeed;
    }
    if (this.keys['shift']) {
      this.camera.position.y -= this.moveSpeed;
    }
  }

  updateGeometry(vertices: Float32Array, indices: Uint32Array, normals: Float32Array): void {
    if (this.geometryMesh) {
      this.scene.remove(this.geometryMesh);
      this.geometryMesh.geometry.dispose();
      if (this.geometryMesh.material instanceof THREE.Material) {
        this.geometryMesh.material.dispose();
      }
    }

    const geometry = new THREE.BufferGeometry();
    geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3));
    geometry.setAttribute('normal', new THREE.BufferAttribute(normals, 3));
    geometry.setIndex(new THREE.BufferAttribute(indices, 1));

    const material = new THREE.MeshPhongMaterial({
      color: 0x00ff88,
      specular: 0x111111,
      shininess: 100,
      wireframe: false,
      side: THREE.DoubleSide
    });

    this.geometryMesh = new THREE.Mesh(geometry, material);
    this.geometryMesh.castShadow = true;
    this.geometryMesh.receiveShadow = true;
    this.scene.add(this.geometryMesh);
  }

  render(): void {
    this.updateMovement();
    this.renderer.render(this.scene, this.camera);
  }

  handleResize(): void {
    const width = window.innerWidth;
    const height = window.innerHeight;
    
    this.camera.aspect = width / height;
    this.camera.updateProjectionMatrix();
    this.renderer.setSize(width, height);
  }
}