#ifndef MESH_H
# define MESH_H

# include <cstdint>
# include <memory>
# include <vector>

# include "VAO.h"
# include "VBO.h"
# include "EBO.h"
# include "OBJParser.h" // for Vertex

class Mesh
{
	public:
		Mesh(const std::vector<Vertex>& vertices, const std::vector<std::uint32_t>& indices);

		void Bind();
		void Unbind();
		void Draw();
		void Delete();

		GLsizei getIndexCount() const;

	private:
		VAO m_vao;
		std::unique_ptr<VBO> m_vbo;
		std::unique_ptr<EBO> m_ebo;
		GLsizei m_indexCount;
};

#endif
