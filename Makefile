ARCH=x86_64
TARGET=$(ARCH)-unknown-georgix-gnu

QEMU=qemu-system-$(ARCH)

default: run

run: target/georgix.iso
	$(QEMU) -cdrom target/georgix.iso -serial stdio -display none

clean:
	cargo clean

.PHONY: default run clean


target/georgix.iso: grub.cfg target/$(TARGET)/release/georgix
	-rm -rf target/iso
	mkdir -p target/iso/boot/grub
	cp grub.cfg target/iso/boot/grub/
	cp target/$(TARGET)/release/georgix target/iso/georgix
	grub-mkrescue -o target/georgix.iso target/iso/

target/$(TARGET)/release/georgix: Cargo.toml Cargo.lock $(shell find src -name *.rs) src/arch/$(ARCH)/linker.ld $(TARGET).json
	cargo xrustc --release --target $(TARGET).json -- -C link-arg=-Tsrc/arch/$(ARCH)/linker.ld
