#include "../include/Scene.h"

Scene::Scene()
	: m_objects()
	, m_nextId(1)
{
}

Scene::ObjectId Scene::createObject(const math::Mat4& model)
{
	const ObjectId id = m_nextId++;
	m_objects.push_back(Object{id, model});
	return id;
}

bool Scene::destroyObject(ObjectId id)
{
	for (std::vector<Object>::iterator it = m_objects.begin(); it != m_objects.end(); ++it)
	{
		if (it->id == id)
		{
			m_objects.erase(it);
			return true;
		}
	}
	return false;
}

void Scene::clear()
{
	m_objects.clear();
}

Scene::Object* Scene::getObject(ObjectId id)
{
	for (std::vector<Object>::iterator it = m_objects.begin(); it != m_objects.end(); ++it)
	{
		if (it->id == id)
			return &(*it);
	}
	return 0;
}

const Scene::Object* Scene::getObject(ObjectId id) const
{
	for (std::vector<Object>::const_iterator it = m_objects.begin(); it != m_objects.end(); ++it)
	{
		if (it->id == id)
			return &(*it);
	}
	return 0;
}

std::size_t Scene::size() const
{
	return m_objects.size();
}

const std::vector<Scene::Object>& Scene::objects() const
{
	return m_objects;
}
