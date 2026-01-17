#include "../include/Material.h"

Material::Material(Shader& shader)
	: m_shader(&shader)
	, m_locationCache()
{
}

void Material::use()
{
	m_shader->Activate();
}

Shader& Material::shader() { return *m_shader; }
const Shader& Material::shader() const { return *m_shader; }

GLint Material::getLocation(const std::string& name)
{
	std::unordered_map<std::string, GLint>::iterator it = m_locationCache.find(name);
	if (it != m_locationCache.end())
		return it->second;

	const GLint loc = glGetUniformLocation(m_shader->ID, name.c_str());
	m_locationCache.insert(std::make_pair(name, loc));
	return loc;
}

void Material::setFloat(const std::string& name, float value)
{
	use();
	glUniform1f(getLocation(name), value);
}

void Material::setVec2(const std::string& name, float x, float y)
{
	use();
	glUniform2f(getLocation(name), x, y);
}

void Material::setVec3(const std::string& name, float x, float y, float z)
{
	use();
	glUniform3f(getLocation(name), x, y, z);
}

void Material::setVec4(const std::string& name, float x, float y, float z, float w)
{
	use();
	glUniform4f(getLocation(name), x, y, z, w);
}

void Material::setInt(const std::string& name, int value)
{
	use();
	glUniform1i(getLocation(name), value);
}

void Material::setMat4(const std::string& name, const math::Mat4& value)
{
	use();
	glUniformMatrix4fv(getLocation(name), 1, GL_FALSE, value.data());
}
