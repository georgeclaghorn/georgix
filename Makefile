export ARCH=x86_64
TARGET=$(ARCH)-unknown-georgix-gnu.json

default: run

run:
	RUSTFLAGS="-C link-arg=-Tsrc/arch/$(ARCH)/linker.ld" cargo xrun --target $(TARGET)

clean:
	cargo clean

.PHONY: default run clean
