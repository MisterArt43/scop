pub mod mesh;
mod parser;
pub mod vertex;

// Ré-exporter pour que ton code extérieur n'ait pas besoin de faire `mesh::mesh::Mesh`
pub use crate::material::Mtl;
pub use mesh::{Mesh, SubMesh};
pub use vertex::Vertex;
