import { SceneManager } from './renderer/scene';
import { GeometryControllerAsync } from './geometry/geometry-controller-async';
import { FPSCounter } from './utils/fps-counter';
import { TerrainPanelBuilder } from './ui/terrain-panel-builder';
import { getDefaultParams, getParamsFromUI, setParamsToUI } from './config/terrain-config';

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
    this.setupTerrainPanel();
    console.log('Application initialized');
  }

  private setupEventListeners(): void {
    window.addEventListener('resize', () => this.handleResize());
    
    // Panel toggle buttons
    const panelToggles = document.querySelectorAll('.panel-toggle');
    panelToggles.forEach(toggle => {
      toggle.addEventListener('click', (e) => {
        const button = e.currentTarget as HTMLElement;
        const panelName = button.dataset.panel;
        this.togglePanel(panelName);
      });
    });
    
    // Keyboard shortcuts for panels
    // window.addEventListener('keydown', (e) => {
    //   if (e.key.toLowerCase() === 'h') {
    //     this.togglePanel('help');
    //   } else if (e.key.toLowerCase() === 'd') {
    //     this.togglePanel('debug');
    //   } else if (e.key.toLowerCase() === 't') {
    //     this.togglePanel('terrain');
    //   } else if (e.key.toLowerCase() === 's') {
    //     this.togglePanel('settings');
    //   }
    // });
  }
  
  private togglePanel(panelName: string | undefined): void {
    if (!panelName) return;
    
    // Get all panels and toggle buttons
    const panels = document.querySelectorAll('.side-panel');
    const toggles = document.querySelectorAll('.panel-toggle');
    
    // Get the specific panel and button
    const panel = document.getElementById(`${panelName}-panel`);
    const button = document.querySelector(`.panel-toggle[data-panel="${panelName}"]`);
    
    if (panel && button) {
      const isActive = panel.classList.contains('active');
      
      // Close all panels and deactivate all buttons
      panels.forEach(p => p.classList.remove('active'));
      toggles.forEach(t => t.classList.remove('active'));
      
      // If the panel wasn't active, open it
      if (!isActive) {
        panel.classList.add('active');
        button.classList.add('active');
      }
    }
  }
  
  private setupDebugPanel(): void {
    const renderDistanceSlider = document.getElementById('render-distance') as HTMLInputElement;
    const renderDistanceValue = document.getElementById('render-distance-value');
    
    if (renderDistanceSlider && renderDistanceValue) {
      renderDistanceSlider.addEventListener('input', (e) => {
        const value = parseInt((e.target as HTMLInputElement).value);
        renderDistanceValue.textContent = value.toString();
        this.geometryController.setRenderDistance(value);
      });
    }
  }
  
  private setupTerrainPanel(): void {
    // Build controls dynamically from configuration
    const terrainContent = document.querySelector('#terrain-panel .panel-content');
    if (terrainContent) {
      TerrainPanelBuilder.buildControls(terrainContent as HTMLElement);
    }
    
    // Setup event handlers (after controls are built)
    const applyButton = document.getElementById('apply-terrain');
    if (applyButton) {
      applyButton.addEventListener('click', () => {
        this.applyTerrainSettings();
      });
    }
    
    const resetButton = document.getElementById('reset-terrain');
    if (resetButton) {
      resetButton.addEventListener('click', () => {
        this.resetTerrainSettings();
      });
    }
  }
  
  private applyTerrainSettings(): void {
    const params = getParamsFromUI();
    this.geometryController.setTerrainParams(params);
  }
  
  private resetTerrainSettings(): void {
    const defaultParams = getDefaultParams();
    setParamsToUI(defaultParams);
    this.geometryController.setTerrainParams(defaultParams);
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