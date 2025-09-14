use std::collections::HashMap;
use super::voxel::VoxelChunk;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerrainParams {
    pub mountain_scale: f32,
    pub hills_scale: f32,
    pub roughness: f32,
    pub sea_level: f32,
    pub tree_density: f32,
    pub cave_threshold: f32,
    pub biome_scale: f32,
    pub desert_threshold: f32,
}

impl Default for TerrainParams {
    fn default() -> Self {
        Self {
            mountain_scale: 30.0,
            hills_scale: 15.0,
            roughness: 3.0,
            sea_level: 0.0,
            tree_density: 3.0,
            cave_threshold: 0.7,
            biome_scale: 200.0,
            desert_threshold: 0.3,
        }
    }
}

pub struct TerrainGenerator {
    camera_x: f32,
    camera_y: f32,
    camera_z: f32,
    radius: f32,
    render_distance: i32,
    params: TerrainParams,
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
            params: TerrainParams::default(),
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
    
    pub fn set_terrain_params(&mut self, params: TerrainParams) {
        self.params = params;
        // Clear chunks to force regeneration with new parameters
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
            let params = self.params.clone();
            
            // Create a closure that captures the terrain calculation
            let chunk = VoxelChunk::new_with_terrain_params(
                pos.0, pos.1, pos.2,
                params,
                |x, z, params| {
                    let mut height = 0.0;
                    
                    // Large scale terrain features (mountains and valleys)
                    height += Self::noise2d_static(x * 0.003, z * 0.003) * params.mountain_scale;
                    
                    // Medium scale hills
                    height += Self::noise2d_static(x * 0.01, z * 0.01) * params.hills_scale;
                    
                    // Small scale bumps (roughness)
                    height += Self::noise2d_static(x * 0.05, z * 0.05) * params.roughness;
                    
                    // Tiny details
                    height += Self::noise2d_static(x * 0.1, z * 0.1) * 1.0;
                    
                    // Create some plateaus and cliffs
                    let plateau = Self::noise2d_static(x * 0.002, z * 0.002);
                    if plateau > 0.3 {
                        height += 20.0;
                    }
                    
                    height
                },
                |x, z, params| {
                    // Biome function with scale parameter
                    let scale = 1.0 / params.biome_scale;
                    Self::noise2d_static(x * scale, z * scale)
                },
                |x, y, z, _params| {
                    // 3D noise for cave generation
                    let noise1 = Self::noise3d_static(x * 0.05, y * 0.05, z * 0.05);
                    let noise2 = Self::noise3d_static(x * 0.1, y * 0.1, z * 0.1) * 0.5;
                    noise1 + noise2
                }
            );
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
    
    fn noise3d_static(x: f32, y: f32, z: f32) -> f32 {
        let ix = x.floor() as i32;
        let iy = y.floor() as i32;
        let iz = z.floor() as i32;
        let fx = x - x.floor();
        let fy = y - y.floor();
        let fz = z - z.floor();
        
        // 8 corner points of the cube
        let a = Self::hash3d_static(ix, iy, iz);
        let b = Self::hash3d_static(ix + 1, iy, iz);
        let c = Self::hash3d_static(ix, iy + 1, iz);
        let d = Self::hash3d_static(ix + 1, iy + 1, iz);
        let e = Self::hash3d_static(ix, iy, iz + 1);
        let f = Self::hash3d_static(ix + 1, iy, iz + 1);
        let g = Self::hash3d_static(ix, iy + 1, iz + 1);
        let h = Self::hash3d_static(ix + 1, iy + 1, iz + 1);
        
        // Smooth interpolation factors
        let ux = fx * fx * (3.0 - 2.0 * fx);
        let uy = fy * fy * (3.0 - 2.0 * fy);
        let uz = fz * fz * (3.0 - 2.0 * fz);
        
        // Interpolate along x axis
        let x1 = a * (1.0 - ux) + b * ux;
        let x2 = c * (1.0 - ux) + d * ux;
        let x3 = e * (1.0 - ux) + f * ux;
        let x4 = g * (1.0 - ux) + h * ux;
        
        // Interpolate along y axis
        let y1 = x1 * (1.0 - uy) + x2 * uy;
        let y2 = x3 * (1.0 - uy) + x4 * uy;
        
        // Interpolate along z axis
        y1 * (1.0 - uz) + y2 * uz
    }
    
    fn hash3d_static(x: i32, y: i32, z: i32) -> f32 {
        let mut n = x + y * 57 + z * 131;
        n = (n << 13) ^ n;
        let m = (n * (n * n * 15731 + 789221) + 1376312589) & 0x7fffffff;
        1.0 - (m as f32) / 1073741824.0
    }
}