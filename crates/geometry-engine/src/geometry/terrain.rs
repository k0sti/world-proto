use std::collections::HashMap;
use super::voxel::VoxelChunk;

pub struct TerrainGenerator {
    camera_x: f32,
    camera_y: f32,
    camera_z: f32,
    radius: f32,
    render_distance: i32,
    voxel_chunks: HashMap<(i32, i32, i32), VoxelChunk>,
}

impl TerrainGenerator {
    pub fn new() -> Self {
        Self {
            camera_x: 0.0,
            camera_y: 0.0,
            camera_z: 0.0,
            radius: 15.0,
            render_distance: 1,
            voxel_chunks: HashMap::new(),
        }
    }

    pub fn update_camera(&mut self, camera_x: f32, camera_y: f32, camera_z: f32, radius: f32) {
        self.camera_x = camera_x;
        self.camera_y = camera_y;
        self.camera_z = camera_z;
        self.radius = radius;
    }
    
    pub fn set_render_distance(&mut self, distance: i32) {
        self.render_distance = distance.max(1).min(5);
        // Clear chunks to force regeneration with new distance
        self.voxel_chunks.clear();
    }

    pub fn generate(&mut self) -> (Vec<f32>, Vec<u32>, Vec<f32>, Vec<f32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut normals = Vec::new();
        let mut colors = Vec::new();
        
        // Generate voxel chunks only
        let chunk_positions = self.get_visible_chunk_positions();
        for chunk_pos in chunk_positions {
            let chunk = self.get_or_create_chunk(chunk_pos);
            let (v, i, n, c) = chunk.generate_mesh();
            
            // Offset indices and append to main geometry
            let vertex_offset = (vertices.len() / 3) as u32;
            vertices.extend(v);
            for idx in i {
                indices.push(idx + vertex_offset);
            }
            normals.extend(n);
            colors.extend(c);
        }

        (vertices, indices, normals, colors)
    }
    
    fn get_visible_chunk_positions(&self) -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();
        
        // Generate chunks around camera position including vertical chunks
        let chunk_x = (self.camera_x / 16.0).floor() as i32;
        let chunk_y = (self.camera_y / 16.0).floor() as i32;
        let chunk_z = (self.camera_z / 16.0).floor() as i32;
        
        let dist = self.render_distance;
        
        // Generate grid of chunks around camera based on render distance
        for dx in -dist..=dist {
            for dy in -(dist + 1)..=(dist + 1) {  // More vertical range to see terrain variation
                for dz in -dist..=dist {
                    positions.push((chunk_x + dx, chunk_y + dy, chunk_z + dz));
                }
            }
        }
        
        positions
    }
    
    fn get_or_create_chunk(&mut self, pos: (i32, i32, i32)) -> &VoxelChunk {
        if !self.voxel_chunks.contains_key(&pos) {
            // Create a closure that captures the terrain calculation
            let chunk = VoxelChunk::new_with_terrain(pos.0, pos.1, pos.2, |x, z| {
                // Recreate the terrain height calculation inline
                let mut height = 0.0;
                let mut amplitude = 4.0;
                let mut frequency = 0.01;
                
                for _ in 0..5 {
                    height += Self::noise2d_static(x * frequency, z * frequency) * amplitude;
                    amplitude *= 0.5;
                    frequency *= 2.0;
                }
                
                height += Self::noise2d_static(x * 0.002, z * 0.002) * 10.0;
                height += Self::noise2d_static(x * 0.1, z * 0.1) * 0.5;
                
                height
            });
            self.voxel_chunks.insert(pos, chunk);
        }
        self.voxel_chunks.get(&pos).unwrap()
    }
    
    // Static version of noise functions for use in closures
    fn noise2d_static(x: f32, y: f32) -> f32 {
        let ix = x.floor() as i32;
        let iy = y.floor() as i32;
        let fx = x - x.floor();
        let fy = y - y.floor();
        
        let a = Self::hash2d_static(ix, iy);
        let b = Self::hash2d_static(ix + 1, iy);
        let c = Self::hash2d_static(ix, iy + 1);
        let d = Self::hash2d_static(ix + 1, iy + 1);
        
        let ux = fx * fx * (3.0 - 2.0 * fx);
        let uy = fy * fy * (3.0 - 2.0 * fy);
        
        let x1 = a * (1.0 - ux) + b * ux;
        let x2 = c * (1.0 - ux) + d * ux;
        
        x1 * (1.0 - uy) + x2 * uy
    }
    
    fn hash2d_static(x: i32, y: i32) -> f32 {
        let mut n = x + y * 57;
        n = (n << 13) ^ n;
        let m = (n * (n * n * 15731 + 789221) + 1376312589) & 0x7fffffff;
        1.0 - (m as f32) / 1073741824.0
    }
}