mod geometry;

use wasm_bindgen::prelude::*;
use geometry::{AnimationState};
use crate::geometry::terrain::TerrainParams;
use serde_wasm_bindgen::from_value;

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
    pub fn generate_frame(&mut self, camera_x: f32, camera_y: f32, camera_z: f32, radius: f32) -> GeometryData {
        self.animation_state.update_camera(camera_x, camera_y, camera_z, radius);
        self.animation_state.generate_geometry()
    }
    
    #[wasm_bindgen]
    pub fn set_render_distance(&mut self, distance: i32) {
        self.animation_state.set_render_distance(distance);
    }
    
    #[wasm_bindgen]
    pub fn set_terrain_params(&mut self, params_js: JsValue) -> Result<(), JsValue> {
        let mut params: TerrainParams = from_value(params_js)?;
        
        // Convert percentages to fractions
        params.cave_threshold /= 100.0;
        params.desert_threshold /= 100.0;
        
        self.animation_state.set_terrain_params(params);
        Ok(())
    }
}

#[wasm_bindgen]
pub struct GeometryData {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    normals: Vec<f32>,
    colors: Vec<f32>,
}

#[wasm_bindgen]
impl GeometryData {
    pub fn new(vertices: Vec<f32>, indices: Vec<u32>, normals: Vec<f32>, colors: Vec<f32>) -> Self {
        Self {
            vertices,
            indices,
            normals,
            colors,
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
    
    #[wasm_bindgen(getter)]
    pub fn colors(&self) -> Vec<f32> {
        self.colors.clone()
    }
}