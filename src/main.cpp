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
		GLuint textureId = 0;
		bool hasTexture = false;
		Vec3 kd{};
		bool hasKd = false;
		Vec3 boundsMin{};
		Vec3 boundsMax{};
		bool hasUVs = false;
		Vec3 ka{0.1f, 0.1f, 0.1f};
		std::string currentObjPath;

		auto loadObjOrThrow = [&](const std::string& path) {
			std::string actualPath = path;
			if (actualPath.empty())
				actualPath = "ressources/42.obj";
			if (!objParser.loadFromFile(actualPath))
				throw std::runtime_error("Failed to load OBJ file: " + actualPath);
			currentObjPath = actualPath;
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
			std::vector<Vertex> verticesData = objParser.getVertices();
			const std::vector<uint32_t>& indicesData = objParser.getIndices();
			const MTLMaterial* mat = objParser.getResolvedActiveMaterial();
			const bool hasTexPixels = (mat && mat->textureWidth > 0 && mat->textureHeight > 0 && !mat->textureData.empty());

			// // Debug UV range; some models use UVs outside [0,1] (tiling), which with repeat can look "decalé".
			// if (!verticesData.empty() && hasUVs)
			// {
			// 	float minU = verticesData[0].uv.x;
			// 	float maxU = verticesData[0].uv.x;
			// 	float minV = verticesData[0].uv.y;
			// 	float maxV = verticesData[0].uv.y;
			// 	for (size_t i = 1; i < verticesData.size(); ++i)
			// 	{
			// 		const float u = verticesData[i].uv.x;
			// 		const float v = verticesData[i].uv.y;
			// 		if (u < minU) minU = u;
			// 		if (u > maxU) maxU = u;
			// 		if (v < minV) minV = v;
			// 		if (v > maxV) maxV = v;
			// 	}
			// 	std::cout << "UV range: U[" << minU << ", " << maxU << "] V[" << minV << ", " << maxV << "]\n";

			// 	// If we display a single texture, ensure U uses the full [0,1] range.
			// 	// Some assets store UVs in a sub-rect of the texture, which looks like a big left/right offset.
			// 	const float eps = 0.0005f;
			// 	if (hasTexPixels && (minU < -eps || maxU > 1.0f + eps || minU > eps || maxU < 1.0f - eps))
			// 	{
			// 		const float rangeU = maxU - minU;
			// 		if (rangeU > 0.00001f)
			// 		{
			// 			for (size_t i = 0; i < verticesData.size(); ++i)
			// 			{
			// 				verticesData[i].uv.x = (verticesData[i].uv.x - minU) / rangeU;
			// 			}
			// 			std::cout << "U remapped to [0,1] for texture display\n";
			// 		}
			// 	}
			// }
			if (mesh)
				mesh->Delete();
			mesh.reset(new Mesh(verticesData, indicesData));

			// (Re)build texture from MTLMaterial::textureData (PPM pixels)
			if (textureId != 0)
			{
				glDeleteTextures(1, &textureId);
				textureId = 0;
			}
			hasTexture = false;
			if (hasTexPixels)
			{
				const int w = mat->textureWidth;
				const int h = mat->textureHeight;
				const size_t expected = static_cast<size_t>(w) * static_cast<size_t>(h);
				if (mat->textureData.size() >= expected)
				{
					std::vector<unsigned char> rgb;
					rgb.resize(expected * 3u);
					for (size_t i = 0; i < expected; ++i)
					{
						const Pixel& p = mat->textureData[i];
						rgb[i * 3u + 0u] = static_cast<unsigned char>(p.r < 0 ? 0 : (p.r > 255 ? 255 : p.r));
						rgb[i * 3u + 1u] = static_cast<unsigned char>(p.g < 0 ? 0 : (p.g > 255 ? 255 : p.g));
						rgb[i * 3u + 2u] = static_cast<unsigned char>(p.b < 0 ? 0 : (p.b > 255 ? 255 : p.b));
					}

					glGenTextures(1, &textureId);
					glBindTexture(GL_TEXTURE_2D, textureId);
					glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
					glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
					glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR);
					glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
					glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
					glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB, w, h, 0, GL_RGB, GL_UNSIGNED_BYTE, rgb.data());
					glGenerateMipmap(GL_TEXTURE_2D);
					glBindTexture(GL_TEXTURE_2D, 0);
					hasTexture = true;
				}
			}
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
			const int useTexture = (hasTexture && hasUVs && textureId != 0) ? 1 : 0;
			material.setInt("uUseTexture", useTexture);
			if (useTexture)
			{
				// UV transform controls (identity by default). If texture doesn't align, try uUvMode=1/2/4/5/6/7.
				material.setInt("uUvMode", 0);
				material.setVec2("uUvScale", 1.0f, 1.0f);
				material.setVec2("uUvOffset", 0.0f, 0.0f);

				glActiveTexture(GL_TEXTURE0);
				glBindTexture(GL_TEXTURE_2D, textureId);
				material.setInt("uTexture", 0);
				material.setInt("uUvMode", 2);
			}
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
			if (useTexture)
				glBindTexture(GL_TEXTURE_2D, 0);

			app.swapBuffers();
			app.pollEvents();
		}

		if (mesh)
			mesh->Delete();
		if (textureId != 0)
			glDeleteTextures(1, &textureId);
		shaderProgram.Delete();
		return 0;
	}
	catch (const std::exception& e)
	{
		std::cerr << e.what() << std::endl;
		return 1;
	}
}