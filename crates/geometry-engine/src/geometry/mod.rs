pub mod primitives;
pub mod heightmap;
pub mod terrain;
pub mod voxel;

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

    pub fn generate_geometry(&mut self) -> GeometryData {
        let (vertices, indices, normals, colors) = self.terrain.generate();
        GeometryData::new(vertices, indices, normals, colors)
    }

    pub fn get_current_vertices(&mut self) -> Vec<f32> {
        let (vertices, _, _, _) = self.terrain.generate();
        vertices
    }

    pub fn get_current_indices(&mut self) -> Vec<u32> {
        let (_, indices, _, _) = self.terrain.generate();
        indices
    }

    pub fn get_current_normals(&mut self) -> Vec<f32> {
        let (_, _, normals, _) = self.terrain.generate();
        normals
    }
    
    pub fn get_animation_info(&self) -> String {
        format!("Procedural Terrain Generator")
    }
}