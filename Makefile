KERNEL_BIN = kernel/target/x86_64-unknown-none/release/scipion-kernel
ISO_IMAGE = scipion.iso
ISO_ROOT = iso_root

.PHONY: all run clean setup_limine

all: run

setup_limine:
	@echo "=== [1/3] Налаштування завантажувача Limine ==="
	rm -rf limine
	git clone https://github.com/limine-bootloader/limine.git --branch=v6.x-binary-side --depth=1 limine || \
	git clone https://github.com/limine-bootloader/limine.git --branch=v7.x-binary --depth=1 limine || \
	git clone https://github.com/limine-bootloader/limine.git --depth=1 limine
	$(MAKE) -C limine

$(KERNEL_BIN): kernel/src/main.rs kernel/Cargo.toml kernel/linker.ld
	@echo "=== [2/3] Компіляція ядра Scipion (Rust nightly) ==="
	cd kernel && cargo build --release

$(ISO_IMAGE): $(KERNEL_BIN) iso_root/limine.cfg
	@echo "=== [3/3] Створення завантажувального образу $(ISO_IMAGE) ==="
	mkdir -p $(ISO_ROOT)/boot
	cp $(KERNEL_BIN) $(ISO_ROOT)/boot/
	cp limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin $(ISO_ROOT)/
	mkdir -p $(ISO_ROOT)/EFI/BOOT
	cp limine/BOOTX64.EFI $(ISO_ROOT)/EFI/BOOT/
	cp limine/BOOTIA32.EFI $(ISO_ROOT)/EFI/BOOT/
	xorriso -as mkisofs -b limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		$(ISO_ROOT) -o $(ISO_IMAGE)
	./limine/limine bios-install $(ISO_IMAGE)

run: $(ISO_IMAGE)
	@echo "=== Запуск Scipion OS в емуляторі QEMU ==="
	qemu-system-x86_64 \
		-M q35 \
		-m 512M \
		-cdrom $(ISO_IMAGE) \
		-serial stdio \
		-vga std

clean:
	cd kernel && cargo clean
	rm -rf $(ISO_ROOT)/boot
	rm -f $(ISO_IMAGE)
