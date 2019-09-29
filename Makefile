out: clean
	@echo "🎁  Building package..."
	wasm-pack build --target web --release --out-name codex --scope nosaj

clean:
	@echo "🗑  Cleaning up...";
	rm -rf ./pkg;

publish:
	@echo "⬆️  Publishing to NPM...";
	cd ./pkg && npm pack && npm publish;