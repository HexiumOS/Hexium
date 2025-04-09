MAKEFLAGS += -rR
.SUFFIXES:

# Assembler and linker settings
AS := nasm
LD := ld

# Assembler flags
ASFLAGS := -f elf64

# Output directories
BIN_DIR := bin

# Find all assembly source files
ASM_SRCS := $(wildcard */main.asm)
PROGRAMS := $(patsubst %/main.asm,$(BIN_DIR)/%,$(ASM_SRCS))

.PHONY: all
all: $(BIN_DIR) $(PROGRAMS)

# Create output directory
$(BIN_DIR):
	mkdir -p $(BIN_DIR)

# Pattern rule for building programs
$(BIN_DIR)/%: %/main.asm
	@echo "Building $@"
	@mkdir -p $(dir $@)
	$(AS) $(ASFLAGS) $< -o $@.o
	$(LD) $@.o -o $@
	@rm $@.o

.PHONY: clean
clean:
	rm -rf $(BIN_DIR)

.PHONY: install
install: all
	mkdir -p ../initrd/applications
	cp -v $(PROGRAMS) ../initrd/applications/

.PHONY: list
list:
	@echo "Available programs:"
	@for prog in $(PROGRAMS); do \
		echo "  $${prog#$(BIN_DIR)/}"; \
	done
