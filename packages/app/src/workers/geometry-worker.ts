import { GeometryGenerator } from '@workspace/geometry-lib';

export interface GeometryMessage {
  type: 'init' | 'update' | 'setRenderDistance';
  cameraX?: number;
  cameraY?: number;
  cameraZ?: number;
  radius?: number;
  renderDistance?: number;
}

export interface GeometryResult {
  vertices: Float32Array;
  indices: Uint32Array;
  normals: Float32Array;
  colors: Float32Array;
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
  private currentCameraY = 0;
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
      this.currentCameraY,
      this.currentCameraZ,
      this.currentRadius
    );
    
    if (geometryData) {
      const vertices = new Float32Array(geometryData.vertices);
      const indices = new Uint32Array(geometryData.indices);
      const normals = new Float32Array(geometryData.normals);
      const colors = new Float32Array(geometryData.colors);
      
      const result: GeometryResult = {
        vertices,
        indices,
        normals,
        colors,
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
        normals.buffer,
        colors.buffer
      ]);
    }
  }
  
  updateCamera(x: number, y: number, z: number, radius: number) {
    this.currentCameraX = x;
    this.currentCameraY = y;
    this.currentCameraZ = z;
    this.currentRadius = radius;
  }
  
  setRenderDistance(distance: number) {
    if (this.generator) {
      this.generator.setRenderDistance(distance);
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
      if (message.cameraX !== undefined && message.cameraY !== undefined && message.cameraZ !== undefined && message.radius !== undefined) {
        manager.updateCamera(message.cameraX, message.cameraY, message.cameraZ, message.radius);
      }
      break;
      
    case 'setRenderDistance':
      if (message.renderDistance !== undefined) {
        manager.setRenderDistance(message.renderDistance);
      }
      break;
  }
});