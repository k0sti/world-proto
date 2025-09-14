use super::terrain::TerrainParams;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum BlockType {
    Air = 0,
    Stone = 1,
    Grass = 2,
    Dirt = 3,
    Water = 4,
    Sand = 5,
    Wood = 6,
    Leaves = 7,
}

impl BlockType {
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => BlockType::Air,
            1 => BlockType::Stone,
            2 => BlockType::Grass,
            3 => BlockType::Dirt,
            4 => BlockType::Water,
            5 => BlockType::Sand,
            6 => BlockType::Wood,
            7 => BlockType::Leaves,
            _ => BlockType::Air,
        }
    }

    pub fn get_color(&self) -> [f32; 3] {
        match self {
            BlockType::Air => [0.0, 0.0, 0.0],
            BlockType::Stone => [0.5, 0.5, 0.5],
            BlockType::Grass => [0.0, 1.0, 0.0],
            BlockType::Dirt => [0.545, 0.271, 0.075],
            BlockType::Water => [0.0, 0.5, 1.0],
            BlockType::Sand => [0.957, 0.894, 0.757],
            BlockType::Wood => [0.396, 0.263, 0.129],
            BlockType::Leaves => [0.133, 0.545, 0.133],
        }
    }
}

pub struct VoxelChunk {
    blocks: [[[u32; 16]; 16]; 16],
    position: (i32, i32, i32),
}

impl VoxelChunk {
    pub fn new_with_terrain_params<F, G, H>(
        chunk_x: i32, 
        chunk_y: i32, 
        chunk_z: i32,
        params: TerrainParams,
        terrain_height_fn: F,
        biome_fn: G,
        detail_fn: H,
    ) -> Self 
    where
        F: Fn(f32, f32, &TerrainParams) -> f32,
        G: Fn(f32, f32, &TerrainParams) -> f32,
        H: Fn(f32, f32, f32, &TerrainParams) -> f32,
    {
        let mut blocks = [[[0u32; 16]; 16]; 16];
        
        let chunk_world_x = chunk_x as f32 * 16.0;
        let chunk_world_y = chunk_y as f32 * 16.0;
        let chunk_world_z = chunk_z as f32 * 16.0;
        
        // Use sea level from params
        let sea_level = params.sea_level;
        
        for x in 0..16 {
            for z in 0..16 {
                let world_x = chunk_world_x + x as f32;
                let world_z = chunk_world_z + z as f32;
                
                // Get terrain height and biome value
                let terrain_height = terrain_height_fn(world_x, world_z, &params);
                let biome = biome_fn(world_x, world_z, &params);
                
                // Determine if this is a beach/desert area
                let is_beach = terrain_height > sea_level - 2.0 && terrain_height < sea_level + 3.0;
                let is_desert = biome > params.desert_threshold && terrain_height > sea_level;
                let is_mountain = terrain_height > 15.0;
                
                for y in 0..16 {
                    let world_y = chunk_world_y + y as f32;
                    
                    // Add some 3D noise for caves
                    let cave_noise = detail_fn(world_x, world_y, world_z, &params);
                    let is_cave = cave_noise > params.cave_threshold && world_y < terrain_height - 5.0;
                    
                    if is_cave {
                        blocks[x][y][z] = 0; // Air for caves
                    } else if world_y < terrain_height - 5.0 {
                        // Deep underground: always stone
                        blocks[x][y][z] = 1;
                    } else if world_y < terrain_height - 2.0 {
                        // Underground: stone
                        blocks[x][y][z] = 1;
                    } else if world_y < terrain_height {
                        // Near surface
                        if is_beach || (terrain_height < sea_level + 1.0) {
                            blocks[x][y][z] = 5; // Sand near water
                        } else if is_desert {
                            blocks[x][y][z] = 5; // Sand in desert
                        } else if is_mountain && world_y > 20.0 {
                            blocks[x][y][z] = 1; // Stone on mountains
                        } else {
                            blocks[x][y][z] = 3; // Dirt
                        }
                    } else if world_y < terrain_height + 1.0 {
                        // Surface layer
                        if world_y < sea_level && terrain_height < sea_level {
                            blocks[x][y][z] = 4; // Water
                        } else if is_beach || is_desert {
                            blocks[x][y][z] = 5; // Sand
                        } else if is_mountain && world_y > 25.0 {
                            blocks[x][y][z] = 1; // Stone on high mountains
                        } else {
                            blocks[x][y][z] = 2; // Grass
                        }
                    } else if world_y < sea_level {
                        // Fill with water up to sea level
                        blocks[x][y][z] = 4;
                    } else {
                        // Check for trees
                        if world_y == (terrain_height + 1.0).floor() && 
                           !is_beach && !is_desert && !is_mountain {
                            // Tree generation based on density parameter
                            let tree_hash = ((world_x as i32 * 73 + world_z as i32 * 179) % 100).abs();
                            if tree_hash < params.tree_density as i32 {
                                // Place a tree trunk
                                for tree_y in 0..5 {
                                    let block_y = y + tree_y;
                                    if block_y < 16 {
                                        if tree_y < 4 {
                                            blocks[x][block_y][z] = 6; // Wood
                                        } else {
                                            blocks[x][block_y][z] = 7; // Leaves
                                        }
                                    }
                                }
                                // Add leaves around the top
                                if y + 3 < 16 {
                                    for dx in -1i32..=1 {
                                        for dz in -1i32..=1 {
                                            if dx != 0 || dz != 0 {
                                                let leaf_x = (x as i32 + dx) as usize;
                                                let leaf_z = (z as i32 + dz) as usize;
                                                if leaf_x < 16 && leaf_z < 16 && y + 3 < 16 {
                                                    if blocks[leaf_x][y + 3][leaf_z] == 0 {
                                                        blocks[leaf_x][y + 3][leaf_z] = 7;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            blocks[x][y][z] = 0; // Air
                        }
                    }
                }
            }
        }
        
        Self {
            blocks,
            position: (chunk_x, chunk_y, chunk_z),
        }
    }
    
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> BlockType {
        BlockType::from_u32(self.blocks[x][y][z])
    }
    
    fn is_face_visible(&self, x: usize, y: usize, z: usize, face: Face) -> bool {
        let block = self.get_block(x, y, z);
        if block == BlockType::Air {
            return false;
        }
        
        match face {
            Face::Top => y == 15 || self.get_block(x, y + 1, z) == BlockType::Air,
            Face::Bottom => y == 0 || self.get_block(x, y - 1, z) == BlockType::Air,
            Face::Left => x == 0 || self.get_block(x - 1, y, z) == BlockType::Air,
            Face::Right => x == 15 || self.get_block(x + 1, y, z) == BlockType::Air,
            Face::Front => z == 15 || self.get_block(x, y, z + 1) == BlockType::Air,
            Face::Back => z == 0 || self.get_block(x, y, z - 1) == BlockType::Air,
        }
    }
    
    pub fn generate_mesh(&self) -> (Vec<f32>, Vec<u32>, Vec<f32>, Vec<f32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut normals = Vec::new();
        let mut colors = Vec::new();
        let mut vertex_count = 0u32;
        
        let block_size = 1.0f32;
        let chunk_offset_x = self.position.0 as f32 * 16.0;
        let chunk_offset_y = self.position.1 as f32 * 16.0;
        let chunk_offset_z = self.position.2 as f32 * 16.0;
        
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let block = self.get_block(x, y, z);
                    if block == BlockType::Air {
                        continue;
                    }
                    
                    let color = block.get_color();
                    let pos_x = chunk_offset_x + x as f32 * block_size;
                    let pos_y = chunk_offset_y + y as f32 * block_size;
                    let pos_z = chunk_offset_z + z as f32 * block_size;
                    
                    // Check each face for visibility and add geometry
                    for face in &[Face::Top, Face::Bottom, Face::Left, Face::Right, Face::Front, Face::Back] {
                        if self.is_face_visible(x, y, z, *face) {
                            let (face_vertices, face_normals) = get_face_geometry(*face, pos_x, pos_y, pos_z, block_size);
                            
                            // Add vertices
                            vertices.extend_from_slice(&face_vertices);
                            normals.extend_from_slice(&face_normals);
                            
                            // Add colors (4 vertices per face)
                            for _ in 0..4 {
                                colors.extend_from_slice(&color);
                            }
                            
                            // Add indices (2 triangles per face)
                            indices.push(vertex_count);
                            indices.push(vertex_count + 1);
                            indices.push(vertex_count + 2);
                            indices.push(vertex_count);
                            indices.push(vertex_count + 2);
                            indices.push(vertex_count + 3);
                            
                            vertex_count += 4;
                        }
                    }
                }
            }
        }
        
        (vertices, indices, normals, colors)
    }
}

#[derive(Clone, Copy, Debug)]
enum Face {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

fn get_face_geometry(face: Face, x: f32, y: f32, z: f32, size: f32) -> (Vec<f32>, Vec<f32>) {
    let vertices = match face {
        Face::Top => vec![
            x, y + size, z,
            x + size, y + size, z,
            x + size, y + size, z + size,
            x, y + size, z + size,
        ],
        Face::Bottom => vec![
            x, y, z + size,
            x + size, y, z + size,
            x + size, y, z,
            x, y, z,
        ],
        Face::Left => vec![
            x, y, z,
            x, y, z + size,
            x, y + size, z + size,
            x, y + size, z,
        ],
        Face::Right => vec![
            x + size, y, z + size,
            x + size, y, z,
            x + size, y + size, z,
            x + size, y + size, z + size,
        ],
        Face::Front => vec![
            x, y, z + size,
            x + size, y, z + size,
            x + size, y + size, z + size,
            x, y + size, z + size,
        ],
        Face::Back => vec![
            x + size, y, z,
            x, y, z,
            x, y + size, z,
            x + size, y + size, z,
        ],
    };
    
    let normal = match face {
        Face::Top => [0.0, 1.0, 0.0],
        Face::Bottom => [0.0, -1.0, 0.0],
        Face::Left => [-1.0, 0.0, 0.0],
        Face::Right => [1.0, 0.0, 0.0],
        Face::Front => [0.0, 0.0, 1.0],
        Face::Back => [0.0, 0.0, -1.0],
    };
    
    let mut normals = Vec::new();
    for _ in 0..4 {
        normals.extend_from_slice(&normal);
    }
    
    (vertices, normals)
}