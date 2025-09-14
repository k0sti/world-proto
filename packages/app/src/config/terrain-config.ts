export interface TerrainParam {
  id: string;
  label: string;
  min: number;
  max: number;
  default: number;
  step: number;
  unit?: string;
  rustField: string;
}

export const TERRAIN_PARAMS: TerrainParam[] = [
  {
    id: 'mountain-scale',
    label: 'Mountain Scale',
    min: 10,
    max: 50,
    default: 30,
    step: 5,
    rustField: 'mountainScale'
  },
  {
    id: 'hills-scale',
    label: 'Hills Scale',
    min: 5,
    max: 25,
    default: 15,
    step: 2,
    rustField: 'hillsScale'
  },
  {
    id: 'roughness',
    label: 'Terrain Roughness',
    min: 1,
    max: 10,
    default: 3,
    step: 1,
    rustField: 'roughness'
  },
  {
    id: 'sea-level',
    label: 'Sea Level',
    min: -10,
    max: 10,
    default: 0,
    step: 2,
    rustField: 'seaLevel'
  },
  {
    id: 'tree-density',
    label: 'Tree Density',
    min: 0,
    max: 10,
    default: 3,
    step: 1,
    unit: '%',
    rustField: 'treeDensity'
  },
  {
    id: 'cave-frequency',
    label: 'Cave Frequency',
    min: 0,
    max: 100,
    default: 70,
    step: 10,
    unit: '%',
    rustField: 'caveThreshold'
  },
  {
    id: 'biome-scale',
    label: 'Biome Scale',
    min: 50,
    max: 500,
    default: 200,
    step: 50,
    rustField: 'biomeScale'
  },
  {
    id: 'desert-threshold',
    label: 'Desert Amount',
    min: 0,
    max: 100,
    default: 30,
    step: 10,
    unit: '%',
    rustField: 'desertThreshold'
  }
];

export interface TerrainParams {
  mountainScale: number;
  hillsScale: number;
  roughness: number;
  seaLevel: number;
  treeDensity: number;
  caveThreshold: number;
  biomeScale: number;
  desertThreshold: number;
}

export function getDefaultParams(): TerrainParams {
  const params: any = {};
  TERRAIN_PARAMS.forEach(param => {
    params[param.rustField] = param.default;
  });
  return params as TerrainParams;
}

export function getParamsFromUI(): TerrainParams {
  const params: any = {};
  TERRAIN_PARAMS.forEach(param => {
    const input = document.getElementById(param.id) as HTMLInputElement;
    params[param.rustField] = parseFloat(input?.value || param.default.toString());
  });
  return params as TerrainParams;
}

export function setParamsToUI(params: TerrainParams): void {
  TERRAIN_PARAMS.forEach(param => {
    const input = document.getElementById(param.id) as HTMLInputElement;
    const valueDisplay = document.getElementById(`${param.id}-value`);
    const value = (params as any)[param.rustField] || param.default;
    
    if (input) {
      input.value = value.toString();
    }
    if (valueDisplay) {
      valueDisplay.textContent = value.toString();
    }
  });
}