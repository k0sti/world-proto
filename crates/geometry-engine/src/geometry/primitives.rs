use std::f32::consts::PI;
use nalgebra::Vector3;

#[derive(Clone)]
pub enum Primitive {
    Sphere { radius: f32, subdivisions: u32 },
    Cube { size: f32 },
    Torus { major_radius: f32, minor_radius: f32, subdivisions: u32 },
    Icosahedron { radius: f32 },
}

impl Primitive {
    pub fn generate(&self) -> (Vec<f32>, Vec<u32>, Vec<f32>) {
        match self {
            Primitive::Sphere { radius, subdivisions } => {
                generate_sphere(*radius, *subdivisions)
            }
            Primitive::Cube { size } => {
                generate_cube(*size)
            }
            Primitive::Torus { major_radius, minor_radius, subdivisions } => {
                generate_torus(*major_radius, *minor_radius, *subdivisions)
            }
            Primitive::Icosahedron { radius } => {
                generate_icosahedron(*radius)
            }
        }
    }
}

fn generate_sphere(radius: f32, subdivisions: u32) -> (Vec<f32>, Vec<u32>, Vec<f32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut normals = Vec::new();

    for lat in 0..=subdivisions {
        let theta = lat as f32 * PI / subdivisions as f32;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        for lon in 0..=subdivisions {
            let phi = lon as f32 * 2.0 * PI / subdivisions as f32;
            let sin_phi = phi.sin();
            let cos_phi = phi.cos();

            let x = cos_phi * sin_theta;
            let y = cos_theta;
            let z = sin_phi * sin_theta;

            vertices.push(x * radius);
            vertices.push(y * radius);
            vertices.push(z * radius);

            normals.push(x);
            normals.push(y);
            normals.push(z);
        }
    }

    for lat in 0..subdivisions {
        for lon in 0..subdivisions {
            let first = lat * (subdivisions + 1) + lon;
            let second = first + subdivisions + 1;

            indices.push(first);
            indices.push(second);
            indices.push(first + 1);

            indices.push(second);
            indices.push(second + 1);
            indices.push(first + 1);
        }
    }

    (vertices, indices, normals)
}

fn generate_cube(size: f32) -> (Vec<f32>, Vec<u32>, Vec<f32>) {
    let half = size / 2.0;
    
    let vertices = vec![
        -half, -half, -half,  half, -half, -half,  half,  half, -half, -half,  half, -half,
        -half, -half,  half,  half, -half,  half,  half,  half,  half, -half,  half,  half,
        -half,  half,  half, -half,  half, -half, -half, -half, -half, -half, -half,  half,
         half,  half,  half,  half,  half, -half,  half, -half, -half,  half, -half,  half,
        -half, -half, -half,  half, -half, -half,  half, -half,  half, -half, -half,  half,
        -half,  half, -half,  half,  half, -half,  half,  half,  half, -half,  half,  half,
    ];

    let indices = vec![
        0,  1,  2,  0,  2,  3,
        4,  5,  6,  4,  6,  7,
        8,  9, 10,  8, 10, 11,
        12, 13, 14, 12, 14, 15,
        16, 17, 18, 16, 18, 19,
        20, 21, 22, 20, 22, 23,
    ];

    let normals = vec![
        0.0, 0.0, -1.0,  0.0, 0.0, -1.0,  0.0, 0.0, -1.0,  0.0, 0.0, -1.0,
        0.0, 0.0,  1.0,  0.0, 0.0,  1.0,  0.0, 0.0,  1.0,  0.0, 0.0,  1.0,
        0.0, 1.0,  0.0,  0.0, 1.0,  0.0,  0.0, 1.0,  0.0,  0.0, 1.0,  0.0,
        1.0, 0.0,  0.0,  1.0, 0.0,  0.0,  1.0, 0.0,  0.0,  1.0, 0.0,  0.0,
        0.0, -1.0, 0.0,  0.0, -1.0, 0.0,  0.0, -1.0, 0.0,  0.0, -1.0, 0.0,
        -1.0, 0.0, 0.0, -1.0, 0.0,  0.0, -1.0, 0.0,  0.0, -1.0, 0.0,  0.0,
    ];

    (vertices, indices, normals)
}

fn generate_torus(major_radius: f32, minor_radius: f32, subdivisions: u32) -> (Vec<f32>, Vec<u32>, Vec<f32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut normals = Vec::new();

    for i in 0..=subdivisions {
        let u = i as f32 * 2.0 * PI / subdivisions as f32;
        let cos_u = u.cos();
        let sin_u = u.sin();

        for j in 0..=subdivisions {
            let v = j as f32 * 2.0 * PI / subdivisions as f32;
            let cos_v = v.cos();
            let sin_v = v.sin();

            let x = (major_radius + minor_radius * cos_v) * cos_u;
            let y = minor_radius * sin_v;
            let z = (major_radius + minor_radius * cos_v) * sin_u;

            vertices.push(x);
            vertices.push(y);
            vertices.push(z);

            let center_x = major_radius * cos_u;
            let center_z = major_radius * sin_u;
            let nx = x - center_x;
            let ny = y;
            let nz = z - center_z;
            let len = (nx * nx + ny * ny + nz * nz).sqrt();

            normals.push(nx / len);
            normals.push(ny / len);
            normals.push(nz / len);
        }
    }

    for i in 0..subdivisions {
        for j in 0..subdivisions {
            let first = i * (subdivisions + 1) + j;
            let second = first + subdivisions + 1;

            indices.push(first);
            indices.push(second);
            indices.push(first + 1);

            indices.push(second);
            indices.push(second + 1);
            indices.push(first + 1);
        }
    }

    (vertices, indices, normals)
}

fn generate_icosahedron(radius: f32) -> (Vec<f32>, Vec<u32>, Vec<f32>) {
    let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
    let a = 1.0;
    let b = 1.0 / phi;

    let mut vertices = vec![
        0.0,  b, -a,   b,  a, 0.0,  -b,  a, 0.0,
        0.0,  b,  a,   0.0, -b,  a,   -a, 0.0,  b,
        0.0, -b, -a,   a,  0.0, -b,    a, 0.0,  b,
        -a,  0.0, -b,   b, -a, 0.0,   -b, -a, 0.0,
    ];

    let length = (a * a + b * b).sqrt();
    for i in 0..vertices.len() {
        vertices[i] = vertices[i] / length * radius;
    }

    let indices = vec![
        2, 1, 0,   1, 2, 3,   5, 4, 3,   4, 8, 3,
        7, 6, 0,   6, 9, 0,   11, 10, 4,  10, 11, 6,
        9, 5, 2,   5, 9, 11,  8, 7, 1,    7, 8, 10,
        2, 5, 3,   8, 1, 3,   9, 2, 0,    1, 7, 0,
        11, 9, 6,  7, 10, 6,  5, 11, 4,   10, 8, 4,
    ];

    let mut normals = Vec::new();
    for i in (0..vertices.len()).step_by(3) {
        let x = vertices[i];
        let y = vertices[i + 1];
        let z = vertices[i + 2];
        let len = (x * x + y * y + z * z).sqrt();
        normals.push(x / len);
        normals.push(y / len);
        normals.push(z / len);
    }

    (vertices, indices, normals)
}