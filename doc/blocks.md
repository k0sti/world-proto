# Voxel Block System Implementation Plan

## Overview
Add a voxel block system that generates 16x16x16 block chunks on top of the existing terrain. Blocks will have indexed colors and the geometry will be combined with terrain geometry.

## Data Structure

### Block Storage
```rust
// crates/geometry-engine/src/geometry/voxel.rs
pub struct VoxelChunk {
    blocks: [[[u32; 16]; 16]; 16],  // 16x16x16 grid
    position: (i32, i32, i32),       // Chunk world position
}

impl VoxelChunk {
    pub fn new(chunk_x: i32, chunk_y: i32, chunk_z: i32) -> Self {
        // Initialize with random values 0-7
        // 0 = empty (air)
        // 1-7 = different block types with colors
    }
}
```

### Block Types and Colors
```rust
pub enum BlockType {
    Air = 0,
    Stone = 1,      // Gray (#808080)
    Grass = 2,      // Green (#00FF00)
    Dirt = 3,       // Brown (#8B4513)
    Water = 4,      // Blue (#0080FF)
    Sand = 5,       // Yellow (#F4E4C1)
    Wood = 6,       // Dark Brown (#654321)
    Leaves = 7,     // Dark Green (#228B22)
}

impl BlockType {
    pub fn get_color(&self) -> [f32; 3] {
        match self {
            BlockType::Air => [0.0, 0.0, 0.0],
            BlockType::Stone => [0.5, 0.5, 0.5],
            BlockType::Grass => [0.0, 1.0, 0.0],
            BlockType::Dirt => [0.545, 0.271, 0.075],
            BlockType::Water => [0.0, 0.5, 1.0],
            BlockType::Sand => [0.957, 0.894, 0.757],
            BlockType::Wood => [0.396, 0.263, 0.129],
            BlockType::Leaves => [0.133, 0.545, 0.133],
        }
    }
}
```

## Geometry Generation

### Mesh Generation Algorithm
1. **Face Culling**: Only generate faces that are exposed (adjacent to air or chunk boundary)
2. **Greedy Meshing** (optional optimization): Combine adjacent faces of same type
3. **Vertex Generation**: Each visible face generates 4 vertices with position, normal, and color

```rust
pub fn generate_chunk_mesh(chunk: &VoxelChunk) -> (Vec<f32>, Vec<u32>, Vec<f32>, Vec<f32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut normals = Vec::new();
    let mut colors = Vec::new();
    
    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                let block_type = chunk.blocks[x][y][z];
                if block_type == 0 { continue; } // Skip air blocks
                
                // Check each face for visibility
                // Generate vertices only for exposed faces
                // Add vertex colors based on block type
            }
        }
    }
    
    (vertices, indices, normals, colors)
}
```

## Integration with Existing System

### 1. Update GeometryData Structure
```rust
// crates/geometry-engine/src/lib.rs
#[wasm_bindgen]
pub struct GeometryData {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    normals: Vec<f32>,
    colors: Vec<f32>,  // NEW: vertex colors (RGB)
}
```

### 2. Modify Terrain Generator
```rust
// crates/geometry-engine/src/geometry/terrain.rs
pub struct TerrainGenerator {
    // ... existing fields ...
    voxel_chunks: HashMap<(i32, i32, i32), VoxelChunk>,
}

impl TerrainGenerator {
    pub fn generate(&self) -> (Vec<f32>, Vec<u32>, Vec<f32>, Vec<f32>) {
        // 1. Generate terrain mesh as before
        let (mut vertices, mut indices, mut normals) = self.generate_terrain();
        let mut colors = vec![0.5, 0.5, 0.5; vertices.len() / 3]; // Default gray for terrain
        
        // 2. Generate voxel chunks based on camera position
        let chunks_to_generate = self.get_visible_chunks();
        
        // 3. For each chunk, generate mesh and append
        for chunk_pos in chunks_to_generate {
            let chunk = self.get_or_create_chunk(chunk_pos);
            let (v, i, n, c) = generate_chunk_mesh(&chunk);
            
            // Offset indices and append to main geometry
            let vertex_offset = vertices.len() / 3;
            vertices.extend(v);
            indices.extend(i.iter().map(|idx| idx + vertex_offset as u32));
            normals.extend(n);
            colors.extend(c);
        }
        
        (vertices, indices, normals, colors)
    }
}
```

### 3. Update TypeScript Interfaces
```typescript
// packages/geometry-lib/src/index.ts
export interface GeometryData {
    vertices: Float32Array;
    indices: Uint32Array;
    normals: Float32Array;
    colors: Float32Array;  // NEW
}
```

### 4. Update Three.js Rendering
```typescript
// packages/app/src/renderer/scene.ts
updateGeometry(vertices: Float32Array, indices: Uint32Array, normals: Float32Array, colors: Float32Array): void {
    // ... existing cleanup ...
    
    const geometry = new THREE.BufferGeometry();
    geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3));
    geometry.setAttribute('normal', new THREE.BufferAttribute(normals, 3));
    geometry.setAttribute('color', new THREE.BufferAttribute(colors, 3));  // NEW
    geometry.setIndex(new THREE.BufferAttribute(indices, 1));
    
    // Use vertex colors in material
    const material = new THREE.MeshPhongMaterial({
        vertexColors: true,  // Enable vertex colors
        side: THREE.DoubleSide
    });
    
    this.geometryMesh = new THREE.Mesh(geometry, material);
}
```

## Implementation Steps

### Phase 1: Core Voxel System
1. Create `voxel.rs` with `VoxelChunk` struct and `BlockType` enum
2. Implement random block initialization
3. Implement basic mesh generation (all faces, no culling)
4. Add color data to `GeometryData` struct

### Phase 2: Optimization
1. Implement face culling (only render exposed faces)
2. Add chunk position offset for world placement
3. Implement chunk caching to avoid regenerating unchanged chunks

### Phase 3: Integration
1. Update `terrain.rs` to include voxel chunk generation
2. Position chunks above terrain height
3. Combine terrain and voxel geometry
4. Update TypeScript interfaces and worker

### Phase 4: Rendering
1. Update Three.js material to use vertex colors
2. Test color rendering
3. Adjust lighting for better visibility

## Performance Considerations

1. **Chunk Size**: 16x16x16 = 4096 blocks per chunk
2. **Memory**: Each chunk uses 16KB (4096 * 4 bytes)
3. **Faces**: Worst case 6 faces * 4096 blocks = 24,576 faces per chunk
4. **Optimization**: Face culling reduces faces by ~70-80% typically

## Future Enhancements

1. **Greedy Meshing**: Combine adjacent same-color faces into larger quads
2. **LOD System**: Lower detail for distant chunks
3. **Chunk Streaming**: Load/unload chunks based on distance
4. **Block Editing**: Add/remove blocks interactively
5. **Texture Atlas**: Replace colors with textures
6. **Ambient Occlusion**: Add AO for better depth perception

## Testing Strategy

1. Start with single chunk at origin
2. Verify face generation and colors
3. Test face culling correctness
4. Add multiple chunks
5. Test performance with varying chunk counts
6. Verify terrain + voxel combination