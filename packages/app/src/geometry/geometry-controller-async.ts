import { SceneManager } from '../renderer/scene';
import { FPSCounter } from '../utils/fps-counter';
import { GeometryResult, TerrainParams } from '../workers/geometry-worker';

export class GeometryControllerAsync {
  private sceneManager: SceneManager;
  private worker: Worker | null = null;
  private geometryFPSCounter: FPSCounter;
  private latestGeometry: GeometryResult | null = null;
  private stats = { vertices: 0, triangles: 0 };
  
  constructor(sceneManager: SceneManager) {
    this.sceneManager = sceneManager;
    this.geometryFPSCounter = new FPSCounter();
  }
  
  async initialize(): Promise<void> {
    return new Promise((resolve) => {
      this.worker = new Worker(
        new URL('../workers/geometry-worker.ts', import.meta.url),
        { type: 'module' }
      );
      
      this.worker.addEventListener('message', (event) => {
        if (event.data.type === 'ready') {
          console.log('Geometry worker initialized');
          resolve();
        } else if (event.data.type === 'geometry') {
          this.handleGeometryUpdate(event.data.data);
        }
      });
      
      this.worker.postMessage({ type: 'init' });
    });
  }
  
  private handleGeometryUpdate(geometry: GeometryResult) {
    this.latestGeometry = geometry;
    this.stats = geometry.stats;
    
    // Update geometry FPS counter
    const fps = this.geometryFPSCounter.update();
    const fpsElement = document.getElementById('geometry-fps');
    if (fpsElement) {
      fpsElement.textContent = fps.toString();
    }
    
    // Update mesh stats
    const vertexElement = document.getElementById('vertex-count');
    const triangleElement = document.getElementById('triangle-count');
    if (vertexElement) vertexElement.textContent = this.stats.vertices.toString();
    if (triangleElement) triangleElement.textContent = this.stats.triangles.toString();
  }
  
  updateCamera(x: number, y: number, z: number, radius: number) {
    if (this.worker) {
      this.worker.postMessage({
        type: 'update',
        cameraX: x,
        cameraY: y,
        cameraZ: z,
        radius
      });
    }
  }
  
  setRenderDistance(distance: number) {
    if (this.worker) {
      this.worker.postMessage({
        type: 'setRenderDistance',
        renderDistance: distance
      });
    }
  }
  
  setTerrainParams(params: TerrainParams) {
    if (this.worker) {
      this.worker.postMessage({
        type: 'setTerrainParams',
        terrainParams: params
      });
    }
  }
  
  // Called from render loop to apply latest geometry if available
  applyLatestGeometry() {
    if (this.latestGeometry) {
      this.sceneManager.updateGeometry(
        this.latestGeometry.vertices,
        this.latestGeometry.indices,
        this.latestGeometry.normals,
        this.latestGeometry.colors
      );
      this.latestGeometry = null; // Clear after applying
    }
  }
  
  getStats() {
    return this.stats;
  }
  
  destroy() {
    if (this.worker) {
      this.worker.terminate();
      this.worker = null;
    }
  }
}