mod geometry;
mod math;

use wasm_bindgen::prelude::*;
use geometry::{AnimationState};

#[wasm_bindgen]
pub struct GeometryEngine {
    animation_state: AnimationState,
}

#[wasm_bindgen]
impl GeometryEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        web_sys::console::log_1(&"GeometryEngine initialized".into());
        Self {
            animation_state: AnimationState::new(),
        }
    }

    #[wasm_bindgen]
    pub fn update_camera(&mut self, camera_x: f32, camera_z: f32, radius: f32) {
        self.animation_state.update_camera(camera_x, camera_z, radius);
    }
    
    #[wasm_bindgen]
    pub fn set_grid_size(&mut self, grid_size: u32) {
        self.animation_state.set_grid_size(grid_size as usize);
    }

    #[wasm_bindgen]
    pub fn generate_frame(&mut self, camera_x: f32, camera_z: f32, radius: f32) -> GeometryData {
        self.animation_state.update_camera(camera_x, camera_z, radius);
        self.animation_state.generate_geometry()
    }

    #[wasm_bindgen]
    pub fn get_vertices(&self) -> Vec<f32> {
        self.animation_state.get_current_vertices()
    }

    #[wasm_bindgen]
    pub fn get_indices(&self) -> Vec<u32> {
        self.animation_state.get_current_indices()
    }

    #[wasm_bindgen]
    pub fn get_normals(&self) -> Vec<f32> {
        self.animation_state.get_current_normals()
    }
    
    #[wasm_bindgen]
    pub fn get_animation_info(&self) -> String {
        self.animation_state.get_animation_info()
    }
}

#[wasm_bindgen]
pub struct GeometryData {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    normals: Vec<f32>,
}

#[wasm_bindgen]
impl GeometryData {
    pub fn new(vertices: Vec<f32>, indices: Vec<u32>, normals: Vec<f32>) -> Self {
        Self {
            vertices,
            indices,
            normals,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn vertices(&self) -> Vec<f32> {
        self.vertices.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn indices(&self) -> Vec<u32> {
        self.indices.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn normals(&self) -> Vec<f32> {
        self.normals.clone()
    }
}