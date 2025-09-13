use nalgebra::Vector3;

pub fn lerp_vector3(a: &Vector3<f32>, b: &Vector3<f32>, t: f32) -> Vector3<f32> {
    a + (b - a) * t
}

pub fn normalize_vector3(v: &Vector3<f32>) -> Vector3<f32> {
    let len = v.magnitude();
    if len > 0.0 {
        v / len
    } else {
        Vector3::new(0.0, 0.0, 0.0)
    }
}

pub fn cross_product(a: &Vector3<f32>, b: &Vector3<f32>) -> Vector3<f32> {
    a.cross(b)
}

pub fn dot_product(a: &Vector3<f32>, b: &Vector3<f32>) -> f32 {
    a.dot(b)
}