VST_DIR := ~/Library/Audio/Plug-Ins/VST
BUNDLE := ./target/bundled/harmonicity-plugin.vst3

.PHONY: always distribute

$(BUNDLE): always
	cargo xtask bundle harmonicity-plugin --release

distribute: $(BUNDLE)
	cp -r $(BUNDLE) $(VST_DIR)
