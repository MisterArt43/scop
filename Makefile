NAME        := scop

# Compilateurs
CXX         := c++
CC          := cc

# Flags
CXXFLAGS    := -Wall -Wextra -Werror -MMD -MP -g3
CFLAGS      := -Wall -Wextra -Werror -MMD -MP -g3

# Dossiers
SRC_DIR     := src
LIB_DIR     := lib
OBJ_DIR     := obj
INC_DIR     := include

# GLFW
GLFW_DIR    := $(LIB_DIR)/glfw
GLFW_INC    := -I$(GLFW_DIR)/include
GLFW_LIB    := -L$(GLFW_DIR)/lib -lglfw

# Includes
INCLUDES    := -I$(INC_DIR) -I$(LIB_DIR)/include $(GLFW_INC)

# Sources
CPP_SRCS    := $(wildcard $(SRC_DIR)/*.cpp)
C_SRCS      := $(LIB_DIR)/src/glad.c
SRCS        := $(CPP_SRCS) $(C_SRCS)

# Objets + deps
OBJS        := $(SRCS:%=$(OBJ_DIR)/%.o)
DEPS        := $(OBJS:.o=.d)

# Link
LDFLAGS     := $(GLFW_LIB) -lGL -ldl -lm -lpthread

# ================= RULES =================

all: $(NAME)

$(NAME): $(OBJS)
	$(CXX) $(OBJS) -o $@ $(LDFLAGS)

# C++
$(OBJ_DIR)/%.cpp.o: %.cpp
	@mkdir -p $(dir $@)
	$(CXX) $(CXXFLAGS) $(INCLUDES) -c $< -o $@

# C
$(OBJ_DIR)/%.c.o: %.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@

clean:
	rm -rf $(OBJ_DIR)

fclean: clean
	rm -f $(NAME)

re: fclean all

-include $(DEPS)

.PHONY: all clean fclean re
