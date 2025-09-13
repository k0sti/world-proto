pub mod primitives;
pub mod heightmap;

use crate::GeometryData;
use heightmap::HeightmapGenerator;

pub struct AnimationState {
    heightmap: HeightmapGenerator,
    time: f32,
}

impl AnimationState {
    pub fn new() -> Self {
        Self {
            heightmap: HeightmapGenerator::new(50, 0.3),
            time: 0.0,
        }
    }

    pub fn update(&mut self, time: f32, _delta_time: f32) {
        self.time = time;
        self.heightmap.update(time);
    }

    pub fn generate_geometry(&self) -> GeometryData {
        let (vertices, indices, normals) = self.heightmap.generate();
        GeometryData::new(vertices, indices, normals)
    }

    pub fn get_current_vertices(&self) -> Vec<f32> {
        let (vertices, _, _) = self.heightmap.generate();
        vertices
    }

    pub fn get_current_indices(&self) -> Vec<u32> {
        let (_, indices, _) = self.heightmap.generate();
        indices
    }

    pub fn get_current_normals(&self) -> Vec<f32> {
        let (_, _, normals) = self.heightmap.generate();
        normals
    }
    
    pub fn get_animation_info(&self) -> String {
        format!("Animated Heightmap | Time: {:.2}", self.time)
    }
}