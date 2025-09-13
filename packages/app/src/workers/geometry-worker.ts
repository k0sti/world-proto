import { GeometryGenerator } from '@workspace/geometry-lib';

export interface GeometryMessage {
  type: 'init' | 'update' | 'setGridSize';
  cameraX?: number;
  cameraZ?: number;
  radius?: number;
  gridSize?: number;
}

export interface GeometryResult {
  vertices: Float32Array;
  indices: Uint32Array;
  normals: Float32Array;
  stats: {
    vertices: number;
    triangles: number;
  };
  timestamp: number;
}

class GeometryWorkerManager {
  private generator: GeometryGenerator | null = null;
  private isRunning = false;
  private updateInterval = 33; // ~30 FPS for geometry updates
  private lastUpdate = 0;
  private currentCameraX = 0;
  private currentCameraZ = 0;
  private currentRadius = 30;
  
  async initialize() {
    this.generator = new GeometryGenerator();
    await this.generator.initialize();
    self.postMessage({ type: 'ready' });
    this.startUpdateLoop();
  }
  
  private startUpdateLoop() {
    this.isRunning = true;
    this.update();
  }
  
  private update = () => {
    if (!this.isRunning || !this.generator) return;
    
    const now = performance.now();
    if (now - this.lastUpdate >= this.updateInterval) {
      this.generateGeometry();
      this.lastUpdate = now;
    }
    
    setTimeout(this.update, 16); // Check roughly 60 times per second
  }
  
  private generateGeometry() {
    if (!this.generator) return;
    
    const geometryData = this.generator.generateFrame(
      this.currentCameraX,
      this.currentCameraZ,
      this.currentRadius
    );
    
    if (geometryData) {
      const vertices = new Float32Array(geometryData.vertices);
      const indices = new Uint32Array(geometryData.indices);
      const normals = new Float32Array(geometryData.normals);
      
      const result: GeometryResult = {
        vertices,
        indices,
        normals,
        stats: {
          vertices: vertices.length / 3,
          triangles: indices.length / 3
        },
        timestamp: performance.now()
      };
      
      // Transfer ownership of the buffers to main thread
      self.postMessage({ type: 'geometry', data: result }, [
        vertices.buffer,
        indices.buffer,
        normals.buffer
      ]);
    }
  }
  
  updateCamera(x: number, z: number, radius: number) {
    this.currentCameraX = x;
    this.currentCameraZ = z;
    this.currentRadius = radius;
  }
  
  setGridSize(gridSize: number) {
    if (this.generator) {
      this.generator.setGridSize(gridSize);
    }
  }
  
  stop() {
    this.isRunning = false;
  }
}

// Worker message handler
const manager = new GeometryWorkerManager();

self.addEventListener('message', async (event) => {
  const message = event.data as GeometryMessage;
  
  switch (message.type) {
    case 'init':
      await manager.initialize();
      break;
      
    case 'update':
      if (message.cameraX !== undefined && message.cameraZ !== undefined && message.radius !== undefined) {
        manager.updateCamera(message.cameraX, message.cameraZ, message.radius);
      }
      break;
      
    case 'setGridSize':
      if (message.gridSize !== undefined) {
        manager.setGridSize(message.gridSize);
      }
      break;
  }
});