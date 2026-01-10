#ifndef INPUT_H
# define INPUT_H

# include <GLFW/glfw3.h>

# include <cstddef>
# include <string>
# include <vector>

class Input
{
	public:
		Input();

		void setObjPathsFromArgv(int argc, char** argv);
		const std::vector<std::string>& argvObjPaths() const;

		bool keyDown(int glfwKey) const;

		bool hasPendingObjPath() const;
		std::string consumePendingObjPath();

		void onKey(GLFWwindow* window, int key, int action);

	private:
		// GLFW key codes are in [0, GLFW_KEY_LAST].
		enum { KEY_MAX = GLFW_KEY_LAST + 1 };

		static std::string openObjFileDialogZenity();
		void setPendingObjPath(const std::string& path);

	private:
		bool m_keys[KEY_MAX];
		bool m_hasPendingObjPath;
		std::string m_pendingObjPath;
		std::vector<std::string> m_argvObjPaths;
		std::size_t m_nextArgvIndex;
};

#endif
