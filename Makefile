NAME := scop
CARGO := cargo

ifeq ($(OS),Windows_NT)
	EXE := .exe
	COPY := copy /Y
	RM := del /F /Q
else
	EXE :=
	COPY := cp
	RM := rm -f
endif

DEBUG_BIN := target/debug/$(NAME)$(EXE)
RELEASE_BIN := target/release/$(NAME)$(EXE)

$(NAME):
	$(CARGO) build --bin $(NAME)
	$(COPY) $(DEBUG_BIN) $(NAME)$(EXE)

all: $(NAME)

debug:
	$(CARGO) build --bin $(NAME)
	$(COPY) $(DEBUG_BIN) $(NAME)$(EXE)

release:
	$(CARGO) build --release --bin $(NAME)
	$(COPY) $(RELEASE_BIN) $(NAME)$(EXE)

run: $(NAME)
	./$(NAME)$(EXE)

check:
	$(CARGO) check --bin $(NAME)

clean:
	$(CARGO) clean

fclean: clean
	-$(RM) $(NAME)$(EXE)

re: fclean all

.PHONY: all debug release run check clean fclean re
