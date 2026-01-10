#include<iostream>
#include<glad/glad.h>
#include<GLFW/glfw3.h>
#include <cmath>
#include <unordered_map>

#include "../include/shaderClass.h"
#include "../include/OBJParser.h"
#include "../include/Camera.h"
#include "../include/Mesh.h"
#include "../include/Material.h"


// Vertices coordinates
GLfloat vertices[] =
{ //               COORDINATES                  /       COLORS           //
	-0.5f, -0.5f * float(sqrt(3)) * 1 / 3, 0.0f,     0.8f, 0.3f,  0.02f, // Lower left corner
	 0.5f, -0.5f * float(sqrt(3)) * 1 / 3, 0.0f,     0.8f, 0.3f,  0.02f, // Lower right corner
	 0.0f,  0.5f * float(sqrt(3)) * 2 / 3, 0.0f,     1.0f, 0.6f,  0.32f, // Upper corner
	-0.25f, 0.5f * float(sqrt(3)) * 1 / 6, 0.0f,     0.9f, 0.45f, 0.17f, // Inner left
	 0.25f, 0.5f * float(sqrt(3)) * 1 / 6, 0.0f,     0.9f, 0.45f, 0.17f, // Inner right
	 0.0f, -0.5f * float(sqrt(3)) * 1 / 3, 0.0f,     0.8f, 0.3f,  0.02f  // Inner down
};

// Indices for vertices order
GLuint indices[] =
{
	0, 3, 5, // Lower left triangle
	3, 2, 4, // Lower right triangle
	5, 4, 1 // Upper triangle
};

struct InputState
{
	bool w;
	bool a;
	bool s;
	bool d;
	bool q;
	bool e;
	bool left;
	bool right;
	bool up;
	bool down;
};

struct AppState
{
	Camera camera;
	InputState input;
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

	if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS)
	{
		glfwSetWindowShouldClose(window, GLFW_TRUE);
		return;
	}

	const bool isDown = (action == GLFW_PRESS || action == GLFW_REPEAT);
	const bool isUp = (action == GLFW_RELEASE);
	if (!isDown && !isUp)
		return;

	switch (key)
	{
		case GLFW_KEY_W: state->input.w = isDown; break;
		case GLFW_KEY_A: state->input.a = isDown; break;
		case GLFW_KEY_S: state->input.s = isDown; break;
		case GLFW_KEY_D: state->input.d = isDown; break;
		case GLFW_KEY_Q: state->input.q = isDown; break;
		case GLFW_KEY_E: state->input.e = isDown; break;
		case GLFW_KEY_LEFT: state->input.left = isDown; break;
		case GLFW_KEY_RIGHT: state->input.right = isDown; break;
		case GLFW_KEY_UP: state->input.up = isDown; break;
		case GLFW_KEY_DOWN: state->input.down = isDown; break;
		default: break;
	}
}

int main()
{
	try
	{
	AppState app{};
	app.input = InputState{false, false, false, false, false, false, false, false, false, false};
	// Initialize GLFW
	glfwInit();

	// Tell GLFW what version of OpenGL we are using 
	// In this case we are using OpenGL 3.3
	glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
	glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
	// Tell GLFW we are using the CORE profile
	// So that means we only have the modern functions
	glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

	// Request a depth buffer + MSAA (must be set before creating the window)
	glfwWindowHint(GLFW_DEPTH_BITS, 24);
	glfwWindowHint(GLFW_SAMPLES, 4);

	// Create a GLFWwindow object of 800 by 800 pixels, naming it "Abucia OpenGL"
	GLFWwindow* window = glfwCreateWindow(800, 800, "Abucia OpenGL", NULL, NULL);
	// Error check if the window fails to create
	if (window == NULL)
	{
		std::cout << "Failed to create GLFW window" << std::endl;
		glfwTerminate();
		return -1;
	}
	// Introduce the window into the current context
	glfwMakeContextCurrent(window);
	glfwSetWindowUserPointer(window, &app);
	glfwSetKeyCallback(window, keyCallback);
	glfwSetFramebufferSizeCallback(window, framebufferSizeCallback);

	//Load GLAD so it configures OpenGL
	gladLoadGL();

	// Render state
	glEnable(GL_DEPTH_TEST);
	glDepthFunc(GL_LESS);
	glClearDepth(1.0);
	glDisable(GL_CULL_FACE);
	// Set initial viewport + camera aspect using real framebuffer size
	int fbWidth = 800;
	int fbHeight = 800;
	glfwGetFramebufferSize(window, &fbWidth, &fbHeight);
	framebufferSizeCallback(window, fbWidth, fbHeight);

	Shader shaderProgram("shaders/basic.vert", "shaders/basic.frag");
	Material material(shaderProgram);

	OBJParser objParser;
	if (!objParser.loadFromFile("ressources/42.obj"))
	{
		throw std::runtime_error("Failed to load OBJ file.");
	}
	Vec3 kd{};
	const bool hasKd = objParser.tryGetActiveDiffuse(kd);
	Vec3 boundsMin = objParser.getBoundsMin();
	Vec3 boundsMax = objParser.getBoundsMax();
	const bool hasUVs = objParser.hasUVs();
	Vec3 ka{0.1f, 0.1f, 0.1f};
	if (!objParser.getActiveMaterialName().empty())
	{
		const std::unordered_map<std::string, MTLMaterial>& mats = objParser.getMaterials();
		std::unordered_map<std::string, MTLMaterial>::const_iterator it = mats.find(objParser.getActiveMaterialName());
		if (it != mats.end())
			ka = it->second.Ka;
	}
	const std::vector<Vertex>& verticesData = objParser.getVertices();
	const std::vector<uint32_t>& indicesData = objParser.getIndices();
	(void)verticesData;
	(void)indicesData;

	Mesh mesh(verticesData, indicesData);
	const math::Mat4 model = math::identity();

	// antialiasing msaa
	glEnable(GL_MULTISAMPLE);

	// Main while loop
	float lastTime = static_cast<float>(glfwGetTime());
	while (!glfwWindowShouldClose(window))
	{
		const float now = static_cast<float>(glfwGetTime());
		const float deltaTime = now - lastTime;
		lastTime = now;

		const float velocity = app.camera.moveSpeed * deltaTime;
		if (app.input.w) app.camera.moveForward(velocity);
		if (app.input.s) app.camera.moveForward(-velocity);
		if (app.input.d) app.camera.moveRight(velocity);
		if (app.input.a) app.camera.moveRight(-velocity);
		// Q = down, E = up
		if (app.input.q) app.camera.moveUp(-velocity);
		if (app.input.e) app.camera.moveUp(velocity);

		float yawDelta = 0.0f;
		float pitchDelta = 0.0f;
		if (app.input.left) yawDelta -= app.camera.lookSpeedRadians * deltaTime;
		if (app.input.right) yawDelta += app.camera.lookSpeedRadians * deltaTime;
		if (app.input.up) pitchDelta += app.camera.lookSpeedRadians * deltaTime;
		if (app.input.down) pitchDelta -= app.camera.lookSpeedRadians * deltaTime;
		if (yawDelta != 0.0f || pitchDelta != 0.0f)
			app.camera.rotateYawPitch(yawDelta, pitchDelta);

		// Specify the color of the background
		glClearColor(0.07f, 0.13f, 0.17f, 1.0f);
		// Clean the back buffer and assign the new color to it
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
		const math::Mat4 view = app.camera.getViewMatrix();
		const math::Mat4 proj = app.camera.getProjectionMatrix();
		material.setMat4("uModel", model);
		material.setMat4("uView", view);
		material.setMat4("uProjection", proj);


		mesh.Draw();
		// Swap the back buffer with the front buffer
		glfwSwapBuffers(window);
		// Take care of all GLFW events
		glfwPollEvents();
	}


	mesh.Delete();

	shaderProgram.Delete();
	// Delete window before ending the program
	glfwDestroyWindow(window);
	// Terminate GLFW before ending the program
	glfwTerminate();
	return 0;
	}
	catch (const std::exception& e)
	{
		std::cerr << e.what() << std::endl;
		return 1;
	}
}