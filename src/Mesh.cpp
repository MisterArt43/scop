#include "../include/Mesh.h"

#include <cstddef> // offsetof

Mesh::Mesh(const std::vector<Vertex>& vertices, const std::vector<std::uint32_t>& indices)
	: m_vao()
	, m_vbo()
	, m_ebo()
	, m_indexCount(static_cast<GLsizei>(indices.size()))
{
	m_vao.Bind();
	m_vbo.reset(new VBO((GLfloat*)vertices.data(), static_cast<GLsizeiptr>(vertices.size() * sizeof(Vertex))));
	m_ebo.reset(new EBO((GLuint*)indices.data(), static_cast<GLsizeiptr>(indices.size() * sizeof(std::uint32_t))));

	m_vao.LinkAttrib(*m_vbo, 0, 3, GL_FLOAT, sizeof(Vertex), (void*)0); // position
	m_vao.LinkAttrib(*m_vbo, 1, 3, GL_FLOAT, sizeof(Vertex), (void*)offsetof(Vertex, normal)); // normal
	m_vao.LinkAttrib(*m_vbo, 2, 2, GL_FLOAT, sizeof(Vertex), (void*)offsetof(Vertex, uv)); // uv

	m_vao.Unbind();
	m_vbo->Unbind();
	m_ebo->Unbind();
}

void Mesh::Bind() { m_vao.Bind(); }

void Mesh::Unbind() { m_vao.Unbind(); }

GLsizei Mesh::getIndexCount() const { return m_indexCount; }

void Mesh::Draw()
{
	m_vao.Bind();
	glDrawElements(GL_TRIANGLES, m_indexCount, GL_UNSIGNED_INT, 0);
}

void Mesh::Delete()
{
	m_vao.Delete();
	if (m_vbo)
		m_vbo->Delete();
	if (m_ebo)
		m_ebo->Delete();
}

void Mesh::loadFromOBJ(const std::string& filepath)
{
	m_parser.loadFromFile(filepath);
	const std::vector<Vertex>& verticesData = m_parser.getVertices();
	const std::vector<uint32_t>& indicesData = m_parser.getIndices();

	m_indexCount = static_cast<GLsizei>(indicesData.size());

	m_vao.Bind();
	if (m_vbo)
		m_vbo->Delete();
	m_vbo.reset(new VBO((GLfloat*)verticesData.data(), static_cast<GLsizeiptr>(verticesData.size() * sizeof(Vertex))));
	if (m_ebo)
		m_ebo->Delete();
	m_ebo.reset(new EBO((GLuint*)indicesData.data(), static_cast<GLsizeiptr>(indicesData.size() * sizeof(std::uint32_t))));

	m_vao.LinkAttrib(*m_vbo, 0, 3, GL_FLOAT, sizeof(Vertex), (void*)0); // position
	m_vao.LinkAttrib(*m_vbo, 1, 3, GL_FLOAT, sizeof(Vertex), (void*)offsetof(Vertex, normal)); // normal
	m_vao.LinkAttrib(*m_vbo, 2, 2, GL_FLOAT, sizeof(Vertex), (void*)offsetof(Vertex, uv)); // uv

	m_vao.Unbind();
	m_vbo->Unbind();
	m_ebo->Unbind();
}