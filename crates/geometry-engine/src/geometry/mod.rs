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
            terrain: TerrainGenerator::new(),
        }
    }

    pub fn update_camera(&mut self, camera_x: f32, camera_y: f32, camera_z: f32, radius: f32) {
        self.terrain.update_camera(camera_x, camera_y, camera_z, radius);
    }

    pub fn generate_geometry(&mut self) -> GeometryData {
        let (vertices, indices, normals, colors) = self.terrain.generate();
        GeometryData::new(vertices, indices, normals, colors)
    }
}