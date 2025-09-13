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