import { TERRAIN_PARAMS, TerrainParam } from '../config/terrain-config';

export class TerrainPanelBuilder {
  static buildControls(container: HTMLElement): void {
    // Clear existing controls
    container.innerHTML = '';
    
    // Build controls from configuration
    TERRAIN_PARAMS.forEach(param => {
      const controlRow = this.createControlRow(param);
      container.appendChild(controlRow);
    });
    
    // Add buttons
    const buttonRow = this.createButtonRow();
    container.appendChild(buttonRow);
  }
  
  private static createControlRow(param: TerrainParam): HTMLElement {
    const row = document.createElement('div');
    row.className = 'control-row';
    
    const label = document.createElement('label');
    label.setAttribute('for', param.id);
    label.textContent = `${param.label}:`;
    row.appendChild(label);
    
    const input = document.createElement('input');
    input.type = 'range';
    input.id = param.id;
    input.min = param.min.toString();
    input.max = param.max.toString();
    input.value = param.default.toString();
    input.step = param.step.toString();
    row.appendChild(input);
    
    const valueSpan = document.createElement('span');
    valueSpan.id = `${param.id}-value`;
    valueSpan.textContent = param.default.toString();
    row.appendChild(valueSpan);
    
    if (param.unit) {
      const unitSpan = document.createElement('span');
      unitSpan.textContent = param.unit;
      row.appendChild(unitSpan);
    }
    
    // Add event listener for live updates
    input.addEventListener('input', (e) => {
      const value = (e.target as HTMLInputElement).value;
      valueSpan.textContent = value;
    });
    
    return row;
  }
  
  private static createButtonRow(): HTMLElement {
    const row = document.createElement('div');
    row.className = 'control-row';
    
    const applyButton = document.createElement('button');
    applyButton.id = 'apply-terrain';
    applyButton.className = 'apply-button';
    applyButton.textContent = 'Apply Changes';
    row.appendChild(applyButton);
    
    const resetButton = document.createElement('button');
    resetButton.id = 'reset-terrain';
    resetButton.className = 'reset-button';
    resetButton.textContent = 'Reset Defaults';
    row.appendChild(resetButton);
    
    return row;
  }
}