MAKEFLAGS += -rR
.SUFFIXES:

override OUTPUT := kernel

override USER_VARIABLE = $(if $(filter $(origin $(1)),default undefined),$(eval override $(1) := $(2)))

$(call USER_VARIABLE,KARCH,x86_64)

ifeq ($(RUST_TARGET),)
    override RUST_TARGET := $(KARCH)-unknown-none
	ifeq ($(KARCH),riscv64)
    	override RUST_TARGET := riscv64gc-unknown-none-elf
	endif
endif

ifeq ($(RUST_PROFILE),)
    override RUST_PROFILE := dev
endif

override RUST_PROFILE_SUBDIR := $(RUST_PROFILE)
ifeq ($(RUST_PROFILE),dev)
    override RUST_PROFILE_SUBDIR := debug
endif

.PHONY: all
all:
	RUSTFLAGS="-C relocation-model=static" cargo build --target $(RUST_TARGET) --profile $(RUST_PROFILE)
	cp target/$(RUST_TARGET)/$(RUST_PROFILE_SUBDIR)/hexium_os kernel

test:
	RUSTFLAGS="-C relocation-model=static" cargo build --tests --target $(RUST_TARGET) --profile $(RUST_PROFILE)
	find . -type f -name 'hexium_os-*' ! -name '*.*' -printf '%T@ %p\n' | sort -nr | head -n1 | cut -d' ' -f2-
	
	shell find . -type f -name 'hexium_os-*' ! -name '*.*' -printf '%T@ %p\n' | sort -nr | head -n1 | cut -d' ' -f2-
	echo $(find . -type f -name 'hexium_os-*' ! -name '*.*' -printf '%T@ %p\n' | sort -nr | head -n1 | cut -d' ' -f2-)
	echo '$(find . -type f -name 'hexium_os-*' ! -name '*.*' -printf '%T@ %p\n' | sort -nr | head -n1 | cut -d' ' -f2-)'
	echo "$(find . -type f -name 'hexium_os-*' ! -name '*.*' -printf '%T@ %p\n' | sort -nr | head -n1 | cut -d' ' -f2-)"
	echo $(shell find . -type f -name 'hexium_os-*' ! -name '*.*' -printf '%T@ %p\n' | sort -nr | head -n1 | cut -d' ' -f2-)
	echo '$(shell find . -type f -name 'hexium_os-*' ! -name '*.*' -printf '%T@ %p\n' | sort -nr | head -n1 | cut -d' ' -f2-)'
	echo "$(shell find . -type f -name 'hexium_os-*' ! -name '*.*' -printf '%T@ %p\n' | sort -nr | head -n1 | cut -d' ' -f2-)"
	cp "$(shell find . -type f -name 'hexium_os-*' ! -name '*.*' -printf '%T@ %p\n' | sort -nr | head -n1 | cut -d' ' -f2-)" kernel-test

.PHONY: clean
clean:
	cargo clean
	rm -rf kernel
	rm -rf kernel-test

.PHONY: distclean
distclean: clean
