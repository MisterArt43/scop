#include <iostream>

#include <glad/glad.h>
#include <memory>
#include <string>
#include <unordered_map>
#include <vector>

#include "../include/Application.h"
#include "../include/Material.h"
#include "../include/Mesh.h"
#include "../include/OBJParser.h"
#include "../include/shaderClass.h"

int main(int argc, char** argv)
{
	try
	{
		Application app{};
		app.setObjPathsFromArgv(argc, argv);
		app.initWindowAndGL(800, 800, "Abucia OpenGL");

		Shader shaderProgram("shaders/basic.vert", "shaders/basic.frag");
		Material material(shaderProgram);

		OBJParser objParser;
		std::unique_ptr<Mesh> mesh;
		Vec3 kd{};
		bool hasKd = false;
		Vec3 boundsMin{};
		Vec3 boundsMax{};
		bool hasUVs = false;
		Vec3 ka{0.1f, 0.1f, 0.1f};
		std::string currentObjPath;

		auto loadObjOrThrow = [&](const std::string& path) {
			if (!objParser.loadFromFile(path))
				throw std::runtime_error("Failed to load OBJ file: " + path);
			currentObjPath = path;
			hasKd = objParser.tryGetActiveDiffuse(kd);
			boundsMin = objParser.getBoundsMin();
			boundsMax = objParser.getBoundsMax();
			hasUVs = objParser.hasUVs();
			ka = Vec3{0.1f, 0.1f, 0.1f};
			if (!objParser.getActiveMaterialName().empty())
			{
				const std::unordered_map<std::string, MTLMaterial>& mats = objParser.getMaterials();
				std::unordered_map<std::string, MTLMaterial>::const_iterator it = mats.find(objParser.getActiveMaterialName());
				if (it != mats.end())
					ka = it->second.Ka;
			}
			const std::vector<Vertex>& verticesData = objParser.getVertices();
			const std::vector<uint32_t>& indicesData = objParser.getIndices();
			if (mesh)
				mesh->Delete();
			mesh.reset(new Mesh(verticesData, indicesData));
		};

		const std::string defaultObj = "ressources/42.obj";
		if (!app.argvObjPaths().empty())	loadObjOrThrow(app.argvObjPaths()[0]);
		else
		{
			loadObjOrThrow(defaultObj);
			std::cout << "Tip: pass .obj paths: ./scop a.obj b.obj\n";
			std::cout << "Tip: TAB opens file picker (needs zenity).\n";
		}

		const math::Mat4 model = math::identity();

		float lastTime = app.time();
		while (!app.shouldClose())
		{
			if (app.hasPendingObjPath())
			{
				const std::string nextPath = app.consumePendingObjPath();
				try
				{
					loadObjOrThrow(nextPath);
					std::cout << "Now displaying: " << currentObjPath << "\n";
				}
				catch (const std::exception& e)
				{
					std::cerr << e.what() << "\n";
				}
			}

			const float now = app.time();
			const float deltaTime = now - lastTime;
			lastTime = now;
			app.update(deltaTime);

			app.beginFrame(0.07f, 0.13f, 0.17f, 1.0f);

			material.use();
			material.setFloat("scale", 0.5f);
			material.setInt("uUseGradient", 1);
			material.setInt("uGradientUseUV", hasUVs ? 1 : 0);
			material.setFloat("uMinY", boundsMin.y);
			material.setFloat("uMaxY", boundsMax.y);
			if (hasKd)
			{
				material.setVec3("uColorA", ka.x, ka.y, ka.z);
				material.setVec3("uColorB", kd.x, kd.y, kd.z);
			}
			else
			{
				material.setVec3("uColorA", 0.10f, 0.20f, 0.60f);
				material.setVec3("uColorB", 0.90f, 0.40f, 0.10f);
			}
			material.setVec3("uColor", 1.0f, 1.0f, 1.0f);
			material.setMat4("uModel", model);
			material.setMat4("uView", app.camera().getViewMatrix());
			material.setMat4("uProjection", app.camera().getProjectionMatrix());

			if (mesh)
				mesh->Draw();

			app.swapBuffers();
			app.pollEvents();
		}

		if (mesh)
			mesh->Delete();
		shaderProgram.Delete();
		return 0;
	}
	catch (const std::exception& e)
	{
		std::cerr << e.what() << std::endl;
		return 1;
	}
}