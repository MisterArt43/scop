#ifndef SCENE_H
# define SCENE_H

# include <cstdint>
# include <vector>

# include "Math3D.h"

class Scene
{
	public:
		using ObjectId = std::uint32_t;

		struct Object
		{
			ObjectId id;
			math::Mat4 model;
		};

		Scene();

		ObjectId createObject(const math::Mat4& model = math::identity());
		bool destroyObject(ObjectId id);
		void clear();

		Object* getObject(ObjectId id);
		const Object* getObject(ObjectId id) const;

		std::size_t size() const;
		const std::vector<Object>& objects() const;

	private:
		std::vector<Object> m_objects;
		ObjectId m_nextId;
};

#endif
