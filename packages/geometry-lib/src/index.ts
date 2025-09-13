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

  generateFrame(cameraX: number, cameraZ: number, radius: number): GeometryData | null {
    if (!this.engine) {
      console.error('GeometryEngine not initialized');
      return null;
    }
    return this.engine.generate_frame(cameraX, cameraZ, radius);
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

  getAnimationInfo(): string {
    if (!this.engine) return 'Not initialized';
    return this.engine.get_animation_info();
  }
  
  setGridSize(gridSize: number): void {
    if (!this.engine) return;
    this.engine.set_grid_size(gridSize);
  }
}

export { GeometryEngine, GeometryData };