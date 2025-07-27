use crate::geometry::Point3D;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub id: u32,
    pub point: Point3D,
}

// #[derive(Debug, Clone)]
// pub struct Edge {
//     pub id: u32,
//     pub start_vertex: Vertex,
//     pub end_vertex: Vertex,
//     pub curve: CurveType,
//     pub sense: bool, // true = same direction as curve, false = opposite
// }

// #[derive(Debug, Clone)]
// pub struct Face {
//     pub id: u32,
//     pub surface: SurfaceType,
//     pub outer_loop: Vec<Edge>, // Edge IDs forming outer boundary
//     pub inner_loops: Vec<Vec<Edge>>, // Edge IDs for holes
//     pub sense: bool, // Surface normal direction
// }

// #[derive(Debug, Clone)]
// pub struct Body {
//     pub id: u32,
//     pub vertices: Vec<Vertex>,
//     pub edges: Vec<Edge>,
//     pub faces: Vec<Face>,
// }
