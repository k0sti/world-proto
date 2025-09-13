import { GeometryGenerator } from '@workspace/geometry-lib';
import { SceneManager } from '../renderer/scene';

export class GeometryController {
  private geometryGenerator: GeometryGenerator;
  private sceneManager: SceneManager;
  private isInitialized: boolean = false;

  constructor(geometryGenerator: GeometryGenerator, sceneManager: SceneManager) {
    this.geometryGenerator = geometryGenerator;
    this.sceneManager = sceneManager;
  }

  initialize(): void {
    if (this.isInitialized) return;
    this.isInitialized = true;
    console.log('GeometryController initialized');
  }

  update(time: number, deltaTime: number): void {
    if (!this.isInitialized) {
      console.warn('GeometryController not initialized');
      return;
    }

    const geometryData = this.geometryGenerator.generateFrame(time, deltaTime);
    
    if (geometryData) {
      const vertices = new Float32Array(geometryData.vertices);
      const indices = new Uint32Array(geometryData.indices);
      const normals = new Float32Array(geometryData.normals);
      
      this.sceneManager.updateGeometry(vertices, indices, normals);
    }
  }

  setAnimationType(type: string): void {
    console.log(`Animation type changed to: ${type}`);
  }

  reset(): void {
    console.log('Geometry controller reset');
  }
}