#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub color: [f32; 3],
}

#[derive(PartialEq, Eq, Hash)]
pub struct VertexKey {
    pub v: usize,          // Index position
    pub vt: Option<usize>, // Index uv
    pub vn: Option<usize>, // Index normal
}

#[derive(Debug, Clone, Copy)]
pub struct ObjIndex {
    pub v: i32,          // Index position OBJ
    pub vt: Option<i32>, // Index uv OBJ
    pub vn: Option<i32>, // Index normal OBJ
}