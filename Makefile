ifeq ($(OS),Windows_NT)
	VST_DIR := "C:\Program Files\Cakewalk\VstPlugins"
#	VST_DIR := D:\vst
	BUNDLE := .\target\bundled\harmonicity-plugin.vst3
else ifeq ($(UNAME_S),Linux)
	$(error Linux isn't configured yet)
else ifeq ($(UNAME_S),Darwin)
	VST_DIR := ~/Library/Audio/Plug-Ins/VST
	BUNDLE := ./target/bundled/harmonicity-plugin.vst3
endif

# Determine the shell command for directory existence check
ifeq ($(OS),Windows_NT)
    # For native Windows, use 'if exist' command syntax
    # Note: Using 'call' to ensure 'echo' runs correctly after 'if exist' within shell function
    # VST_DIR is quoted in case of spaces in the path
    _CHECK_DIR_EXISTS_CMD = cmd /c "if exist $(VST_DIR) (echo yes) else (echo no)"
else
    # For Unix-like environments (Linux, macOS, Cygwin, MinGW/MSYS2)
    # Using 'test -d' to check if it's a directory
    _CHECK_DIR_EXISTS_CMD = test -d "$(VST_DIR)" && echo yes || echo no
endif

# Execute the shell command to check for directory existence and store the result
VST_DIR_EXISTS := $(shell $(_CHECK_DIR_EXISTS_CMD))
ifeq ($(VST_DIR_EXISTS),no)
	$(error Distribute folder not found)
endif

.PHONY: always distribute

$(BUNDLE): always
	cargo xtask bundle harmonicity-plugin --release

distribute: $(BUNDLE)
	cp -r $(BUNDLE) $(VST_DIR)
