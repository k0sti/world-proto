import { SceneManager } from './renderer/scene';
import { GeometryControllerAsync } from './geometry/geometry-controller-async';
import { FPSCounter } from './utils/fps-counter';

class Application {
  private sceneManager: SceneManager;
  private geometryController: GeometryControllerAsync;
  private lastTime: number = 0;
  private isRunning: boolean = false;
  private renderFPSCounter: FPSCounter;
  private cameraUpdateInterval: number = 100; // Update camera position every 100ms
  private lastCameraUpdate: number = 0;

  constructor() {
    this.sceneManager = new SceneManager();
    this.geometryController = new GeometryControllerAsync(this.sceneManager);
    this.renderFPSCounter = new FPSCounter();
  }

  async initialize(): Promise<void> {
    const canvas = document.getElementById('render-canvas') as HTMLCanvasElement;
    if (!canvas) throw new Error('Canvas element not found');

    this.sceneManager.initialize(canvas);
    await this.geometryController.initialize();
    
    this.setupEventListeners();
    this.setupDebugPanel();
    console.log('Application initialized');
  }

  private setupEventListeners(): void {
    window.addEventListener('resize', () => this.handleResize());
  }
  
  private setupDebugPanel(): void {
    const gridSizeSlider = document.getElementById('grid-size') as HTMLInputElement;
    const gridSizeValue = document.getElementById('grid-size-value');
    
    if (gridSizeSlider && gridSizeValue) {
      gridSizeSlider.addEventListener('input', (e) => {
        const value = parseInt((e.target as HTMLInputElement).value);
        gridSizeValue.textContent = value.toString();
        this.geometryController.setGridSize(value);
      });
    }
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

    // Get camera position
    const cameraPos = this.sceneManager.getCameraPosition();
    
    // Update camera position display
    const cameraXElement = document.getElementById('camera-x');
    const cameraYElement = document.getElementById('camera-y');
    const cameraZElement = document.getElementById('camera-z');
    if (cameraXElement) cameraXElement.textContent = cameraPos.x.toFixed(2);
    if (cameraYElement) cameraYElement.textContent = cameraPos.y.toFixed(2);
    if (cameraZElement) cameraZElement.textContent = cameraPos.z.toFixed(2);
    
    // Update camera position for geometry generation (throttled)
    if (currentTime - this.lastCameraUpdate > this.cameraUpdateInterval) {
      this.geometryController.updateCamera(cameraPos.x, cameraPos.y, cameraPos.z, 30.0);
      this.lastCameraUpdate = currentTime;
    }

    // Apply latest geometry if available (decoupled from generation)
    this.geometryController.applyLatestGeometry();
    
    // Render the scene
    this.sceneManager.render();
    
    // Update render FPS counter
    const fps = this.renderFPSCounter.update();
    const fpsElement = document.getElementById('fps-counter');
    if (fpsElement) {
      fpsElement.textContent = fps.toString();
    }
    
    // Update stats display
    const statsElement = document.getElementById('stats');
    if (statsElement) {
      statsElement.textContent = 'Procedural Terrain Generator';
    }

    requestAnimationFrame(this.animate);
  };
}

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