#include "../include/Input.h"

#include <cstdio>
#include <cstring>

Input::Input()
	: m_hasPendingObjPath(false)
	, m_pendingObjPath()
	, m_argvObjPaths()
	, m_nextArgvIndex(0)
{
	std::memset(m_keys, 0, sizeof(m_keys));
}

void Input::setObjPathsFromArgv(int argc, char** argv)
{
	m_argvObjPaths.clear();
	m_nextArgvIndex = 0;
	for (int i = 1; i < argc; ++i)
	{
		if (argv[i] && *argv[i])
			m_argvObjPaths.push_back(std::string(argv[i]));
	}
	m_nextArgvIndex = m_argvObjPaths.empty() ? 0 : 1;
}

const std::vector<std::string>& Input::argvObjPaths() const
{
	return m_argvObjPaths;
}

bool Input::keyDown(int glfwKey) const
{
	if (glfwKey < 0 || glfwKey >= KEY_MAX)
		return false;
	return m_keys[glfwKey];
}

bool Input::hasPendingObjPath() const
{
	return m_hasPendingObjPath;
}

std::string Input::consumePendingObjPath()
{
	m_hasPendingObjPath = false;
	std::string out = m_pendingObjPath;
	m_pendingObjPath.clear();
	return out;
}

void Input::onKey(GLFWwindow* window, int key, int action)
{
	if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS)
	{
		glfwSetWindowShouldClose(window, GLFW_TRUE);
		return;
	}

	if (key == GLFW_KEY_R && action == GLFW_PRESS)
	{
		setPendingObjPath(std::string());
		return;
	}

	if (key == GLFW_KEY_TAB && action == GLFW_PRESS)
	{
		glfwSetInputMode(window, GLFW_CURSOR, GLFW_CURSOR_HIDDEN);
		if (m_nextArgvIndex < m_argvObjPaths.size())
			setPendingObjPath(m_argvObjPaths[m_nextArgvIndex++]);
		else
		{
			const std::string picked = openObjFileDialogZenity();
			if (!picked.empty())
				setPendingObjPath(picked);
		}
		glfwSetInputMode(window, GLFW_CURSOR, GLFW_CURSOR_DISABLED);
		return;
	}

	const bool isDown = (action == GLFW_PRESS || action == GLFW_REPEAT);
	const bool isUp = (action == GLFW_RELEASE);
	if (!isDown && !isUp)
		return;

	if (key >= 0 && key < KEY_MAX)
		m_keys[key] = isDown;
}

std::string Input::openObjFileDialogZenity()
{
	const char* cmd = "zenity --file-selection --title=\"Open OBJ\" --file-filter=\"*.obj\" 2>/dev/null";
	FILE* pipe = popen(cmd, "r");
	if (!pipe)
		return std::string();
	char buffer[4096];
	std::string line;
	if (fgets(buffer, sizeof(buffer), pipe))
		line = buffer;
	pclose(pipe);
	while (!line.empty() && (line.back() == '\n' || line.back() == '\r'))
		line.pop_back();
	return line;
}

void Input::setPendingObjPath(const std::string& path)
{
	m_pendingObjPath = path;
	m_hasPendingObjPath = true;
}
