#include <VulkanContext.hpp>
#include <stdexcept>
#include <iostream>

VulkanContext::VulkanContext() : window(nullptr), instance(VK_NULL_HANDLE), device(VK_NULL_HANDLE), 
                                graphicsQueue(VK_NULL_HANDLE), surface(VK_NULL_HANDLE), 
                                swapchain(VK_NULL_HANDLE), renderPass(VK_NULL_HANDLE), 
                                graphicsPipeline(VK_NULL_HANDLE), pipelineLayout(VK_NULL_HANDLE) {
}

VulkanContext::~VulkanContext() {
    cleanup();
}

void VulkanContext::initWindow(int width, int height, const char* title) {
    if (!glfwInit()) {
        throw std::runtime_error("Failed to initialize GLFW!");
    }

    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);  // Indique que nous n'utilisons pas OpenGL
    window = glfwCreateWindow(width, height, title, nullptr, nullptr);
    if (!window) {
        glfwTerminate();
        throw std::runtime_error("Failed to create GLFW window!");
    }
}

void VulkanContext::initVulkan()
{
    try
    {

        // Débogage pour vérifier l'initialisation des objets Vulkan
        std::cout << "Initializing Vulkan..." << std::endl;

        // Vérifier les objets Vulkan nécessaires


        std::cout << "Vulkan initialized successfully!" << std::endl;

        createSurface();       // Assurez-vous que la surface est valide
        createInstance();      // Assurez-vous que le VulkanInstance est valide
        
        createWindowSurface();
        if (surface == VK_NULL_HANDLE)
        {
            throw std::runtime_error("Surface is not initialized!");
        }

        pickPhysicalDevice();  // Vérifiez si le physicalDevice est bien sélectionné
        if (physicalDevice == VK_NULL_HANDLE)
        {
            throw std::runtime_error("Physical device is not initialized!");
        }



        createLogicalDevice(); // Vérifiez que le logicalDevice est créé
        createSwapchain();     // Vérifiez si le swapchain est correctement créé
        createRenderPass();
        createGraphicsPipeline();
        createFramebuffers();
        createCommandPool();
        createCommandBuffers();
    }
    catch (const std::exception &e)
    {
        std::cerr << "Erreur: " << e.what() << std::endl;

        const char *description;
        int code = glfwGetError(&description);

        if (description)
        {
            std::cerr << "Error: " << description << std::endl;
        }

        cleanup();
    }
}

void VulkanContext::createInstance() {
    VkApplicationInfo appInfo{};
    appInfo.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
    appInfo.pApplicationName = "Vulkan Triangle";
    appInfo.applicationVersion = VK_MAKE_API_VERSION(1, 0, 0, 0);
    appInfo.pEngineName = "No Engine";
    appInfo.engineVersion = VK_MAKE_API_VERSION(1, 0, 0, 0);
    appInfo.apiVersion = VK_API_VERSION_1_0;

    VkInstanceCreateInfo createInfo{};
    createInfo.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
    createInfo.pApplicationInfo = &appInfo;

    uint32_t glfwExtensionCount = 0;
    const char** glfwExtensions = glfwGetRequiredInstanceExtensions(&glfwExtensionCount);
    createInfo.enabledExtensionCount = glfwExtensionCount;
    createInfo.ppEnabledExtensionNames = glfwExtensions;

    if (vkCreateInstance(&createInfo, nullptr, &instance) != VK_SUCCESS) {
        throw std::runtime_error("Failed to create instance!");
    }
}

void VulkanContext::createSurface() {
    // Assurez-vous que GLFW a été initialisé avant d'appeler cette fonction
    if (!glfwInit()) {
        throw std::runtime_error("Failed to initialize GLFW!");
    }

    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);  // Vulkan, pas OpenGL
    window = glfwCreateWindow(800, 600, "Vulkan", nullptr, nullptr);
    if (!window) {
        glfwTerminate();
        throw std::runtime_error("Failed to create GLFW window!");
    }

    

    std::cout << "Surface created successfully!" << std::endl;
}

void VulkanContext::createWindowSurface() {
    // Vérifier les extensions Vulkan pour GLFW
    uint32_t glfwExtensionCount = 0;
    const char** glfwExtensions = glfwGetRequiredInstanceExtensions(&glfwExtensionCount);

    // Si des extensions nécessaires sont absentes, les ajouter à la liste d'extensions pour l'instance
    std::vector<const char*> extensions(glfwExtensions, glfwExtensions + glfwExtensionCount);
    
    if (glfwVulkanSupported()) {
        std::cout << "Vulkan is supported!" << std::endl;
    } else {
        std::cerr << "Error: Vulkan is not supported on this system!" << std::endl;
        throw std::runtime_error("Vulkan is not supported on this system!");
    }

    // Créez la surface Vulkan associée à la fenêtre GLFW
    VkResult result = glfwCreateWindowSurface(instance, window, nullptr, &surface);
    
    if (result != VK_SUCCESS) {
        std::cerr << "Error: Failed to create window surface! Vulkan result code: " << result << std::endl;
        throw std::runtime_error("Failed to create window surface!");
    }
}

void VulkanContext::pickPhysicalDevice() {
    uint32_t deviceCount = 0;
    vkEnumeratePhysicalDevices(instance, &deviceCount, nullptr);

    if (deviceCount == 0) {
        throw std::runtime_error("No Vulkan-supported physical devices found!");
    }

    std::vector<VkPhysicalDevice> devices(deviceCount);
    vkEnumeratePhysicalDevices(instance, &deviceCount, devices.data());

    bool deviceFound = false;  // Flag pour savoir si un périphérique compatible a été trouvé

    for (const auto& device : devices) {
        VkPhysicalDeviceProperties deviceProperties;
        vkGetPhysicalDeviceProperties(device, &deviceProperties);
        
        // Vérification si l'appareil est compatible
        if (isDeviceSuitable(device)) {
            physicalDevice = device;
            std::cout << "Selected Physical Device: " << deviceProperties.deviceName << std::endl;
            deviceFound = true;
            break;  // On arrête dès qu'un périphérique compatible est trouvé
        }
    }

    if (!deviceFound) {
        throw std::runtime_error("Failed to find a suitable physical device!");
    }
}

bool VulkanContext::isDeviceSuitable(VkPhysicalDevice device) {
    VkPhysicalDeviceProperties deviceProperties;
    vkGetPhysicalDeviceProperties(device, &deviceProperties);

    // Exemple de critère supplémentaire : vérifier que le périphérique prend en charge Vulkan 1.1 ou plus
    uint32_t apiVersion = deviceProperties.apiVersion;  // Récupérer la version de l'API Vulkan

    // Vérifier que le périphérique prend en charge Vulkan 1.1 ou supérieur
    if (VK_VERSION_MAJOR(apiVersion) < 1 || 
        (VK_VERSION_MAJOR(apiVersion) == 1 && VK_VERSION_MINOR(apiVersion) < 1)) {
        std::cout << "Device does not support Vulkan 1.1 or higher!" << std::endl;
        return false;
    }

    // Vérification si le périphérique supporte Vulkan
    VkPhysicalDeviceFeatures deviceFeatures;
    vkGetPhysicalDeviceFeatures(device, &deviceFeatures);

    // Vérification de la présence d'une file d'attente de présentation pour ce périphérique
    VkBool32 presentSupport = false;
    vkGetPhysicalDeviceSurfaceSupportKHR(device, 0, surface, &presentSupport);  // Index de file d'attente de présentation (0 ici)

    // Vérifier si le périphérique supporte les files d'attente graphiques
    VkQueueFamilyProperties queueFamilyProperties;
    uint32_t queueFamilyCount = 0;
    vkGetPhysicalDeviceQueueFamilyProperties(device, &queueFamilyCount, nullptr);
    std::vector<VkQueueFamilyProperties> queueFamilies(queueFamilyCount);
    vkGetPhysicalDeviceQueueFamilyProperties(device, &queueFamilyCount, queueFamilies.data());

    bool hasGraphicsQueue = false;
    bool hasPresentQueue = false;
    for (uint32_t i = 0; i < queueFamilyCount; i++) {
        if (queueFamilies[i].queueFlags & VK_QUEUE_GRAPHICS_BIT) {
            hasGraphicsQueue = true;
        }

        // Vérification si le périphérique peut gérer la surface de présentation
        VkBool32 presentSupportTmp = false;
        vkGetPhysicalDeviceSurfaceSupportKHR(device, i, surface, &presentSupportTmp);
        if (presentSupportTmp) {
            hasPresentQueue = true;
        }

        // Si nous avons à la fois une file d'attente graphique et une file de présentation, on peut utiliser ce périphérique
        if (hasGraphicsQueue && hasPresentQueue) {
            break;
        }
    }

    // Si le périphérique ne dispose pas des queues nécessaires, on ne l'accepte pas
    if (!hasGraphicsQueue || !hasPresentQueue || !presentSupport) {
        return false;
    }

    // Vous pouvez aussi filtrer par type de GPU ici (GPU discret vs intégré)
    if (deviceProperties.deviceType == VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU) {
        // GPU discret préféré (par exemple pour des performances graphiques optimales)
        return true;
    }

    // Retourner true pour le premier périphérique qui satisfait ces critères
    return false;
}

void VulkanContext::createLogicalDevice() {
    if (physicalDevice == VK_NULL_HANDLE) {
        throw std::runtime_error("Physical device is not initialized!");
    }

    uint32_t queueFamilyCount = 0;
    vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice, &queueFamilyCount, nullptr);
    if (queueFamilyCount == 0) {
        throw std::runtime_error("Failed to find queue families!");
    }

    std::vector<VkQueueFamilyProperties> queueFamilies(queueFamilyCount);
    vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice, &queueFamilyCount, queueFamilies.data());

    int graphicsQueueFamilyIndex = -1;
    for (uint32_t i = 0; i < queueFamilies.size(); i++) {
        if (queueFamilies[i].queueFlags & VK_QUEUE_GRAPHICS_BIT) {
            graphicsQueueFamilyIndex = i;
            break;
        }
    }

    if (graphicsQueueFamilyIndex == -1) {
        throw std::runtime_error("Failed to find a graphics queue family!");
    }

    float queuePriority = 1.0f;
    VkDeviceQueueCreateInfo queueCreateInfo{};
    queueCreateInfo.sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
    queueCreateInfo.queueFamilyIndex = graphicsQueueFamilyIndex;
    queueCreateInfo.queueCount = 1;
    queueCreateInfo.pQueuePriorities = &queuePriority;

    VkDeviceCreateInfo createInfo{};
    createInfo.sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
    createInfo.queueCreateInfoCount = 1;
    createInfo.pQueueCreateInfos = &queueCreateInfo;

    if (vkCreateDevice(physicalDevice, &createInfo, nullptr, &device) != VK_SUCCESS) {
        throw std::runtime_error("Failed to create logical device!");
    }

    vkGetDeviceQueue(device, graphicsQueueFamilyIndex, 0, &graphicsQueue);
}

void VulkanContext::createSwapchain() {
    if (physicalDevice == VK_NULL_HANDLE) {
        throw std::runtime_error("Physical device is not initialized!");
    }
    if (surface == VK_NULL_HANDLE) {
        throw std::runtime_error("Surface is not initialized!");
    }

    // Query surface capabilities
    VkSurfaceCapabilitiesKHR capabilities;
    if (vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physicalDevice, surface, &capabilities) != VK_SUCCESS) {
        throw std::runtime_error("Failed to get surface capabilities!");
    }

    uint32_t formatCount;
    if (vkGetPhysicalDeviceSurfaceFormatsKHR(physicalDevice, surface, &formatCount, nullptr) != VK_SUCCESS) {
        throw std::runtime_error("Failed to get surface formats!");
    }
    if (formatCount == 0) {
        throw std::runtime_error("No surface formats found!");
    }

    std::vector<VkSurfaceFormatKHR> formats(formatCount);
    if (vkGetPhysicalDeviceSurfaceFormatsKHR(physicalDevice, surface, &formatCount, formats.data()) != VK_SUCCESS) {
        throw std::runtime_error("Failed to get surface formats data!");
    }

    swapchainImageFormat = formats[0].format;

    // Create swapchain
    uint32_t imageCount = capabilities.minImageCount + 1;
    if (capabilities.maxImageCount > 0 && imageCount > capabilities.maxImageCount) {
        imageCount = capabilities.maxImageCount;
    }

    VkSwapchainCreateInfoKHR createInfo{};
    createInfo.sType = VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR;
    createInfo.surface = surface;
    createInfo.minImageCount = imageCount;
    createInfo.imageFormat = swapchainImageFormat;
    createInfo.imageExtent = capabilities.currentExtent;
    createInfo.imageArrayLayers = 1;
    createInfo.imageUsage = VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;
    createInfo.preTransform = VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR;
    createInfo.compositeAlpha = VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;
    createInfo.presentMode = VK_PRESENT_MODE_FIFO_KHR;
    createInfo.clipped = VK_TRUE;

    if (vkCreateSwapchainKHR(device, &createInfo, nullptr, &swapchain) != VK_SUCCESS) {
        throw std::runtime_error("Failed to create swapchain!");
    }

    // Get swapchain images
    uint32_t swapchainImageCount;
    if (vkGetSwapchainImagesKHR(device, swapchain, &swapchainImageCount, nullptr) != VK_SUCCESS) {
        throw std::runtime_error("Failed to get swapchain images!");
    }
    if (swapchainImageCount == 0) {
        throw std::runtime_error("No swapchain images found!");
    }
    swapchainImages.resize(swapchainImageCount);
    if (vkGetSwapchainImagesKHR(device, swapchain, &swapchainImageCount, swapchainImages.data()) != VK_SUCCESS) {
        throw std::runtime_error("Failed to get swapchain images data!");
    }

    // Create image views for swapchain images
    swapchainImageViews.resize(swapchainImages.size());
    for (size_t i = 0; i < swapchainImages.size(); ++i) {
        VkImageViewCreateInfo createInfo{};
        createInfo.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
        createInfo.image = swapchainImages[i];
        createInfo.viewType = VK_IMAGE_VIEW_TYPE_2D;
        createInfo.format = swapchainImageFormat;
        createInfo.components.r = VK_COMPONENT_SWIZZLE_IDENTITY;
        createInfo.components.g = VK_COMPONENT_SWIZZLE_IDENTITY;
        createInfo.components.b = VK_COMPONENT_SWIZZLE_IDENTITY;
        createInfo.components.a = VK_COMPONENT_SWIZZLE_IDENTITY;
        createInfo.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
        createInfo.subresourceRange.baseMipLevel = 0;
        createInfo.subresourceRange.levelCount = 1;
        createInfo.subresourceRange.baseArrayLayer = 0;
        createInfo.subresourceRange.layerCount = 1;

        if (vkCreateImageView(device, &createInfo, nullptr, &swapchainImageViews[i]) != VK_SUCCESS) {
            throw std::runtime_error("Failed to create image views!");
        }
    }
}

void VulkanContext::createRenderPass() {
    VkAttachmentDescription colorAttachment{};
    colorAttachment.format = swapchainImageFormat;
    colorAttachment.samples = VK_SAMPLE_COUNT_1_BIT;
    colorAttachment.loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR;
    colorAttachment.storeOp = VK_ATTACHMENT_STORE_OP_STORE;
    colorAttachment.stencilLoadOp = VK_ATTACHMENT_LOAD_OP_DONT_CARE;
    colorAttachment.stencilStoreOp = VK_ATTACHMENT_STORE_OP_DONT_CARE;
    colorAttachment.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
    colorAttachment.finalLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;

    VkAttachmentReference colorAttachmentRef{};
    colorAttachmentRef.attachment = 0;
    colorAttachmentRef.layout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;

    VkSubpassDescription subpass{};
    subpass.pipelineBindPoint = VK_PIPELINE_BIND_POINT_GRAPHICS;
    subpass.colorAttachmentCount = 1;
    subpass.pColorAttachments = &colorAttachmentRef;

    VkRenderPassCreateInfo renderPassInfo{};
    renderPassInfo.sType = VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO;
    renderPassInfo.attachmentCount = 1;
    renderPassInfo.pAttachments = &colorAttachment;
    renderPassInfo.subpassCount = 1;
    renderPassInfo.pSubpasses = &subpass;

    if (vkCreateRenderPass(device, &renderPassInfo, nullptr, &renderPass) != VK_SUCCESS) {
        throw std::runtime_error("Failed to create render pass!");
    }
}

void VulkanContext::createGraphicsPipeline() {
    std::vector<unsigned int> vertexShaderCode = { /* Vertex shader binary code */ };
    std::vector<unsigned int> fragmentShaderCode = { /* Fragment shader binary code */ };

    VkShaderModule vertexShaderModule = createShaderModule(convertToCharVector(vertexShaderCode));
    VkShaderModule fragmentShaderModule = createShaderModule(convertToCharVector(fragmentShaderCode));

    VkPipelineShaderStageCreateInfo vertShaderStageInfo{};
    vertShaderStageInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
    vertShaderStageInfo.stage = VK_SHADER_STAGE_VERTEX_BIT;
    vertShaderStageInfo.module = vertexShaderModule;
    vertShaderStageInfo.pName = "main";

    VkPipelineShaderStageCreateInfo fragShaderStageInfo{};
    fragShaderStageInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
    fragShaderStageInfo.stage = VK_SHADER_STAGE_FRAGMENT_BIT;
    fragShaderStageInfo.module = fragmentShaderModule;
    fragShaderStageInfo.pName = "main";

    VkPipelineShaderStageCreateInfo shaderStages[] = { vertShaderStageInfo, fragShaderStageInfo };

    // Configurer le pipeline de dessin (inclus vertex input, input assembly, etc.)
    // Voir Vulkan documentation pour des exemples détaillés de la configuration du pipeline graphique
}

void VulkanContext::createFramebuffers() {
    // Créer un framebuffer pour chaque image swapchain
}

void VulkanContext::createCommandBuffers() {
    // Allouer et enregistrer les commandes de dessin
}

void VulkanContext::createCommandPool() {
    // Créer un command pool
}
void VulkanContext::mainLoop() {
    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();
        // Vous pouvez ajouter d'autres traitements ici, comme la gestion de la présentation des images
    }
}

std::vector<char> VulkanContext::convertToCharVector(const std::vector<unsigned int>& code) {
    std::vector<char> charVec(code.begin(), code.end());
    return charVec;
}

VkShaderModule VulkanContext::createShaderModule(const std::vector<char>& code) {
    VkShaderModuleCreateInfo createInfo{};
    createInfo.sType = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;
    createInfo.codeSize = code.size();
    createInfo.pCode = reinterpret_cast<const uint32_t*>(code.data());

    VkShaderModule shaderModule;
    if (vkCreateShaderModule(device, &createInfo, nullptr, &shaderModule) != VK_SUCCESS) {
        throw std::runtime_error("Failed to create shader module!");
    }

    return shaderModule;
}

void VulkanContext::cleanup() {
    if (graphicsPipeline != VK_NULL_HANDLE) {
        vkDestroyPipeline(device, graphicsPipeline, nullptr);
        graphicsPipeline = VK_NULL_HANDLE;
    }

    if (renderPass != VK_NULL_HANDLE) {
        vkDestroyRenderPass(device, renderPass, nullptr);
        renderPass = VK_NULL_HANDLE;
    }

    if (swapchain != VK_NULL_HANDLE) {
        // for (auto framebuffer : swapChainFramebuffers) {
        //     vkDestroyFramebuffer(device, framebuffer, nullptr);
        // }
        vkDestroySwapchainKHR(device, swapchain, nullptr);
        swapchain = VK_NULL_HANDLE;
    }

    if (surface != VK_NULL_HANDLE) {
        vkDestroySurfaceKHR(instance, surface, nullptr);
        surface = VK_NULL_HANDLE;
    }

    if (device != VK_NULL_HANDLE) {
        vkDestroyDevice(device, nullptr);
        device = VK_NULL_HANDLE;
    }

    if (instance != VK_NULL_HANDLE) {
        vkDestroyInstance(instance, nullptr);
        instance = VK_NULL_HANDLE;
    }
}

