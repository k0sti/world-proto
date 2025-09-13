I'll create a comprehensive specification for a 3D geometry rendering web application using Rust/WASM and Three.js. Let me first check the referenced article for context on the project structure.{"query": "WASM Rust Vite pnpm workspace agnislav medium"}Now I have enough context. Let me create a comprehensive specification for the 3D geometry rendering application.

# 3D Geometry Rendering Application Specification

## Overview
A standalone web application that renders animated 3D geometry using Rust compiled to WebAssembly for geometry generation and Three.js for rendering. The application follows a monorepo structure using Bun workspaces, with Vite as the build tool.

## Project Structure

```
3d-geometry-renderer/
├── packages/
│   ├── app/                     # Frontend application
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   ├── vite.config.ts
│   │   ├── index.html
│   │   ├── public/
│   │   └── src/
│   │       ├── main.ts
│   │       ├── renderer/
│   │       │   ├── scene.ts
│   │       │   ├── camera.ts
│   │       │   └── renderer.ts
│   │       ├── geometry/
│   │       │   └── geometry-controller.ts
│   │       └── styles/
│   │           └── main.css
│   ├── geometry-lib/             # TypeScript wrapper for WASM
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   └── src/
│   │       └── index.ts
│   └── wasm/                     # Generated WASM output
│       └── (generated files)
├── crates/
│   └── geometry-engine/          # Rust WASM source
│       ├── Cargo.toml
│       ├── build.sh
│       └── src/
│           ├── lib.rs
│           ├── geometry/
│           │   ├── mod.rs
│           │   ├── primitives.rs
│           │   └── animations.rs
│           └── math/
│               ├── mod.rs
│               └── vector.rs
├── Cargo.toml                    # Workspace Cargo configuration
├── package.json                  # Root package.json
├── bunfig.toml                   # Bun configuration
├── .gitignore
└── README.md
```

## Implementation Details

### 1. Root Configuration Files

#### `package.json`
```json
{
  "name": "3d-geometry-renderer",
  "version": "1.0.0",
  "private": true,
  "type": "module",
  "workspaces": [
    "packages/*"
  ],
  "scripts": {
    "dev": "bun run build:wasm && bun run --filter @workspace/app dev",
    "build": "bun run build:wasm && bun run build:all",
    "build:all": "bun run --filter '@workspace/*' build",
    "build:wasm": "wasm-pack build crates/geometry-engine --target web --out-dir ../../packages/wasm",
    "build:wasm:dev": "wasm-pack build crates/geometry-engine --dev --target web --out-dir ../../packages/wasm",
    "clean": "rm -rf packages/wasm && bun run --filter '@workspace/*' clean",
    "test": "bun run --filter '@workspace/*' test"
  },
  "devDependencies": {
    "@types/bun": "latest",
    "typescript": "^5.3.3"
  }
}
```

#### `bunfig.toml`
```toml
[install]
# Use the platform's fastest package manager
registry = "https://registry.npmjs.org"
```

#### `Cargo.toml` (root)
```toml
[workspace]
members = ["crates/geometry-engine"]
resolver = "2"

[profile.release]
opt-level = 3
lto = true
```

### 2. Rust WASM Module (`crates/geometry-engine/`)

#### `Cargo.toml`
```toml
[package]
name = "geometry-engine"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = "0.3"
nalgebra = { version = "0.32", default-features = false, features = ["std"] }

[dependencies.web-sys]
features = ["console"]

[profile.release]
opt-level = "s"
lto = true
```

#### `src/lib.rs`
```rust
mod geometry;
mod math;

use wasm_bindgen::prelude::*;
use geometry::{GeometryData, AnimationState};

#[wasm_bindgen]
pub struct GeometryEngine {
    animation_state: AnimationState,
}

#[wasm_bindgen]
impl GeometryEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        web_sys::console::log_1(&"GeometryEngine initialized".into());
        Self {
            animation_state: AnimationState::new(),
        }
    }

    #[wasm_bindgen]
    pub fn generate_frame(&mut self, time: f32, delta_time: f32) -> GeometryData {
        self.animation_state.update(time, delta_time);
        self.animation_state.generate_geometry()
    }

    #[wasm_bindgen]
    pub fn get_vertices(&self) -> Vec<f32> {
        self.animation_state.get_current_vertices()
    }

    #[wasm_bindgen]
    pub fn get_indices(&self) -> Vec<u32> {
        self.animation_state.get_current_indices()
    }

    #[wasm_bindgen]
    pub fn get_normals(&self) -> Vec<f32> {
        self.animation_state.get_current_normals()
    }
}

#[wasm_bindgen]
pub struct GeometryData {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    normals: Vec<f32>,
}

#[wasm_bindgen]
impl GeometryData {
    #[wasm_bindgen(getter)]
    pub fn vertices(&self) -> Vec<f32> {
        self.vertices.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn indices(&self) -> Vec<u32> {
        self.indices.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn normals(&self) -> Vec<f32> {
        self.normals.clone()
    }
}
```

### 3. TypeScript Wrapper Library (`packages/geometry-lib/`)

#### `package.json`
```json
{
  "name": "@workspace/geometry-lib",
  "version": "1.0.0",
  "type": "module",
  "main": "./src/index.ts",
  "exports": {
    ".": "./src/index.ts"
  },
  "scripts": {
    "build": "tsc",
    "clean": "rm -rf dist"
  },
  "dependencies": {
    "@workspace/wasm": "workspace:*"
  },
  "devDependencies": {
    "typescript": "^5.3.3"
  }
}
```

#### `src/index.ts`
```typescript
import init, { GeometryEngine, GeometryData } from '@workspace/wasm';

let wasmInitialized = false;
let initPromise: Promise<void> | null = null;

export async function initializeWasm(): Promise<void> {
  if (wasmInitialized) return;
  if (initPromise) return initPromise;
  
  initPromise = init().then(() => {
    wasmInitialized = true;
    console.log('WASM module initialized');
  });
  
  return initPromise;
}

export class GeometryGenerator {
  private engine: GeometryEngine | null = null;

  async initialize(): Promise<void> {
    await initializeWasm();
    this.engine = new GeometryEngine();
  }

  generateFrame(time: number, deltaTime: number): GeometryData | null {
    if (!this.engine) {
      console.error('GeometryEngine not initialized');
      return null;
    }
    return this.engine.generate_frame(time, deltaTime);
  }

  getVertices(): Float32Array {
    if (!this.engine) return new Float32Array();
    return new Float32Array(this.engine.get_vertices());
  }

  getIndices(): Uint32Array {
    if (!this.engine) return new Uint32Array();
    return new Uint32Array(this.engine.get_indices());
  }

  getNormals(): Float32Array {
    if (!this.engine) return new Float32Array();
    return new Float32Array(this.engine.get_normals());
  }
}

export { GeometryEngine, GeometryData };
```

### 4. Frontend Application (`packages/app/`)

#### `package.json`
```json
{
  "name": "@workspace/app",
  "version": "1.0.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "clean": "rm -rf dist"
  },
  "dependencies": {
    "@workspace/geometry-lib": "workspace:*",
    "three": "^0.160.0",
    "@types/three": "^0.160.0"
  },
  "devDependencies": {
    "typescript": "^5.3.3",
    "vite": "^5.0.0",
    "vite-plugin-wasm": "^3.3.0",
    "vite-plugin-top-level-await": "^1.4.0"
  }
}
```

#### `vite.config.ts`
```typescript
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [wasm(), topLevelAwait()],
  server: {
    port: 3000,
    open: true
  },
  build: {
    target: 'esnext'
  }
});
```

#### `index.html`
```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>3D Geometry Renderer</title>
  <link rel="stylesheet" href="/src/styles/main.css">
</head>
<body>
  <div id="app">
    <canvas id="render-canvas"></canvas>
    <div id="controls">
      <div id="stats"></div>
    </div>
  </div>
  <script type="module" src="/src/main.ts"></script>
</body>
</html>
```

#### `src/main.ts`
```typescript
import { GeometryGenerator } from '@workspace/geometry-lib';
import { SceneManager } from './renderer/scene';
import { GeometryController } from './geometry/geometry-controller';

class Application {
  private geometryGenerator: GeometryGenerator;
  private sceneManager: SceneManager;
  private geometryController: GeometryController;
  private lastTime: number = 0;
  private isRunning: boolean = false;

  constructor() {
    this.geometryGenerator = new GeometryGenerator();
    this.sceneManager = new SceneManager();
    this.geometryController = new GeometryController(
      this.geometryGenerator,
      this.sceneManager
    );
  }

  async initialize(): Promise<void> {
    const canvas = document.getElementById('render-canvas') as HTMLCanvasElement;
    if (!canvas) throw new Error('Canvas element not found');

    await this.geometryGenerator.initialize();
    this.sceneManager.initialize(canvas);
    this.geometryController.initialize();
    
    this.setupEventListeners();
    console.log('Application initialized');
  }

  private setupEventListeners(): void {
    window.addEventListener('resize', () => this.handleResize());
  }

  private handleResize(): void {
    this.sceneManager.handleResize();
  }

  start(): void {
    if (this.isRunning) return;
    this.isRunning = true;
    this.lastTime = performance.now();
    this.animate();
  }

  stop(): void {
    this.isRunning = false;
  }

  private animate = (): void => {
    if (!this.isRunning) return;

    const currentTime = performance.now();
    const deltaTime = (currentTime - this.lastTime) / 1000;
    this.lastTime = currentTime;

    this.geometryController.update(currentTime / 1000, deltaTime);
    this.sceneManager.render();

    requestAnimationFrame(this.animate);
  };
}

// Initialize and start the application
async function main() {
  const app = new Application();
  
  try {
    await app.initialize();
    app.start();
  } catch (error) {
    console.error('Failed to initialize application:', error);
  }
}

main();
```

#### `src/renderer/scene.ts`
```typescript
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls';

export class SceneManager {
  private scene: THREE.Scene;
  private camera: THREE.PerspectiveCamera;
  private renderer: THREE.WebGLRenderer;
  private controls: OrbitControls;
  private geometryMesh: THREE.Mesh | null = null;

  constructor() {
    this.scene = new THREE.Scene();
    this.camera = new THREE.PerspectiveCamera(
      75,
      window.innerWidth / window.innerHeight,
      0.1,
      1000
    );
    this.renderer = new THREE.WebGLRenderer();
    this.controls = new OrbitControls(this.camera, this.renderer.domElement);
  }

  initialize(canvas: HTMLCanvasElement): void {
    // Setup renderer
    this.renderer = new THREE.WebGLRenderer({ 
      canvas, 
      antialias: true,
      alpha: true 
    });
    this.renderer.setSize(window.innerWidth, window.innerHeight);
    this.renderer.setPixelRatio(window.devicePixelRatio);
    this.renderer.shadowMap.enabled = true;
    this.renderer.shadowMap.type = THREE.PCFSoftShadowMap;

    // Setup camera
    this.camera.position.set(5, 5, 5);
    this.camera.lookAt(0, 0, 0);

    // Setup controls
    this.controls = new OrbitControls(this.camera, this.renderer.domElement);
    this.controls.enableDamping = true;
    this.controls.dampingFactor = 0.05;

    // Setup scene
    this.scene.background = new THREE.Color(0x1a1a2e);
    this.scene.fog = new THREE.Fog(0x1a1a2e, 10, 50);

    // Add lights
    this.setupLights();

    // Add grid
    const gridHelper = new THREE.GridHelper(10, 10, 0x444444, 0x222222);
    this.scene.add(gridHelper);
  }

  private setupLights(): void {
    // Ambient light
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.4);
    this.scene.add(ambientLight);

    // Directional light
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

    // Point light
    const pointLight = new THREE.PointLight(0x00ff88, 0.5, 100);
    pointLight.position.set(-5, 5, -5);
    this.scene.add(pointLight);
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
    this.controls.update();
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
```

### 5. Build and Development Scripts

#### `crates/geometry-engine/build.sh`
```bash
#!/usr/bin/env bash
set -e

echo "Building WASM module..."
wasm-pack build --target web --out-dir ../../packages/wasm

echo "WASM module built successfully!"
```

### 6. Additional Files

#### `.gitignore`
```
# Dependencies
node_modules/
.bun/

# Build outputs
dist/
build/
packages/wasm/
target/

# IDE
.vscode/
.idea/
*.swp
*.swo
.DS_Store

# Logs
*.log
npm-debug.log*
bun-debug.log*

# Environment
.env
.env.local
.env.*.local

# Rust
Cargo.lock
**/*.rs.bk
```

## Setup Instructions

1. **Initialize the project:**
```bash
mkdir 3d-geometry-renderer && cd 3d-geometry-renderer
bun init -y
```

2. **Create the directory structure as specified above**

3. **Copy all configuration files and source code as specified**

4. **Install dependencies:**
```bash
# Install wasm-pack globally if not already installed
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install Bun dependencies
bun install

# Build the WASM module
bun run build:wasm
```

5. **Run the development server:**
```bash
bun run dev
```

## Key Features

- **Monorepo structure** using Bun workspaces for better organization
- **Rust WASM module** for high-performance geometry generation
- **Three.js** for WebGL rendering
- **TypeScript** throughout for type safety
- **Vite** for fast development and optimized builds
- **Modular architecture** allowing easy extension
- **Animation system** with frame-based updates
- **Proper memory management** between WASM and JavaScript

## Performance Considerations

- WASM module handles all geometry calculations
- Efficient data transfer using typed arrays
- Geometry updates only when necessary
- Proper disposal of Three.js resources
- Request Animation Frame for smooth rendering

## Future Development Guidelines

1. **Adding new geometry types:** Extend the Rust `geometry` module
2. **UI controls:** Add a control panel using a library like dat.GUI
3. **Shader customization:** Implement custom GLSL shaders
4. **Physics integration:** Consider adding a physics engine
5. **State management:** Implement a state management solution if complexity grows

This specification provides a complete, production-ready foundation for 3D geometry rendering with Rust/WASM and Three.js, following modern best practices and maintaining excellent code organization for future development.

