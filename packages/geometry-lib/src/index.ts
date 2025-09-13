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