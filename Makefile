arch ?= x86_64
target ?= $(arch)-rux
kernel := build/kernel-$(arch).bin
iso := build/rux-$(arch).iso

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))
rust_lib := target/$(target)/debug/librux.a

.PHONY: all clean run iso

all: $(kernel)

clean:
	-@rm -r build
	@cargo clean

# Run the compiled OS with Qemu
run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

# Create the ISO using grub-mkrescue
$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
	@rm -r build/isofiles

# Link the kernel
$(kernel): $(rust_lib) $(assembly_object_files) $(linker_script)
	@ld --gc-sections -n -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_lib)

# Compile the Rust part of the kernel
${rust_lib}: $(shell find src -iname '*.rs') $(target).json Cargo.toml Cargo.lock .cargo/config.toml 
	@cargo build --target $(target).json

# Assemble the architecture-specific assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -f elf64 $< -o $@

-include target/$(target)/debug/librux.d
