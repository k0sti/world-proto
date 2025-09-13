use nalgebra::Vector3;

pub struct HeightmapGenerator {
    grid_size: usize,
    grid_resolution: f32,
    time: f32,
}

impl HeightmapGenerator {
    pub fn new(grid_size: usize, grid_resolution: f32) -> Self {
        Self {
            grid_size,
            grid_resolution,
            time: 0.0,
        }
    }

    pub fn update(&mut self, time: f32) {
        self.time = time;
    }

    pub fn generate(&self) -> (Vec<f32>, Vec<u32>, Vec<f32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut normals = Vec::new();

        let half_size = (self.grid_size as f32 * self.grid_resolution) / 2.0;

        // Generate vertices with sine wave height
        for z in 0..=self.grid_size {
            for x in 0..=self.grid_size {
                let x_pos = x as f32 * self.grid_resolution - half_size;
                let z_pos = z as f32 * self.grid_resolution - half_size;
                
                // Calculate height using multiple sine waves
                let height = self.calculate_height(x_pos, z_pos);
                
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

        (vertices, indices, normals)
    }

    fn calculate_height(&self, x: f32, z: f32) -> f32 {
        let mut height = 0.0;
        
        // Wave 1: Primary wave moving diagonally
        height += (x * 0.1 + z * 0.1 + self.time).sin() * 0.5;
        
        // Wave 2: Secondary wave with different frequency
        height += (x * 0.2 - z * 0.15 + self.time * 1.5).sin() * 0.3;
        
        // Wave 3: Smaller, faster wave for detail
        height += (x * 0.4 + z * 0.3 + self.time * 2.0).sin() * 0.2;
        
        // Wave 4: Large slow wave
        height += ((x * 0.05 + z * 0.05 + self.time * 0.5).sin() * 
                   (x * 0.05 - z * 0.05 + self.time * 0.3).cos()) * 0.4;
        
        // Wave 5: Radial wave from center
        let dist = (x * x + z * z).sqrt();
        height += (dist * 0.2 - self.time * 2.0).sin() * 0.3;
        
        height
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