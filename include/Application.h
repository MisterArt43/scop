#pragma once

#include "Camera.h"
#include "Input.h"

struct GLFWwindow;

#include <string>
#include <vector>

class Application
{
public:
	Application();
	~Application();

	void setObjPathsFromArgv(int argc, char** argv);

	void initWindowAndGL(int width, int height, const char* title);
	void shutdown();

	GLFWwindow* window() const;
	bool shouldClose() const;
	void swapBuffers();
	void pollEvents();
	float time() const;

	void beginFrame(float r, float g, float b, float a);
	void update(float deltaTime);

	const std::vector<std::string>& argvObjPaths() const;
	bool hasPendingObjPath() const;
	std::string consumePendingObjPath();

	void attachToWindow(GLFWwindow* window);

	Camera& camera();
	const Camera& camera() const;
	Input& input();
	const Input& input() const;

	void onFramebufferSize(GLFWwindow* window, int width, int height);
	void onKey(GLFWwindow* window, int key, int scancode, int action, int mods);
	void onCursorPos(GLFWwindow* window, double xpos, double ypos);

	static void framebufferSizeCallback(GLFWwindow* window, int width, int height);
	static void keyCallback(GLFWwindow* window, int key, int scancode, int action, int mods);
	static void cursorPosCallback(GLFWwindow* window, double xpos, double ypos);

private:
	Camera m_camera;
	Input m_input;
	GLFWwindow* m_window;
	bool m_glfwInitialized;
	bool m_hasLastCursorPos;
	double m_lastCursorX;
	double m_lastCursorY;
	float m_pendingMouseYawRadians;
	float m_pendingMousePitchRadians;
	float m_mouseSensitivityRadiansPerPixel;
};
