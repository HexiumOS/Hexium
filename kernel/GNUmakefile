include ../tools/utils.mk

override OUTPUT := kernel

$(call USER_VARIABLE, KARCH, x86_64)

.PHONY: all clean distclean

ifeq ($(RUST_TARGET),)
    override RUST_TARGET := $(KARCH)-unknown-none
endif

ifeq ($(RUST_PROFILE),)
    override RUST_PROFILE := dev
endif

override RUST_PROFILE_SUBDIR := $(RUST_PROFILE)
ifeq ($(RUST_PROFILE),dev)
    override RUST_PROFILE_SUBDIR := debug
endif

all:
	RUSTFLAGS="-C relocation-model=static -C force-frame-pointers=yes" cargo build --target $(RUST_TARGET) --profile $(RUST_PROFILE)
	cp target/$(RUST_TARGET)/$(RUST_PROFILE_SUBDIR)/$$(cd target/$(RUST_TARGET)/$(RUST_PROFILE_SUBDIR) && find -maxdepth 1 -perm -111 -type f) kernel

#
# Clean intermediate and build files
#
clean:
	cargo clean
	rm -f kernel

#
# Clean other files
#
disclean: clean