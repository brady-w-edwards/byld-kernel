#[derive(Debug, Clone)]
pub struct Vertex {
    pub id: u32,
    pub point: Point3D,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub id: u32,
    pub start_vertex: u32,
    pub end_vertex: u32,
    pub curve: CurveType,
    pub sense: bool, // true = same direction as curve, false = opposite
}

#[derive(Debug, Clone)]
pub struct Face {
    pub id: u32,
    pub surface: SurfaceType,
    pub outer_loop: Vec<u32>, // Edge IDs forming outer boundary
    pub inner_loops: Vec<Vec<u32>>, // Edge IDs for holes
    pub sense: bool, // Surface normal direction
}

#[derive(Debug, Clone)]
pub struct Body {
    pub id: u32,
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
    pub transform: Option<Transform>,
}