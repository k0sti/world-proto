export class FPSCounter {
  private frameCount = 0;
  private lastTime = performance.now();
  private fps = 0;
  private updateInterval = 250; // Update 4 times per second
  
  update(): number {
    this.frameCount++;
    const currentTime = performance.now();
    const deltaTime_ms = currentTime - this.lastTime;
    
    if (deltaTime_ms >= this.updateInterval) {
      this.fps = Math.round((this.frameCount * 1000) / deltaTime_ms);
      this.frameCount = 0;
      this.lastTime = currentTime;
    }
    
    return this.fps;
  }
  
  getFPS(): number {
    return this.fps;
  }
}