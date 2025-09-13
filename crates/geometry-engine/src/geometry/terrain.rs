use nalgebra::Vector3;
use std::collections::HashMap;
use super::voxel::VoxelChunk;

pub struct TerrainGenerator {
    grid_size: usize,
    grid_resolution: f32,
    camera_x: f32,
    camera_y: f32,
    camera_z: f32,
    radius: f32,
    voxel_chunks: HashMap<(i32, i32, i32), VoxelChunk>,
}

impl TerrainGenerator {
    pub fn new(grid_size: usize, grid_resolution: f32) -> Self {
        Self {
            grid_size,
            grid_resolution,
            camera_x: 0.0,
            camera_y: 0.0,
            camera_z: 0.0,
            radius: 15.0,
            voxel_chunks: HashMap::new(),
        }
    }

    pub fn update_camera(&mut self, camera_x: f32, camera_y: f32, camera_z: f32, radius: f32) {
        self.camera_x = camera_x;
        self.camera_y = camera_y;
        self.camera_z = camera_z;
        self.radius = radius;
    }
    
    pub fn set_grid_size(&mut self, grid_size: usize) {
        self.grid_size = grid_size;
    }

    pub fn generate(&mut self) -> (Vec<f32>, Vec<u32>, Vec<f32>, Vec<f32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut normals = Vec::new();
        let mut colors = Vec::new();

        // Calculate grid bounds based on camera position and radius
        let half_grid = (self.grid_size as f32 * self.grid_resolution) / 2.0;
        let start_x = self.camera_x - half_grid;
        let start_z = self.camera_z - half_grid;

        // Generate vertices with terrain height
        for z in 0..=self.grid_size {
            for x in 0..=self.grid_size {
                let x_pos = start_x + (x as f32 * self.grid_resolution);
                let z_pos = start_z + (z as f32 * self.grid_resolution);
                
                // Calculate height using noise function
                let height = self.calculate_terrain_height(x_pos, z_pos);
                
                vertices.push(x_pos);
                vertices.push(height);
                vertices.push(z_pos);
            }
        }

        // Generate indices for triangles
        for z in 0..self.grid_size {
            for x in 0..self.grid_size {
                let top_left = z * (self.grid_size + 1) + x;
                let top_right = top_left + 1;
                let bottom_left = (z + 1) * (self.grid_size + 1) + x;
                let bottom_right = bottom_left + 1;

                // First triangle
                indices.push(top_left as u32);
                indices.push(bottom_left as u32);
                indices.push(top_right as u32);

                // Second triangle
                indices.push(top_right as u32);
                indices.push(bottom_left as u32);
                indices.push(bottom_right as u32);
            }
        }

        // Calculate normals
        normals = self.calculate_normals(&vertices, &indices);
        
        // Add default terrain color (gray-brown)
        for _ in 0..(vertices.len() / 3) {
            colors.push(0.4);
            colors.push(0.35);
            colors.push(0.3);
        }
        
        // Generate voxel chunks
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
        
        // Generate 3x3x3 grid of chunks around camera
        for dx in -1..=1 {
            for dy in -2..=2 {  // More vertical range to see terrain variation
                for dz in -1..=1 {
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

    pub fn calculate_terrain_height(&self, world_x: f32, world_z: f32) -> f32 {
        // Multi-octave noise for realistic terrain
        let mut height = 0.0;
        let mut amplitude = 4.0;
        let mut frequency = 0.01;
        
        // Add multiple octaves of noise
        for octave in 0..5 {
            height += self.noise2d(world_x * frequency, world_z * frequency) * amplitude;
            amplitude *= 0.5;
            frequency *= 2.0;
        }
        
        // Add some larger features
        height += self.noise2d(world_x * 0.002, world_z * 0.002) * 10.0;
        
        // Add small detail
        height += self.noise2d(world_x * 0.1, world_z * 0.1) * 0.5;
        
        height
    }

    // Simple pseudorandom noise function based on coordinates
    fn noise2d(&self, x: f32, y: f32) -> f32 {
        let ix = x.floor() as i32;
        let iy = y.floor() as i32;
        let fx = x - x.floor();
        let fy = y - y.floor();
        
        // Get noise values at corners
        let a = self.hash2d(ix, iy);
        let b = self.hash2d(ix + 1, iy);
        let c = self.hash2d(ix, iy + 1);
        let d = self.hash2d(ix + 1, iy + 1);
        
        // Smooth interpolation
        let ux = fx * fx * (3.0 - 2.0 * fx);
        let uy = fy * fy * (3.0 - 2.0 * fy);
        
        // Bilinear interpolation
        let x1 = a * (1.0 - ux) + b * ux;
        let x2 = c * (1.0 - ux) + d * ux;
        
        x1 * (1.0 - uy) + x2 * uy
    }
    
    // Hash function for pseudorandom values
    fn hash2d(&self, x: i32, y: i32) -> f32 {
        let mut n = x + y * 57;
        n = (n << 13) ^ n;
        let m = (n * (n * n * 15731 + 789221) + 1376312589) & 0x7fffffff;
        1.0 - (m as f32) / 1073741824.0
    }

    fn calculate_normals(&self, vertices: &[f32], indices: &[u32]) -> Vec<f32> {
        let mut normals = vec![0.0; vertices.len()];
        
        // Calculate face normals and accumulate to vertex normals
        for i in (0..indices.len()).step_by(3) {
            let i0 = indices[i] as usize * 3;
            let i1 = indices[i + 1] as usize * 3;
            let i2 = indices[i + 2] as usize * 3;
            
            let v0 = Vector3::new(vertices[i0], vertices[i0 + 1], vertices[i0 + 2]);
            let v1 = Vector3::new(vertices[i1], vertices[i1 + 1], vertices[i1 + 2]);
            let v2 = Vector3::new(vertices[i2], vertices[i2 + 1], vertices[i2 + 2]);
            
            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let face_normal = edge1.cross(&edge2).normalize();
            
            // Add face normal to each vertex of the triangle
            for idx in &[i0, i1, i2] {
                normals[*idx] += face_normal.x;
                normals[*idx + 1] += face_normal.y;
                normals[*idx + 2] += face_normal.z;
            }
        }
        
        // Normalize all vertex normals
        for i in (0..normals.len()).step_by(3) {
            let n = Vector3::new(normals[i], normals[i + 1], normals[i + 2]);
            let normalized = n.normalize();
            normals[i] = normalized.x;
            normals[i + 1] = normalized.y;
            normals[i + 2] = normalized.z;
        }
        
        normals
    }
}