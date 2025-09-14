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

  generateFrame(cameraX: number, cameraY: number, cameraZ: number, radius: number): GeometryData | null {
    if (!this.engine) {
      console.error('GeometryEngine not initialized');
      return null;
    }
    return this.engine.generate_frame(cameraX, cameraY, cameraZ, radius);
  }
  
  setRenderDistance(distance: number): void {
    if (!this.engine) {
      console.error('GeometryEngine not initialized');
      return;
    }
    this.engine.set_render_distance(distance);
  }
  
  setTerrainParams(params: any): void {
    if (!this.engine) {
      console.error('GeometryEngine not initialized');
      return;
    }
    this.engine.set_terrain_params(params);
  }
}

export { GeometryEngine, GeometryData };