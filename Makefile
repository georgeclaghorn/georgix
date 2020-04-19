export ARCH=x86_64
export RUSTFLAGS=-C link-arg=-Tsrc/arch/$(ARCH)/linker.ld

TARGET=$(ARCH)-unknown-georgix-gnu.json

default: run

run:
	cargo xrun --target $(TARGET)

test:
	cargo xtest --target $(TARGET)

clean:
	cargo clean

.PHONY: default run clean
