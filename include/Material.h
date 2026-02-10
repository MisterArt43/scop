#ifndef MATERIAL_H
# define MATERIAL_H

# include <string>
# include <unordered_map>

# include "Math3D.h"
# include "shaderClass.h"

class Material
{
	public:
		Material(Shader& shader);

		void use();

		void setFloat(const std::string& name, float value);
		void setVec2(const std::string& name, float x, float y);
		void setVec3(const std::string& name, float x, float y, float z);
		void setVec4(const std::string& name, float x, float y, float z, float w);
		void setInt(const std::string& name, int value);
		void setMat4(const std::string& name, const math::Mat4& value);

		Shader& shader();
		const Shader& shader() const;

	private:
		Shader* m_shader;
		std::unordered_map<std::string, GLint> m_locationCache;

		GLint getLocation(const std::string& name);
};

#endif
