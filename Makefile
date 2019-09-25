out: clean
	@echo "🎁  Building package..."
	wasm-pack build --target nodejs --release --out-name codex --scope nosaj

clean:
	@echo "🗑  Cleaning up...";
	rm -rf ./pkg;