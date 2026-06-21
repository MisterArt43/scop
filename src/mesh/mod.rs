pub mod mesh;
pub mod material;
pub mod vertex;
mod parser;

// Ré-exporter pour que ton code extérieur n'ait pas besoin de faire `mesh::mesh::Mesh`
pub use mesh::{Mesh, SubMesh};
pub use material::Mtl;
pub use vertex::Vertex;