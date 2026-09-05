pub mod actor;
pub mod mesh;
mod parser;
pub mod submesh;
pub mod vertex;

// Ré-exporter pour que ton code extérieur n'ait pas besoin de faire `mesh::mesh::Mesh`
pub use actor::Actor;
pub use mesh::Mesh;
pub use submesh::SubMesh;
pub use vertex::Vertex;
