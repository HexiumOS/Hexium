include tools/utils.mk

$(call USER_VARIABLE, KARCH, x86_64)
$(call USER_VARIABLE, QEMUFLAGS, -m 512M -M smm=off -serial stdio -d int -D qemu_log.log -no-reboot -no-shutdown)

override IMAGE_NAME := hexium-$(KARCH)

.PHONY: all run kernel bootloader clean disclean

all: $(IMAGE_NAME).iso

#
# Run the OS
#
run: run-$(KARCH)

#
# Run the 64-bit (x86_64) version of the OS
#
run-x86_64: ovmf/ovmf-code-$(KARCH).fd ovmf/ovmf-vars-$(KARCH).fd $(IMAGE_NAME).iso
	qemu-system-$(KARCH) \
		-M q35 \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-$(KARCH).fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-$(KARCH).fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS)

#
# Build the kernel
#
kernel:
	$(MAKE) -C kernel

#
# Create the ISO image
#
$(IMAGE_NAME).iso: kernel bootloader
	rm -rf iso_root
	mkdir -p iso_root/boot
	cp -v kernel/kernel iso_root/boot/
	mkdir -p iso_root/boot/limine
	cp -v limine.conf iso_root/boot/limine/
	mkdir -p iso_root/EFI/BOOT
	cp -v limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin iso_root/boot/limine/
	cp -v limine/BOOTX64.EFI iso_root/EFI/BOOT/
	cp -v limine/BOOTIA32.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin			\
		-no-emul-boot -boot-load-size 4 -boot-info-table			\
		--efi-boot boot/limine/limine-uefi-cd.bin					\
		-efi-boot-part --efi-boot-image --protective-msdos-label	\
		iso_root -o $(IMAGE_NAME).iso
	./limine/limine bios-install $(IMAGE_NAME).iso
	rm -rf iso_root

#
# Clean intermediate files
#
clean:
	$(MAKE) -C kernel clean
	rm -rf iso_root *.iso


#
# Clean downloaded files
#
distclean: clean
	$(MAKE) -C kernel distclean
	rm -rf ovmf

#
# Download the OVMF firmware
#
ovmf/ovmf-code-$(KARCH).fd:
	mkdir -p ovmf
	curl -Lo $@ https://github.com/osdev0/edk2-ovmf-nightly/releases/latest/download/ovmf-code-$(KARCH).fd

#
# Download the OVMF variables
#
ovmf/ovmf-vars-$(KARCH).fd:
	mkdir -p ovmf
	curl -Lo $@ https://github.com/osdev0/edk2-ovmf-nightly/releases/latest/download/ovmf-vars-$(KARCH).fd

#
# Build the bootloader (Limine)
#
bootloader:
	$(MAKE) -C limine