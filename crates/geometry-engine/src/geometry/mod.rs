pub mod primitives;
pub mod animations;

use crate::GeometryData;
use primitives::Primitive;
use animations::AnimationType;
use nalgebra::{Vector3, Matrix4};

pub struct AnimationState {
    current_primitive: Primitive,
    animation_type: AnimationType,
    time: f32,
    rotation: Vector3<f32>,
    scale: f32,
    morph_factor: f32,
}

impl AnimationState {
    pub fn new() -> Self {
        Self {
            current_primitive: Primitive::Sphere { radius: 1.0, subdivisions: 32 },
            animation_type: AnimationType::Morph,
            time: 0.0,
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: 1.0,
            morph_factor: 0.0,
        }
    }

    pub fn update(&mut self, time: f32, _delta_time: f32) {
        self.time = time;
        
        // Cycle through animation types every 10 seconds
        let animation_cycle = (time / 10.0) as usize % 4;
        self.animation_type = match animation_cycle {
            0 => AnimationType::Rotate,
            1 => AnimationType::Scale,
            2 => AnimationType::Morph,
            _ => AnimationType::Complex,
        };
        
        // Always update rotation for visual feedback
        self.rotation.x = time * 0.5;
        self.rotation.y = time * 0.7;
        self.rotation.z = time * 0.3;
        
        match self.animation_type {
            AnimationType::Rotate => {
                // Rotation is already set above
                self.scale = 1.0;
            }
            AnimationType::Scale => {
                self.scale = 1.0 + (time * 2.0).sin() * 0.5;
            }
            AnimationType::Morph => {
                self.morph_factor = (time * 0.5).sin() * 0.5 + 0.5;
                // Change primitive every 2.5 seconds
                let cycle = (time / 2.5) as usize % 4;
                self.current_primitive = match cycle {
                    0 => Primitive::Sphere { radius: 1.0, subdivisions: 32 },
                    1 => Primitive::Cube { size: 1.5 },
                    2 => Primitive::Torus { major_radius: 1.0, minor_radius: 0.3, subdivisions: 32 },
                    _ => Primitive::Icosahedron { radius: 1.2 },
                };
            }
            AnimationType::Complex => {
                self.scale = 1.0 + (time * 2.0).sin() * 0.3;
                self.morph_factor = (time * 0.5).cos() * 0.5 + 0.5;
                // Also cycle primitives in complex mode
                let cycle = (time / 3.0) as usize % 4;
                self.current_primitive = match cycle {
                    0 => Primitive::Sphere { radius: 1.2, subdivisions: 32 },
                    1 => Primitive::Cube { size: 1.0 },
                    2 => Primitive::Torus { major_radius: 0.8, minor_radius: 0.25, subdivisions: 32 },
                    _ => Primitive::Icosahedron { radius: 1.0 },
                };
            }
        }
    }

    pub fn generate_geometry(&self) -> GeometryData {
        let (mut vertices, indices, mut normals) = self.current_primitive.generate();
        
        let transform = self.create_transform_matrix();
        for i in (0..vertices.len()).step_by(3) {
            let v = Vector3::new(vertices[i], vertices[i + 1], vertices[i + 2]);
            let transformed = transform.transform_point(&nalgebra::Point3::from(v));
            vertices[i] = transformed.x;
            vertices[i + 1] = transformed.y;
            vertices[i + 2] = transformed.z;
        }
        
        for i in (0..normals.len()).step_by(3) {
            let n = Vector3::new(normals[i], normals[i + 1], normals[i + 2]);
            let transformed = transform.transform_vector(&n).normalize();
            normals[i] = transformed.x;
            normals[i + 1] = transformed.y;
            normals[i + 2] = transformed.z;
        }

        GeometryData::new(vertices, indices, normals)
    }

    fn create_transform_matrix(&self) -> Matrix4<f32> {
        let rotation_x = Matrix4::from_euler_angles(self.rotation.x, 0.0, 0.0);
        let rotation_y = Matrix4::from_euler_angles(0.0, self.rotation.y, 0.0);
        let rotation_z = Matrix4::from_euler_angles(0.0, 0.0, self.rotation.z);
        let scale = Matrix4::new_scaling(self.scale);
        
        scale * rotation_z * rotation_y * rotation_x
    }

    pub fn get_current_vertices(&self) -> Vec<f32> {
        let (vertices, _, _) = self.current_primitive.generate();
        vertices
    }

    pub fn get_current_indices(&self) -> Vec<u32> {
        let (_, indices, _) = self.current_primitive.generate();
        indices
    }

    pub fn get_current_normals(&self) -> Vec<f32> {
        let (_, _, normals) = self.current_primitive.generate();
        normals
    }
    
    pub fn get_animation_info(&self) -> String {
        let animation_type = match self.animation_type {
            AnimationType::Rotate => "Rotate",
            AnimationType::Scale => "Scale",
            AnimationType::Morph => "Morph",
            AnimationType::Complex => "Complex",
        };
        
        let primitive_type = match &self.current_primitive {
            Primitive::Sphere { .. } => "Sphere",
            Primitive::Cube { .. } => "Cube",
            Primitive::Torus { .. } => "Torus",
            Primitive::Icosahedron { .. } => "Icosahedron",
        };
        
        format!("Animation: {} | Shape: {}", animation_type, primitive_type)
    }
}