arch ?= x86_64
target ?= $(arch)-rux
build_dir ?= build
run_flags ?= -serial stdio
profile ?= dev

ifeq ($(profile),dev)
	profile_dir := debug
else
	profile_dir := $(profile)
endif

kernel := $(build_dir)/kernel-$(profile)-$(arch).bin
iso := $(build_dir)/rux-$(profile)-$(arch).iso

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
rust_source_files := $(shell find src -iname '*.rs')
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
	$(build_dir)/arch/$(arch)/%.o, $(assembly_source_files))
rust_lib := $(build_dir)/$(target)/$(profile_dir)/librux.a

.PHONY: all clean check doc run iso

all: $(kernel)

clean:
	-@rm -r build 2> /dev/null
	@cargo clean --target-dir $(build_dir)

check:
	@cargo -Z unstable-options check --target $(target).json --profile=$(profile) --target-dir $(build_dir)

doc:
	@cargo -Z unstable-options doc --target $(target).json --target-dir $(build_dir) --document-private-items

# Run the compiled OS with Qemu
run: $(iso)
	@qemu-system-x86_64 $(run_flags) -boot d -cdrom $(iso)

iso: $(iso)

# Create the ISO using grub-mkrescue
$(iso): $(kernel) $(grub_cfg)
	@mkdir -p $(build_dir)/isofiles/boot/grub
	@cp $(kernel) $(build_dir)/isofiles/boot/kernel.bin
	@cp $(grub_cfg) $(build_dir)/isofiles/boot/grub
	@grub-mkrescue -o $(iso) $(build_dir)/isofiles 2> /dev/null
	@rm -r $(build_dir)/isofiles

# Link the kernel
$(kernel): $(rust_lib) $(assembly_object_files) $(linker_script)
	@ld.lld --gc-sections -n -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_lib)

# Compile the Rust part of the kernel
${rust_lib}: $(rust_source_files) $(target).json Cargo.toml Cargo.lock .cargo/config.toml 
	@cargo -Z unstable-options build --target $(target).json --profile=$(profile) --target-dir $(build_dir)

# Assemble the architecture-specific assembly files
$(build_dir)/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -f elf64 $< -o $@

-include $(build_dir)/$(target)/$(profile_dir)/librux.d
