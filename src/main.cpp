// main.cpp

#include <VulkanContext.hpp>
#include <scope.hpp>

int main() {
    try {
        VulkanContext vulkanContext;
        // vulkanContext.initWindow(800, 600, "Vulkan Triangle");
        vulkanContext.initVulkan();
        // vulkanContext.mainLoop();
    } catch (const std::exception& e) {
        std::cerr << "Erreur: " << e.what() << std::endl;
        return EXIT_FAILURE;
    }

    return EXIT_SUCCESS;
}
