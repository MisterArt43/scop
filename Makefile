# Sources et répertoires
SRCS = src/main.cpp \
       src/VulkanContext.cpp

OBJ_DIR = build/obj
DEP_DIR = build/dep
INC_DIR = inc

OBJS = $(addprefix $(OBJ_DIR)/, $(notdir $(SRCS:.cpp=.o)))
DEPS = $(addprefix $(DEP_DIR)/, $(notdir $(SRCS:.cpp=.d)))

CXX = g++
CXXFLAGS = -Wall -Wextra -std=c++11 -g3 -I$(INC_DIR) -I$(HOME)/local/include
LDFLAGS = -L$(HOME)/local/lib -lglfw -lvulkan

# Cible principale
all: vulkan

vulkan: $(OBJS)
	$(CXX) $(CXXFLAGS) -o $@ $^ $(LDFLAGS)

# Création des répertoires pour les objets et dépendances
$(OBJ_DIR) $(DEP_DIR):
	@mkdir -p $@

# Règle pour les fichiers objets
$(OBJ_DIR)/%.o: src/%.cpp $(DEP_DIR)/%.d | $(OBJ_DIR)
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Règle pour les fichiers de dépendance
$(DEP_DIR)/%.d: src/%.cpp | $(DEP_DIR)
	$(CXX) -MM $(CXXFLAGS) $< | sed 's|^.*:|$(OBJ_DIR)/$(notdir $@):|' > $@

# Nettoyage
clean:
	rm -rf $(OBJ_DIR) $(DEP_DIR) vulkan

# Rebuild complet
re: clean all

# Inclusion des fichiers de dépendance, s'ils existent
-include $(DEPS)

.PHONY: all clean re
