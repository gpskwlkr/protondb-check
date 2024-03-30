# Define the targets and their respective filenames
TARGET_LINUX = x86_64-unknown-linux-gnu
TARGET_WINDOWS = x86_64-pc-windows-gnu
OUT_DIR_LINUX = target/$(TARGET_LINUX)/release
OUT_DIR_WINDOWS = target/$(TARGET_WINDOWS)/release
PROJECT_NAME = protondb-check
TAR_FILE = $(PROJECT_NAME)_$(TARGET_LINUX).tar.gz
ZIP_FILE = $(PROJECT_NAME)_$(TARGET_WINDOWS).zip

# Default target
all: build pack

linux: 
	cargo build --release --target=$(TARGET_LINUX)
	tar -czvf $(TAR_FILE) -C $(OUT_DIR_LINUX) $(PROJECT_NAME)

windows:
	cargo build --release --target=$(TARGET_WINDOWS)
	zip -r $(ZIP_FILE) $(OUT_DIR_WINDOWS)/$(PROJECT_NAME).exe

# Build the project for Linux and Windows
build:
	cargo build --release --target=$(TARGET_LINUX)
	cargo build --release --target=$(TARGET_WINDOWS)

# Pack the built binaries
pack:
	tar -czvf $(TAR_FILE) -C $(OUT_DIR_LINUX) $(PROJECT_NAME)
	zip -r $(ZIP_FILE) $(OUT_DIR_WINDOWS)/$(PROJECT_NAME).exe

# Clean up the build artifacts
clean:
	cargo clean
	rm -f $(TAR_FILE) $(ZIP_FILE)

.PHONY: all build pack clean

