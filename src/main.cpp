#include <iostream>
#include <glad/glad.h>
#include <GLFW/glfw3.h>

#include <cmath>
#include <memory>
#include <string>
#include <unordered_map>
#include <vector>

#include "../include/Camera.h"
#include "../include/Input.h"
#include "../include/Material.h"
#include "../include/Mesh.h"
#include "../include/OBJParser.h"
#include "../include/shaderClass.h"

struct AppState
{
	Camera camera;
	Input input;
};

static void framebufferSizeCallback(GLFWwindow* window, int width, int height)
{
	glViewport(0, 0, width, height);
	AppState* state = static_cast<AppState*>(glfwGetWindowUserPointer(window));
	if (!state)
		return;
	if (height <= 0)
		return;
	state->camera.setAspect(static_cast<float>(width) / static_cast<float>(height));
}

static void keyCallback(GLFWwindow* window, int key, int scancode, int action, int mods)
{
	(void)scancode;
	(void)mods;
	AppState* state = static_cast<AppState*>(glfwGetWindowUserPointer(window));
	if (!state)
		return;
	state->input.onKey(window, key, action);
}

int main(int argc, char** argv)
{
	try
	{
		AppState app{};
		app.input.setObjPathsFromArgv(argc, argv);

		glfwInit();
		glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
		glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
		glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
		glfwWindowHint(GLFW_DEPTH_BITS, 24);
		glfwWindowHint(GLFW_SAMPLES, 4);

		GLFWwindow* window = glfwCreateWindow(800, 800, "Abucia OpenGL", NULL, NULL);
		if (window == NULL)
		{
			std::cout << "Failed to create GLFW window" << std::endl;
			glfwTerminate();
			return -1;
		}
		glfwMakeContextCurrent(window);
		glfwSetWindowUserPointer(window, &app);
		glfwSetKeyCallback(window, keyCallback);
		glfwSetFramebufferSizeCallback(window, framebufferSizeCallback);

		gladLoadGL();

		glEnable(GL_DEPTH_TEST);
		glDepthFunc(GL_LESS);
		glClearDepth(1.0);
		glDisable(GL_CULL_FACE);
		glEnable(GL_MULTISAMPLE);

		int fbWidth = 800;
		int fbHeight = 800;
		glfwGetFramebufferSize(window, &fbWidth, &fbHeight);
		framebufferSizeCallback(window, fbWidth, fbHeight);

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

		const std::string defaultObj = "ressources/Intergalactic_Spaceship-(Wavefront).obj";
		if (!app.input.argvObjPaths().empty())	loadObjOrThrow(app.input.argvObjPaths()[0]);
		else
		{
			loadObjOrThrow(defaultObj);
			std::cout << "Tip: pass .obj paths: ./scop a.obj b.obj\n";
			std::cout << "Tip: TAB opens file picker (needs zenity).\n";
		}

		const math::Mat4 model = math::identity();

		float lastTime = static_cast<float>(glfwGetTime());
		while (!glfwWindowShouldClose(window))
		{
			if (app.input.hasPendingObjPath())
			{
				const std::string nextPath = app.input.consumePendingObjPath();
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

			const float now = static_cast<float>(glfwGetTime());
			const float deltaTime = now - lastTime;
			lastTime = now;

			const float velocity = app.camera.moveSpeed * deltaTime;
			if (app.input.keyDown(GLFW_KEY_W)) app.camera.moveForward(velocity);
			if (app.input.keyDown(GLFW_KEY_S)) app.camera.moveForward(-velocity);
			if (app.input.keyDown(GLFW_KEY_D)) app.camera.moveRight(velocity);
			if (app.input.keyDown(GLFW_KEY_A)) app.camera.moveRight(-velocity);
			if (app.input.keyDown(GLFW_KEY_Q)) app.camera.moveUp(-velocity);
			if (app.input.keyDown(GLFW_KEY_E)) app.camera.moveUp(velocity);

			float yawDelta = 0.0f;
			float pitchDelta = 0.0f;
			if (app.input.keyDown(GLFW_KEY_LEFT)) yawDelta -= app.camera.lookSpeedRadians * deltaTime;
			if (app.input.keyDown(GLFW_KEY_RIGHT)) yawDelta += app.camera.lookSpeedRadians * deltaTime;
			if (app.input.keyDown(GLFW_KEY_UP)) pitchDelta += app.camera.lookSpeedRadians * deltaTime;
			if (app.input.keyDown(GLFW_KEY_DOWN)) pitchDelta -= app.camera.lookSpeedRadians * deltaTime;
			if (yawDelta != 0.0f || pitchDelta != 0.0f)
				app.camera.rotateYawPitch(yawDelta, pitchDelta);

			glClearColor(0.07f, 0.13f, 0.17f, 1.0f);
			glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

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
			material.setMat4("uView", app.camera.getViewMatrix());
			material.setMat4("uProjection", app.camera.getProjectionMatrix());

			if (mesh)
				mesh->Draw();

			glfwSwapBuffers(window);
			glfwPollEvents();
		}

		if (mesh)
			mesh->Delete();
		shaderProgram.Delete();
		glfwDestroyWindow(window);
		glfwTerminate();
		return 0;
	}
	catch (const std::exception& e)
	{
		std::cerr << e.what() << std::endl;
		return 1;
	}
}