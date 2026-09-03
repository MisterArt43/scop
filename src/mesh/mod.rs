pub mod mesh;
pub mod submesh;
pub mod actor;
mod parser;
pub mod vertex;

// Ré-exporter pour que ton code extérieur n'ait pas besoin de faire `mesh::mesh::Mesh`
pub use mesh::Mesh;
pub use submesh::SubMesh;
pub use vertex::Vertex;
pub use actor::Actor;