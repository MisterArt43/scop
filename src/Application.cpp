#include <glad/glad.h>
#include <GLFW/glfw3.h>

#include <stdexcept>

#include "../include/Application.h"

Application::Application()
	: m_window(NULL)
	, m_glfwInitialized(false)
	, m_hasLastCursorPos(false)
	, m_lastCursorX(0.0)
	, m_lastCursorY(0.0)
	, m_pendingMouseYawRadians(0.0f)
	, m_pendingMousePitchRadians(0.0f)
	, m_mouseSensitivityRadiansPerPixel(0.0025f)
{
}

Application::~Application()
{
	shutdown();
}

void Application::setObjPathsFromArgv(int argc, char** argv)
{
	m_input.setObjPathsFromArgv(argc, argv);
}

void Application::initWindowAndGL(int width, int height, const char* title)
{
	shutdown();

	if (glfwInit() == GLFW_FALSE)
		throw std::runtime_error("glfwInit failed");
	m_glfwInitialized = true;

	glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
	glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
	glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
	glfwWindowHint(GLFW_DEPTH_BITS, 24);
	glfwWindowHint(GLFW_SAMPLES, 4);

	m_window = glfwCreateWindow(width, height, title, NULL, NULL);
	if (m_window == NULL)
	{
		shutdown();
		throw std::runtime_error("Failed to create GLFW window");
	}
	glfwMakeContextCurrent(m_window);

	if (gladLoadGL() == 0)
	{
		shutdown();
		throw std::runtime_error("Failed to load GLAD");
	}

	glEnable(GL_DEPTH_TEST);
	glDepthFunc(GL_LESS);
	glClearDepth(1.0);
	glDisable(GL_CULL_FACE);
	glEnable(GL_MULTISAMPLE);

	attachToWindow(m_window);
	glfwSetInputMode(m_window, GLFW_CURSOR, GLFW_CURSOR_DISABLED);

	int fbWidth = width;
	int fbHeight = height;
	glfwGetFramebufferSize(m_window, &fbWidth, &fbHeight);
	onFramebufferSize(m_window, fbWidth, fbHeight);
}

void Application::shutdown()
{
	if (m_window)
	{
		glfwDestroyWindow(m_window);
		m_window = NULL;
	}
	if (m_glfwInitialized)
	{
		glfwTerminate();
		m_glfwInitialized = false;
	}
}

GLFWwindow* Application::window() const
{
	return m_window;
}

bool Application::shouldClose() const
{
	if (!m_window)
		return true;
	return glfwWindowShouldClose(m_window) != 0;
}

void Application::swapBuffers()
{
	if (m_window)
		glfwSwapBuffers(m_window);
}

void Application::pollEvents()
{
	glfwPollEvents();
}

float Application::time() const
{
	return static_cast<float>(glfwGetTime());
}

void Application::beginFrame(float r, float g, float b, float a)
{
	glClearColor(r, g, b, a);
	glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
}

void Application::update(float deltaTime)
{
	const float velocity = m_camera.moveSpeed * deltaTime;
	if (m_input.keyDown(GLFW_KEY_W)) m_camera.moveForward(velocity);
	if (m_input.keyDown(GLFW_KEY_S)) m_camera.moveForward(-velocity);
	if (m_input.keyDown(GLFW_KEY_D)) m_camera.moveRight(velocity);
	if (m_input.keyDown(GLFW_KEY_A)) m_camera.moveRight(-velocity);
	if (m_input.keyDown(GLFW_KEY_Q)) m_camera.moveUp(-velocity);
	if (m_input.keyDown(GLFW_KEY_E)) m_camera.moveUp(velocity);

	float yawDelta = m_pendingMouseYawRadians;
	float pitchDelta = m_pendingMousePitchRadians;
	m_pendingMouseYawRadians = 0.0f;
	m_pendingMousePitchRadians = 0.0f;
	if (m_input.keyDown(GLFW_KEY_LEFT)) yawDelta -= m_camera.lookSpeedRadians * deltaTime;
	if (m_input.keyDown(GLFW_KEY_RIGHT)) yawDelta += m_camera.lookSpeedRadians * deltaTime;
	if (m_input.keyDown(GLFW_KEY_UP)) pitchDelta += m_camera.lookSpeedRadians * deltaTime;
	if (m_input.keyDown(GLFW_KEY_DOWN)) pitchDelta -= m_camera.lookSpeedRadians * deltaTime;
	if (yawDelta != 0.0f || pitchDelta != 0.0f)
		m_camera.rotateYawPitch(yawDelta, pitchDelta);
	
	if (m_input.keyDown(GLFW_KEY_LEFT_SHIFT)) glPolygonMode( GL_FRONT_AND_BACK, GL_LINE );
	else glPolygonMode( GL_FRONT_AND_BACK, GL_FILL );
}

const std::vector<std::string>& Application::argvObjPaths() const
{
	return m_input.argvObjPaths();
}

bool Application::hasPendingObjPath() const
{
	return m_input.hasPendingObjPath();
}

std::string Application::consumePendingObjPath()
{
	return m_input.consumePendingObjPath();
}

void Application::attachToWindow(GLFWwindow* window)
{
	m_window = window;
	glfwSetWindowUserPointer(window, this);
	glfwSetKeyCallback(window, Application::keyCallback);
	glfwSetFramebufferSizeCallback(window, Application::framebufferSizeCallback);
	glfwSetCursorPosCallback(window, Application::cursorPosCallback);
	m_hasLastCursorPos = false;
}

Camera& Application::camera()
{
	return m_camera;
}

const Camera& Application::camera() const
{
	return m_camera;
}

Input& Application::input()
{
	return m_input;
}

const Input& Application::input() const
{
	return m_input;
}

void Application::onFramebufferSize(GLFWwindow* window, int width, int height)
{
	(void)window;
	glViewport(0, 0, width, height);
	if (height <= 0)
		return;
	m_camera.setAspect(static_cast<float>(width) / static_cast<float>(height));
}

void Application::onKey(GLFWwindow* window, int key, int scancode, int action, int mods)
{
	(void)scancode;
	(void)mods;
	m_input.onKey(window, key, action);
}

void Application::onCursorPos(GLFWwindow* window, double xpos, double ypos)
{
	(void)window;
	if (!m_hasLastCursorPos)
	{
		m_hasLastCursorPos = true;
		m_lastCursorX = xpos;
		m_lastCursorY = ypos;
		return;
	}

	const double dx = xpos - m_lastCursorX;
	const double dy = ypos - m_lastCursorY;
	m_lastCursorX = xpos;
	m_lastCursorY = ypos;

	m_pendingMouseYawRadians += static_cast<float>(dx) * m_mouseSensitivityRadiansPerPixel;
	// Screen Y grows downward; moving mouse up should pitch up.
	m_pendingMousePitchRadians += static_cast<float>(-dy) * m_mouseSensitivityRadiansPerPixel;
}

void Application::framebufferSizeCallback(GLFWwindow* window, int width, int height)
{
	Application* state = static_cast<Application*>(glfwGetWindowUserPointer(window));
	if (!state)
		return;
	state->onFramebufferSize(window, width, height);
}

void Application::keyCallback(GLFWwindow* window, int key, int scancode, int action, int mods)
{
	Application* state = static_cast<Application*>(glfwGetWindowUserPointer(window));
	if (!state)
		return;
	state->onKey(window, key, scancode, action, mods);
}

void Application::cursorPosCallback(GLFWwindow* window, double xpos, double ypos)
{
	Application* state = static_cast<Application*>(glfwGetWindowUserPointer(window));
	if (!state)
		return;
	state->onCursorPos(window, xpos, ypos);
}
