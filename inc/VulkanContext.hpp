#ifndef VULKAN_CONTEXT_HPP
#define VULKAN_CONTEXT_HPP

#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>
#include <vulkan/vulkan.h>
#include <vector>
#include <string>

class VulkanContext {
public:
    VulkanContext();
    ~VulkanContext();

    // Initialisation de la fenêtre GLFW et de Vulkan
    void initWindow(int width, int height, const char* title);
    void initVulkan();

    // Boucle principale de l'application
    void mainLoop();

private:
    // Membres pour la gestion de la fenêtre et de Vulkan
    GLFWwindow* window;
    VkInstance instance;
    VkPhysicalDevice physicalDevice;
    VkDevice device;
    VkQueue graphicsQueue;
    VkSurfaceKHR surface;
    VkSwapchainKHR swapchain;
    VkFormat swapchainImageFormat;
    VkExtent2D swapchainExtent;
    std::vector<VkImage> swapchainImages;
    std::vector<VkImageView> swapchainImageViews;  // Déclarez ici
    VkRenderPass renderPass;
    VkPipeline graphicsPipeline;
    VkPipelineLayout pipelineLayout;
    VkCommandPool commandPool;

    // Fonctions pour l'initialisation de Vulkan
    void createInstance();
    void createSurface();
    void pickPhysicalDevice();
    void createLogicalDevice();
    void createSwapchain();
    void createRenderPass();
    void createGraphicsPipeline();
    void createFramebuffers();
    void createCommandPool();
    void createCommandBuffers();
    void createWindowSurface();
    bool isDeviceSuitable(VkPhysicalDevice device);

    // Fonction utilitaire pour charger des modules de shader
    VkShaderModule createShaderModule(const std::vector<char>& code);
    std::vector<char> readFile(const std::string& filename);

    // Cleanup de Vulkan
    void cleanup();

    // Fonction utilitaire pour convertir un vecteur de données
    std::vector<char> convertToCharVector(const std::vector<unsigned int>& code);
};

#endif // VULKAN_CONTEXT_HPP
