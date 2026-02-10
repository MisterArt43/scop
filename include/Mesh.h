#ifndef MESH_H
# define MESH_H

# include <cstdint>
# include <memory>
# include <vector>
# include <map>

# include "VAO.h"
# include "VBO.h"
# include "EBO.h"
# include "OBJParser.h" // for Vertex
#include "Material.h"

class Mesh
{
	public:
		Mesh(const std::vector<Vertex>& vertices, const std::vector<std::uint32_t>& indices);

		void Bind();
		void Unbind();
		void Draw();
		void Delete();

		GLsizei getIndexCount() const;
		void loadFromOBJ(const std::string& filepath);

	private:
		VAO m_vao;
		std::unique_ptr<VBO> m_vbo;
		std::unique_ptr<EBO> m_ebo;
		//materials
		std::unordered_map<std::string, Material> m_materials;
		GLsizei m_indexCount;
		OBJParser m_parser;
};

#endif
