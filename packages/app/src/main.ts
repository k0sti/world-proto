import { GeometryGenerator } from '@workspace/geometry-lib';
import { SceneManager } from './renderer/scene';
import { GeometryController } from './geometry/geometry-controller';
import { FPSCounter } from './utils/fps-counter';

class Application {
  private geometryGenerator: GeometryGenerator;
  private sceneManager: SceneManager;
  private geometryController: GeometryController;
  private lastTime: number = 0;
  private isRunning: boolean = false;
  private fpsCounter: FPSCounter;

  constructor() {
    this.geometryGenerator = new GeometryGenerator();
    this.sceneManager = new SceneManager();
    this.geometryController = new GeometryController(
      this.geometryGenerator,
      this.sceneManager
    );
    this.fpsCounter = new FPSCounter();
  }

  async initialize(): Promise<void> {
    const canvas = document.getElementById('render-canvas') as HTMLCanvasElement;
    if (!canvas) throw new Error('Canvas element not found');

    await this.geometryGenerator.initialize();
    this.sceneManager.initialize(canvas);
    this.geometryController.initialize();
    
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

    this.geometryController.update(currentTime / 1000, deltaTime);
    this.sceneManager.render();
    
    // Update FPS counter
    const fps = this.fpsCounter.update();
    const fpsElement = document.getElementById('fps-counter');
    if (fpsElement) {
      fpsElement.textContent = fps.toString();
    }
    
    // Update stats display
    const animationInfo = this.geometryGenerator.getAnimationInfo();
    const statsElement = document.getElementById('stats');
    if (statsElement) {
      statsElement.textContent = animationInfo;
    }
    
    // Update mesh stats
    const stats = this.geometryController.getStats();
    const vertexElement = document.getElementById('vertex-count');
    const triangleElement = document.getElementById('triangle-count');
    if (vertexElement) vertexElement.textContent = stats.vertices.toString();
    if (triangleElement) triangleElement.textContent = stats.triangles.toString();

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