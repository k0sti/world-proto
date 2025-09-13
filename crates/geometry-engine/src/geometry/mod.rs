pub mod primitives;
pub mod heightmap;
pub mod terrain;

use crate::GeometryData;
use terrain::TerrainGenerator;

pub struct AnimationState {
    terrain: TerrainGenerator,
}

impl AnimationState {
    pub fn new() -> Self {
        Self {
            terrain: TerrainGenerator::new(100, 0.5),
        }
    }

    pub fn update_camera(&mut self, camera_x: f32, camera_z: f32, radius: f32) {
        self.terrain.update_camera(camera_x, camera_z, radius);
    }
    
    pub fn set_grid_size(&mut self, grid_size: usize) {
        self.terrain.set_grid_size(grid_size);
    }

    pub fn generate_geometry(&self) -> GeometryData {
        let (vertices, indices, normals) = self.terrain.generate();
        GeometryData::new(vertices, indices, normals)
    }

    pub fn get_current_vertices(&self) -> Vec<f32> {
        let (vertices, _, _) = self.terrain.generate();
        vertices
    }

    pub fn get_current_indices(&self) -> Vec<u32> {
        let (_, indices, _) = self.terrain.generate();
        indices
    }

    pub fn get_current_normals(&self) -> Vec<f32> {
        let (_, _, normals) = self.terrain.generate();
        normals
    }
    
    pub fn get_animation_info(&self) -> String {
        format!("Procedural Terrain Generator")
    }
}